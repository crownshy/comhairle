use super::error::{Result, TranscriptionServiceError};
use super::{Transcriber, TranscriptEvent, Transcription};
use async_trait::async_trait;
use aws_config;
use aws_sdk_transcribe;
use aws_sdk_transcribe::types::{LanguageCode, Media, MediaFormat, TranscriptionJobStatus};
use aws_sdk_transcribestreaming;
use aws_sdk_transcribestreaming::primitives::Blob;
use aws_sdk_transcribestreaming::types::{AudioEvent, AudioStream, MediaEncoding};
use aws_smithy_http::event_stream::EventStreamSender;
use bytes::Bytes;
use std::process::Stdio;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;
use tokio::sync::mpsc::{self, Receiver};
use tracing::{error, info, warn};

async fn write_stream(mut rx: Receiver<Bytes>, filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename).await?;

    while let Some(chunk) = rx.recv().await {
        file.write_all(&chunk).await?;
    }

    Ok(())
}

fn ffmpeg_convert_stream(mut audio_in: mpsc::Receiver<Bytes>) -> Result<mpsc::Receiver<Bytes>> {
    let (pcm_tx, pcm_rx) = mpsc::channel::<Bytes>(32);
    let mut ffmpeg = Command::new("ffmpeg")
        .args([
            "-loglevel",
            "debug", // Enable debug to see what's happening
            "-fflags",
            "+genpts+igndts", // Generate pts and ignore dts for streaming
            "-probesize",
            "10240", // Larger probe size for WebM header parsing
            "-analyzeduration",
            "10240", // Allow time to analyze WebM container
            "-f",
            "webm", // Explicitly specify input format
            "-i",
            "pipe:0",
            "-f",
            "s16le",
            "-acodec",
            "pcm_s16le",
            "-ac",
            "1",
            "-ar",
            "16000",
            "-flush_packets",
            "1", // Flush output immediately
            "pipe:1",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped()) // Capture stderr for debug
        .spawn()
        .inspect_err(|e| error!("FFmpeg spawn error: {e:#?}"))
        .map_err(|e| TranscriptionServiceError::StreamingTranscriptionFailure(e.to_string()))?;

    let mut ffmpeg_stdin = ffmpeg.stdin.take().unwrap();
    let mut ffmpeg_stdout = ffmpeg.stdout.take().unwrap();
    let ffmpeg_stderr = ffmpeg.stderr.take().unwrap();

    // Spawn FFmpeg with streaming-optimized parameters
    // Monitor FFmpeg stderr for debugging
    tokio::spawn(async move {
        use tokio::io::AsyncBufReadExt;
        let reader = tokio::io::BufReader::new(ffmpeg_stderr);
        let mut lines = reader.lines();
        while let Ok(Some(line)) = lines.next_line().await {
            info!("FFmpeg: {}", line);
        }
    });

    tokio::spawn(async move {
        let mut total_bytes = 0;
        let mut chunk_count = 0;
        let mut buffer = Vec::new();
        let mut header_sent = false;

        while let Some(webm) = audio_in.recv().await {
            chunk_count += 1;
            total_bytes += webm.len();
            info!(
                "Chunk {}: Received {} bytes (total: {})",
                chunk_count,
                webm.len(),
                total_bytes
            );

            // Basic WebM validation - check for EBML signature in first chunk
            if chunk_count == 1 && webm.len() >= 4 {
                let first_bytes = &webm[0..4];
                info!("First 4 bytes: {:02X?}", first_bytes);
                if first_bytes != &[0x1A, 0x45, 0xDF, 0xA3] {
                    warn!(
                        "WARNING: Does not start with EBML signature! Got: {:02X?}",
                        first_bytes
                    );
                } else {
                    info!("Valid EBML signature found");
                }
            }

            buffer.extend_from_slice(&webm);

            // For the first chunks, buffer until we have enough for WebM header
            if !header_sent && buffer.len() >= 1024 {
                info!("Sending buffered header data: {} bytes", buffer.len());
                if let Err(e) = ffmpeg_stdin.write_all(&buffer).await {
                    error!("Failed to write header to FFmpeg stdin: {e:#?}");
                    break;
                }
                if let Err(e) = ffmpeg_stdin.flush().await {
                    error!("Failed to flush FFmpeg header: {e:#?}");
                    break;
                }
                buffer.clear();
                header_sent = true;
            } else if header_sent {
                // After header, stream data directly
                if let Err(e) = ffmpeg_stdin.write_all(&webm).await {
                    error!("Failed to write to FFmpeg stdin: {e:#?}");
                    break;
                }
                if let Err(e) = ffmpeg_stdin.flush().await {
                    error!("Failed to flush FFmpeg stdin: {e:#?}");
                    break;
                }
            }
        }

        // TODO: this may be unneccessary and be already written above
        // Send any remaining buffer data
        if !buffer.is_empty() {
            info!("Sending final buffer: {} bytes", buffer.len());
            let _ = ffmpeg_stdin.write_all(&buffer).await;
            let _ = ffmpeg_stdin.flush().await;
        }

        // Signal FFmpeg input is done
        drop(ffmpeg_stdin);
        info!(
            "FFmpeg input stream closed after {} chunks ({} total bytes)",
            chunk_count, total_bytes
        );
    });

    // FFmpeg output → PCM (continuous reading with chunking)
    tokio::spawn(async move {
        let mut buf = vec![0u8; 4096];
        let mut accumulated_audio = Vec::new();
        let mut total_audio = Vec::new();
        let min_chunk_size = 3200; // 200ms at 16kHz mono 16-bit = 3200 bytes (reduce frequency)
        let mut last_send = std::time::Instant::now();
        // TODO: probably safe to remove?
        let min_send_interval = std::time::Duration::from_millis(0); // Minimum 100ms between sends

        loop {
            match ffmpeg_stdout.read(&mut buf).await {
                Ok(0) => {
                    info!("FFmpeg output stream ended");
                    // Send any remaining audio
                    if !accumulated_audio.is_empty() {
                        info!("Sending final {} bytes of audio", accumulated_audio.len());
                        let _ = pcm_tx.send(Bytes::from(accumulated_audio.clone())).await;
                    }
                    break; // EOF
                }
                Ok(n) => {
                    info!("Read {} bytes from FFmpeg", n);
                    accumulated_audio.extend_from_slice(&buf[..n]);
                    total_audio.extend_from_slice(&buf[..n]);

                    // Send chunks with rate limiting to avoid AWS throttling
                    while accumulated_audio.len() >= min_chunk_size {
                        // Rate limit: ensure minimum interval between sends
                        let elapsed = last_send.elapsed();
                        if elapsed < min_send_interval {
                            tokio::time::sleep(min_send_interval - elapsed).await;
                        }

                        let chunk: Vec<u8> = accumulated_audio.drain(..min_chunk_size).collect();
                        info!(
                            "Sending {} byte audio chunk to AWS ({}ms audio)",
                            chunk.len(),
                            chunk.len() / 32
                        );

                        if pcm_tx.send(Bytes::from(chunk)).await.is_err() {
                            info!("Failed to send audio chunk");
                            return;
                        }

                        last_send = std::time::Instant::now();
                    }
                }
                Err(e) => {
                    error!("FFmpeg read error: {e:#?}");
                    break;
                }
            }
        }
        // Close the PCM channel when FFmpeg output ends
        drop(pcm_tx);
    });
    Ok(pcm_rx)
}

pub async fn write_converted_stream_to_file(audio_stream: Receiver<Bytes>) -> Result<()> {
    let pcm_rx = ffmpeg_convert_stream(audio_stream)?;
    write_stream(pcm_rx, "testaudo.pca")
        .await
        .map_err(|e| TranscriptionServiceError::StreamingTranscriptionFailure(e.to_string()))?;
    Ok(())
}

/// Takes the raw transcript we get back from amazon and
/// condenses individual word utterances into longer
/// strings split by author or pauses
fn consolidated_transcription(raw_transcription: &Transcription) -> Transcription {
    let pause_split: f64 = 5000.0;

    let mut new_events = vec![];

    let mut speaker_id: Option<String> = None;
    let mut segment_start_time = None;
    let mut segment_end_time = None;
    let mut running_segment = String::from("");

    // Find where pending events start (they should be trailing)
    let first_pending_idx = raw_transcription.events.iter().position(|e| e.is_pending);

    // Process only non-pending events normally
    let events_to_process = if let Some(idx) = first_pending_idx {
        &raw_transcription.events[..idx]
    } else {
        &raw_transcription.events[..]
    };

    for event in events_to_process {
        if segment_start_time.is_none() {
            segment_start_time = Some(event.start_time)
        }

        if segment_end_time.is_none() {
            segment_end_time = Some(event.end_time)
        }

        // We need different behavior when we have speaker ids
        // vs not
        if raw_transcription.has_speaker_ids {
            // If the speaker id is none then it's usually
            // punctuation, we just append this onto the
            // existing segment. Don't update segment_end_time since
            // punctuation is instantaneous and shouldn't affect pause detection
            if event.speaker_id.is_none() {
                running_segment = format!("{running_segment}{}", event.text);
            }
            // If the speaker hasn't changed and the pause between the last
            // utterance and this one is short enough then we want to just append
            // the content to the running transcript
            else if speaker_id == event.speaker_id
                && event.start_time - segment_end_time.unwrap() < pause_split
            {
                segment_end_time = Some(event.end_time);
                if running_segment.is_empty() {
                    running_segment = event.text.clone();
                } else {
                    running_segment = format!("{running_segment} {}", event.text);
                }
            }
            // Otherwise this segment has ended. We want to append it to
            // the new event log and reset our running totals
            else {
                // Only push if we have accumulated text (not the first event)
                if !running_segment.is_empty() {
                    new_events.push(TranscriptEvent {
                        text: running_segment.clone(),
                        start_time: segment_start_time.unwrap(),
                        end_time: segment_end_time.unwrap(),
                        speaker_id: speaker_id.clone(),
                        is_pending: false,
                    });
                }
                segment_start_time = Some(event.start_time);
                segment_end_time = Some(event.end_time);
                running_segment = event.text.clone();
                speaker_id = event.speaker_id.clone();
            }
        }
        // If there are no speaker ids then this is easier
        // we simply look for gaps
        else {
            if event.start_time - segment_end_time.unwrap() > pause_split {
                // Only push if we have accumulated text (not the first event)
                if !running_segment.is_empty() {
                    new_events.push(TranscriptEvent {
                        text: running_segment.clone(),
                        start_time: segment_start_time.unwrap(),
                        end_time: segment_end_time.unwrap(),
                        speaker_id: speaker_id.clone(),
                        is_pending: false,
                    });
                }
                segment_start_time = Some(event.start_time);
                segment_end_time = Some(event.end_time);
                running_segment = event.text.clone();
            } else {
                if running_segment.is_empty() {
                    running_segment = event.text.clone();
                } else {
                    running_segment = format!("{running_segment} {}", event.text);
                }
                segment_end_time = Some(event.end_time);
            }
        }
    }

    // Push the final accumulated segment if it's not empty
    if !running_segment.is_empty() {
        new_events.push(TranscriptEvent {
            text: running_segment,
            start_time: segment_start_time.unwrap(),
            end_time: segment_end_time.unwrap(),
            speaker_id: speaker_id,
            is_pending: false,
        });
    }

    // Now consolidate all trailing pending events into a single event
    if let Some(first_pending_idx) = first_pending_idx {
        let pending_events = &raw_transcription.events[first_pending_idx..];

        if !pending_events.is_empty() {
            let mut pending_text = String::new();
            let pending_start = pending_events[0].start_time;
            let pending_end = pending_events[pending_events.len() - 1].end_time;
            let mut pending_speaker = None;

            for event in pending_events {
                if pending_text.is_empty() {
                    pending_text = event.text.clone();
                } else {
                    // Add space between words unless the previous text ends with punctuation
                    if event.speaker_id.is_none() {
                        // Punctuation - no space
                        pending_text = format!("{}{}", pending_text, event.text);
                    } else {
                        // Regular word - add space
                        pending_text = format!("{} {}", pending_text, event.text);
                    }
                }

                // Take the first non-none speaker_id we find
                if pending_speaker.is_none() && event.speaker_id.is_some() {
                    pending_speaker = event.speaker_id.clone();
                }
            }

            new_events.push(TranscriptEvent {
                text: pending_text,
                start_time: pending_start,
                end_time: pending_end,
                speaker_id: pending_speaker,
                is_pending: true,
            });
        }
    }

    Transcription {
        start_time: raw_transcription.start_time.clone(),
        events: new_events,
        has_speaker_ids: raw_transcription.has_speaker_ids,
    }
}

#[async_trait]
impl Transcriber for AmazonTranscriber {
    async fn transcribe(&self, _audio: &Vec<u8>) -> Result<String> {
        Err(TranscriptionServiceError::BatchProcessingUnsupported)
    }

    async fn transcribe_from_bulk_store(&self, store_name: &str, location: &str) -> Result<()> {
        let uri = format!("s3://{store_name}/{location}/main_room_recording.wav"); // TODO: change
                                                                                   // to recording.wav
        let audio_file = Media::builder().media_file_uri(uri).build();
        let job_name = "audio_test_3"; // TODO: use uuid?

        self.transcribe_client
            .start_transcription_job()
            .transcription_job_name(job_name)
            .output_bucket_name(store_name)
            .output_key(format!("{location}/transcript.json"))
            .media(audio_file)
            .media_format(MediaFormat::Wav)
            .language_code(LanguageCode::EnUs)
            .send()
            .await
            // .inspect_err(|e| error!("Failed to start transcription: {e:#?}"))
            .inspect_err(|e| {
                error!("Failed to start transcription: {e:#?}");

                // Extract AWS service error metadata
                use aws_sdk_transcribe::error::ProvideErrorMetadata;
                error!("  code:    {:?}", e.code());
                error!("  message: {:?}", e.message());

                // If it's specifically a service error, get HTTP status
                if let aws_sdk_transcribe::error::SdkError::ServiceError(se) = e {
                    error!("  HTTP status: {}", se.raw().status());
                    error!("  raw body: {:?}", se.raw().body());
                }
            })
            // .map_err(|e| TranscriptionServiceError::TranscriptionFailure(e.to_string()))?;
            .map_err(|e| {
                use aws_sdk_transcribe::error::ProvideErrorMetadata;
                let detail = format!("code={:?} message={:?} debug={e:#?}", e.code(), e.message());
                TranscriptionServiceError::TranscriptionFailure(detail)
            })?;

        let mut snooze: u64 = 100;
        let mut snooze_total = snooze;

        let mut found = false;

        println!("Waiting for transcription job to finish");

        while !found {
            let response = self
                .transcribe_client
                .get_transcription_job()
                .transcription_job_name(job_name)
                .send()
                .await
                .inspect_err(|e| error!("Failed to get transcription job: {e:#?}"))
                .map_err(|e| TranscriptionServiceError::TranscriptionFailure(e.to_string()))?;

            let job = response.transcription_job.unwrap(); // TODO:
            let status = job.transcription_job_status.unwrap(); // TODO:

            if status == TranscriptionJobStatus::Completed
                || status == TranscriptionJobStatus::Failed
            {
                println!("Waited {} milliseconds for job to finish", snooze_total);

                if status == TranscriptionJobStatus::Completed {
                    println!("Transcription: ");

                    let uri = job.transcript.unwrap().transcript_file_uri.unwrap(); // TODO:
                    println!();
                    println!("    >>>>    Uri for transcription: {uri:#?}");
                    println!();
                }

                found = true
            } else {
                snooze *= 2;
                snooze_total += snooze;

                tokio::time::sleep(tokio::time::Duration::from_millis(snooze)).await;
            }
        }

        Ok(())
    }

    fn model_detects_speakers(&self) -> bool {
        true
    }

    async fn transcribe_live(
        &self,
        input_stream: Receiver<Bytes>,
    ) -> Result<Receiver<Transcription>> {
        // Output to caller (text)
        let (transcript_tx, transcript_rx) = mpsc::channel::<Transcription>(32);

        // PCM channel → AWS

        // WebM → FFmpeg input (async with buffering)
        let pcm_rx = ffmpeg_convert_stream(input_stream)?;
        // write_stream(pcm_rx, "testaudo.pca").await?;

        // PCM → Amazon Transcribe
        let audio_stream = EventStreamSender::from(async_stream::stream! {
            let mut pcm_rx = pcm_rx;
            while let Some(chunk) = pcm_rx.recv().await {
                yield Ok(AudioStream::AudioEvent(AudioEvent::builder()
                    .audio_chunk(Blob::new(chunk))
                    .build()));
            }
        });

        info!("Starting Amazon Transcribe stream with PCM 16kHz mono");
        let mut transcribe_stream = self
            .streaming_client
            .start_stream_transcription()
            .show_speaker_label(true)
            .language_code("en-US".into())
            .media_encoding(MediaEncoding::Pcm)
            .media_sample_rate_hertz(16000)
            .audio_stream(audio_stream)
            .send()
            .await
            .inspect_err(|e| error!("Failed to start transcription stream: {e:#?}"))
            .map_err(|e| TranscriptionServiceError::StreamingTranscriptionFailure(e.to_string()))?;

        info!("Amazon Transcribe stream started successfully");

        // Transcribe → Output channel
        tokio::spawn(async move {
            info!("Started listening for Amazon Transcribe results...");
            let mut full_transcription: Transcription = Transcription::new();
            full_transcription.has_speaker_ids = true;

            loop {
                match transcribe_stream.transcript_result_stream.recv().await {
                    Ok(event) => {
                        if let Some(event) = event {
                            if let Some(transcript) =
                                // TODO: remove unwrap
                                event.as_transcript_event().unwrap().transcript()
                            {
                                if let Some(results) = &transcript.results {
                                    info!("Transcription result event {results:#?}");
                                    for result in results.iter() {
                                        if let Some(alt) = result.alternatives().first() {
                                            if let Some(items) = &alt.items {
                                                let mut all_items = vec![];
                                                for item in items {
                                                    if let Some(content) = &item.content {
                                                        all_items.push(TranscriptEvent {
                                                            text: content.to_owned(),
                                                            start_time: item.start_time,
                                                            end_time: item.end_time,
                                                            speaker_id: item.speaker.to_owned(),
                                                            is_pending: result.is_partial,
                                                        })
                                                    }
                                                }
                                                full_transcription.events.retain(|x| !x.is_pending);

                                                full_transcription
                                                    .events
                                                    .extend_from_slice(all_items.as_slice());

                                                let result =
                                                    consolidated_transcription(&full_transcription);

                                                info!("\n\n------------");
                                                info!("{result:#?}");
                                                info!("------------\n\n");

                                                let _ = transcript_tx
                                                    //TODO figure out if we can remove this clone
                                                    .send(result)
                                                    .await;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("Transcription stream error: {e:#?}");

                        // Handle specific AWS rate limiting errors
                        let error_str = format!("{:?}", e);
                        if error_str.contains("ENHANCE_YOUR_CALM")
                            || error_str.contains("RST frames")
                        {
                            warn!("AWS rate limit detected - waiting before retry...");
                            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                        } else {
                            break;
                        }
                    }
                }
            }
        });

        Ok(transcript_rx)
    }

    fn supports_streaming(&self) -> bool {
        true
    }
}

pub struct AmazonTranscriber {
    transcribe_client: aws_sdk_transcribe::Client,
    streaming_client: aws_sdk_transcribestreaming::Client,
}

impl AmazonTranscriber {
    pub async fn new() -> Self {
        let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;

        let transcribe_client = aws_sdk_transcribe::Client::new(&config);
        let streaming_client = aws_sdk_transcribestreaming::Client::new(&config);
        Self {
            transcribe_client,
            streaming_client,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error::Error;

    fn create_event(
        text: &str,
        start_time: f64,
        end_time: f64,
        speaker_id: Option<&str>,
        is_pending: bool,
    ) -> TranscriptEvent {
        TranscriptEvent {
            text: text.to_string(),
            start_time,
            end_time,
            speaker_id: speaker_id.map(|s| s.to_string()),
            is_pending,
        }
    }

    #[test]
    fn test_consolidate_transcription_with_speaker_changes() {
        // Test data based on example_transcription.ts
        // Speaker 0 says: "So we just talk naturally and we'll see how well this does."
        // Then speaker 1 says: "All right. Sounds good."
        let mut raw = Transcription::new();
        raw.has_speaker_ids = true;

        // Speaker 0's utterance (0.776s - 3.076s)
        raw.events
            .push(create_event("So", 0.776, 0.926, Some("0"), false));
        raw.events
            .push(create_event("we", 0.926, 0.986, Some("0"), false));
        raw.events
            .push(create_event("just", 0.986, 1.126, Some("0"), false));
        raw.events
            .push(create_event("talk", 1.126, 1.356, Some("0"), false));
        raw.events
            .push(create_event("naturally", 1.356, 1.726, Some("0"), false));
        raw.events
            .push(create_event("and", 1.726, 2.036, Some("0"), false));
        raw.events
            .push(create_event("we'll", 2.036, 2.146, Some("0"), false));
        raw.events
            .push(create_event("see", 2.146, 2.246, Some("0"), false));
        raw.events
            .push(create_event("how", 2.246, 2.446, Some("0"), false));
        raw.events
            .push(create_event("well", 2.446, 2.456, Some("0"), false));
        raw.events
            .push(create_event("this", 2.456, 2.686, Some("0"), false));
        raw.events
            .push(create_event("does", 2.686, 3.076, Some("0"), false));
        raw.events
            .push(create_event(".", 3.076, 3.076, None, false));

        // Speaker 1's utterance (3.186s - 3.606s)
        raw.events
            .push(create_event("All", 3.186, 3.336, Some("1"), false));
        raw.events
            .push(create_event("right", 3.336, 3.606, Some("1"), false));
        raw.events
            .push(create_event(".", 3.606, 3.606, None, false));

        // Speaker 1 continues (3.886s - 4.606s)
        raw.events
            .push(create_event("Sounds", 3.886, 4.206, Some("1"), false));
        raw.events
            .push(create_event("good", 4.206, 4.606, Some("1"), false));
        raw.events
            .push(create_event(".", 4.606, 4.606, None, false));

        let consolidated = consolidated_transcription(&raw);

        // Should have 2 consolidated segments (speaker changes create breaks)
        // Note: "All right." and "Sounds good." are merged because same speaker
        // with only 280ms pause (< 5000ms threshold)
        assert_eq!(consolidated.events.len(), 2);
        assert_eq!(consolidated.has_speaker_ids, true);

        // First segment: Speaker 0
        assert_eq!(consolidated.events[0].speaker_id, Some("0".to_string()));
        assert_eq!(
            consolidated.events[0].text,
            "So we just talk naturally and we'll see how well this does."
        );
        assert_eq!(consolidated.events[0].start_time, 0.776);
        assert_eq!(consolidated.events[0].end_time, 3.076);
        assert_eq!(consolidated.events[0].is_pending, false);

        // Second segment: Speaker 1 says both utterances merged due to short pause
        assert_eq!(consolidated.events[1].speaker_id, Some("1".to_string()));
        assert_eq!(consolidated.events[1].text, "All right. Sounds good.");
        assert_eq!(consolidated.events[1].start_time, 3.186);
        assert_eq!(consolidated.events[1].end_time, 4.606);
    }

    #[test]
    fn test_consolidate_transcription_with_long_pause() {
        // Test pause detection (pause_split = 5000ms)
        let mut raw = Transcription::new();
        raw.has_speaker_ids = true;

        // Speaker 0: "I disagree."
        raw.events
            .push(create_event("I", 11.476, 11.646, Some("1"), false));
        raw.events
            .push(create_event("disagree", 11.646, 12.406, Some("1"), false));
        raw.events
            .push(create_event(".", 12.406, 12.406, None, false));

        // Long pause of 3.45 seconds (within threshold)
        // Speaker 0: "OK, that seems..."
        raw.events
            .push(create_event("OK", 15.576, 15.856, Some("0"), false));
        raw.events
            .push(create_event(",", 15.856, 15.856, None, false));
        raw.events
            .push(create_event("that", 16.046, 16.286, Some("0"), false));

        let consolidated = consolidated_transcription(&raw);

        // Should have 2 segments due to speaker change (even though pause < 5s)
        assert_eq!(consolidated.events.len(), 2);

        assert_eq!(consolidated.events[0].speaker_id, Some("1".to_string()));
        assert_eq!(consolidated.events[0].text, "I disagree.");

        assert_eq!(consolidated.events[1].speaker_id, Some("0".to_string()));
        assert_eq!(consolidated.events[1].text, "OK, that");
    }

    #[test]
    fn test_consolidate_transcription_with_pending_events() {
        // Test that trailing pending events are consolidated into a single event
        let mut raw = Transcription::new();
        raw.has_speaker_ids = true;

        raw.events
            .push(create_event("Hello", 1.0, 1.5, Some("0"), false));
        raw.events
            .push(create_event("world", 1.5, 2.0, Some("0"), false));
        raw.events
            .push(create_event("pending", 2.0, 2.5, Some("0"), true));
        raw.events
            .push(create_event("word", 2.5, 3.0, Some("0"), true));

        let consolidated = consolidated_transcription(&raw);

        // Should have 1 consolidated segment + 1 consolidated pending event
        assert_eq!(consolidated.events.len(), 2);

        // First is consolidated non-pending
        assert_eq!(consolidated.events[0].text, "Hello world");
        assert_eq!(consolidated.events[0].is_pending, false);

        // Trailing pending events are consolidated into one
        assert_eq!(consolidated.events[1].text, "pending word");
        assert_eq!(consolidated.events[1].is_pending, true);
        assert_eq!(consolidated.events[1].start_time, 2.0);
        assert_eq!(consolidated.events[1].end_time, 3.0);
    }

    #[test]
    fn test_consolidate_pending_events_with_punctuation() {
        // Test that pending events with punctuation are properly consolidated
        let mut raw = Transcription::new();
        raw.has_speaker_ids = true;

        raw.events
            .push(create_event("Hello", 1.0, 1.5, Some("0"), false));
        raw.events
            .push(create_event("How", 2.0, 2.5, Some("0"), true));
        raw.events
            .push(create_event("are", 2.5, 2.8, Some("0"), true));
        raw.events
            .push(create_event("you", 2.8, 3.0, Some("0"), true));
        raw.events.push(create_event("?", 3.0, 3.0, None, true));

        let consolidated = consolidated_transcription(&raw);

        // Should have 1 non-pending segment + 1 consolidated pending segment
        assert_eq!(consolidated.events.len(), 2);

        assert_eq!(consolidated.events[0].text, "Hello");
        assert_eq!(consolidated.events[0].is_pending, false);

        // Pending events consolidated with proper punctuation (no space before ?)
        assert_eq!(consolidated.events[1].text, "How are you?");
        assert_eq!(consolidated.events[1].is_pending, true);
    }

    #[test]
    fn test_consolidate_transcription_without_speaker_ids() {
        // Test consolidation based on pauses only (no speaker IDs)
        let mut raw = Transcription::new();
        raw.has_speaker_ids = false;

        // Short sequence
        raw.events
            .push(create_event("Hello", 1.0, 1.5, None, false));
        raw.events
            .push(create_event("world", 1.5, 2.0, None, false));
        raw.events
            .push(create_event("there", 2.0, 2.5, None, false));

        // Long pause (> 5000ms) - should split here
        raw.events
            .push(create_event("after", 7600.0, 7800.0, None, false));
        raw.events
            .push(create_event("pause", 7800.0, 8000.0, None, false));

        let consolidated = consolidated_transcription(&raw);

        // Should have 2 segments due to long pause
        assert_eq!(consolidated.events.len(), 2);

        assert_eq!(consolidated.events[0].text, "Hello world there");
        assert_eq!(consolidated.events[0].start_time, 1.0);
        assert_eq!(consolidated.events[0].end_time, 2.5);

        assert_eq!(consolidated.events[1].text, "after pause");
        assert_eq!(consolidated.events[1].start_time, 7600.0);
        assert_eq!(consolidated.events[1].end_time, 8000.0);
    }

    #[test]
    fn test_consolidate_empty_transcription() {
        // Test with empty events
        let raw = Transcription::new();
        let consolidated = consolidated_transcription(&raw);

        assert_eq!(consolidated.events.len(), 0);
    }

    #[test]
    fn test_consolidate_single_event() {
        // Test with a single event
        let mut raw = Transcription::new();
        raw.has_speaker_ids = true;
        raw.events
            .push(create_event("Hello", 1.0, 1.5, Some("0"), false));

        let consolidated = consolidated_transcription(&raw);

        assert_eq!(consolidated.events.len(), 1);
        assert_eq!(consolidated.events[0].text, "Hello");
    }

    #[test]
    fn test_consolidate_long_conversation_with_interruptions() {
        // Test based on example_transcription.ts data with speaker interruptions
        let mut raw = Transcription::new();
        raw.has_speaker_ids = true;

        // Speaker 0: "So this would be in the context of a deliberation where people are sitting around the room and"
        raw.events
            .push(create_event("So", 5.086, 5.166, Some("0"), false));
        raw.events
            .push(create_event("this", 5.166, 5.326, Some("0"), false));
        raw.events
            .push(create_event("would", 5.326, 5.446, Some("0"), false));
        raw.events
            .push(create_event("be", 5.446, 5.526, Some("0"), false));
        raw.events
            .push(create_event("in", 5.526, 5.686, Some("0"), false));
        raw.events
            .push(create_event("the", 5.686, 5.846, Some("0"), false));
        raw.events
            .push(create_event("context", 5.846, 6.446, Some("0"), false));
        raw.events
            .push(create_event("of", 6.446, 6.926, Some("0"), false));
        raw.events
            .push(create_event("a", 6.926, 7.456, Some("0"), false));
        raw.events
            .push(create_event("deliberation", 7.646, 8.316, Some("0"), false));
        raw.events
            .push(create_event("where", 8.316, 8.446, Some("0"), false));
        raw.events
            .push(create_event("people", 8.446, 8.606, Some("0"), false));
        raw.events
            .push(create_event("are", 8.606, 8.686, Some("0"), false));
        raw.events
            .push(create_event("sitting", 8.686, 8.926, Some("0"), false));
        raw.events
            .push(create_event("around", 8.926, 9.146, Some("0"), false));
        raw.events
            .push(create_event("the", 9.146, 9.156, Some("0"), false));
        raw.events
            .push(create_event("room", 9.156, 9.606, Some("0"), false));
        raw.events
            .push(create_event("and", 9.606, 9.876, Some("0"), false));

        // Speaker 1 interrupts: "I think you're wrong."
        raw.events
            .push(create_event("I", 9.876, 10.246, Some("1"), false));
        raw.events
            .push(create_event("think", 10.246, 10.366, Some("1"), false));
        raw.events
            .push(create_event("you're", 10.366, 10.646, Some("1"), false));
        raw.events
            .push(create_event("wrong", 10.646, 11.276, Some("1"), false));
        raw.events
            .push(create_event(".", 11.276, 11.276, None, false));

        let consolidated = consolidated_transcription(&raw);

        // Should have 2 segments due to speaker interruption
        assert_eq!(consolidated.events.len(), 2);

        // Speaker 0's segment (interrupted mid-sentence)
        assert_eq!(consolidated.events[0].speaker_id, Some("0".to_string()));
        assert!(consolidated.events[0].text.starts_with("So this would be"));
        assert!(consolidated.events[0].text.ends_with("and"));
        assert_eq!(consolidated.events[0].start_time, 5.086);

        // Speaker 1 interrupts
        assert_eq!(consolidated.events[1].speaker_id, Some("1".to_string()));
        assert_eq!(consolidated.events[1].text, "I think you're wrong.");
        assert_eq!(consolidated.events[1].start_time, 9.876);
    }

    #[tokio::test]
    #[ignore]
    async fn test_transcription_of_bulk_storage_audio_file(
    ) -> std::result::Result<(), Box<dyn Error>> {
        let transcriber = AmazonTranscriber::new().await;
        let result = transcriber
            .transcribe_from_bulk_store(
                "comhairle-media",
                "events/3c22d53d-07df-4d46-802e-486b79dd1a80",
            )
            .await?;

        Ok(())
    }
}
