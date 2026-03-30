use anyhow::{anyhow, Context, Result};
use gstreamer as gst;
use gstreamer::prelude::*;
use lib_gst_meet::{xmpp_parsers::BareJid, Authentication, Connection, JitsiConference, JitsiConferenceConfig};
use tracing::{info, error};

/// Creates a GStreamer recording sink that saves audio to a file
fn create_recording_sink(filename: &str) -> Result<gst::Element> {
    info!("Creating recording sink for file: {}", filename);

    // Create a bin to hold our recording pipeline
    let bin = gst::Bin::new(Some("recording-bin"));

    // Create elements
    let queue = gst::ElementFactory::make("queue")
        .name("recording-queue")
        .build()
        .context("Failed to create queue")?;

    let audioconvert = gst::ElementFactory::make("audioconvert")
        .name("recording-audioconvert")
        .build()
        .context("Failed to create audioconvert")?;

    let audioresample = gst::ElementFactory::make("audioresample")
        .name("recording-audioresample")
        .build()
        .context("Failed to create audioresample")?;

    let encoder = gst::ElementFactory::make("wavenc")
        .name("recording-wavenc")
        .build()
        .context("Failed to create wavenc")?;

    let filesink = gst::ElementFactory::make("filesink")
        .name("recording-filesink")
        .property("location", filename)
        .build()
        .context("Failed to create filesink")?;

    // Add elements to bin
    bin.add_many(&[&queue, &audioconvert, &audioresample, &encoder, &filesink])
        .context("Failed to add elements to bin")?;

    // Link elements
    gst::Element::link_many(&[&queue, &audioconvert, &audioresample, &encoder, &filesink])
        .context("Failed to link recording elements")?;

    // Create ghost pad for the bin's entry point
    let sink_pad = queue.static_pad("sink")
        .ok_or_else(|| anyhow!("Failed to get queue sink pad"))?;

    let ghost_pad = gst::GhostPad::with_target(Some("sink"), &sink_pad)
        .context("Failed to create ghost pad")?;

    // Add a probe to monitor data flow
    let buffer_count = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let buffer_count_clone = buffer_count.clone();
    ghost_pad.add_probe(gst::PadProbeType::BUFFER, move |_pad, probe_info| {
        if let Some(gst::PadProbeData::Buffer(_buffer)) = &probe_info.data {
            let count = buffer_count_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            if count % 100 == 0 {
                info!("Recording pipeline: received {} audio buffers", count);
            }
        }
        gst::PadProbeReturn::Ok
    });

    ghost_pad.set_active(true)
        .context("Failed to activate ghost pad")?;

    bin.add_pad(&ghost_pad)
        .context("Failed to add ghost pad")?;

    // Add a bus watch to monitor for errors
    let bus = bin.bus().ok_or_else(|| anyhow!("Failed to get bus from recording bin"))?;
    bus.add_watch(move |_bus, msg| {
        use gst::MessageView;
        match msg.view() {
            MessageView::Error(err) => {
                error!(
                    "Recording pipeline error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
            }
            MessageView::Warning(warning) => {
                error!(
                    "Recording pipeline warning from {:?}: {} ({:?})",
                    warning.src().map(|s| s.path_string()),
                    warning.error(),
                    warning.debug()
                );
            }
            MessageView::Eos(..) => {
                info!("Recording pipeline received EOS");
            }
            MessageView::StateChanged(state_changed) => {
                if state_changed.src().map(|s| s.type_().name() == "GstBin").unwrap_or(false) {
                    info!(
                        "Recording bin state changed from {:?} to {:?}",
                        state_changed.old(),
                        state_changed.current()
                    );
                }
            }
            _ => {}
        }
        Continue(true)
    })
    .context("Failed to add bus watch")?;

    // Set the bin to PLAYING state so data can flow through it
    bin.set_state(gst::State::Playing)
        .context("Failed to set recording bin to PLAYING state")?;

    info!("Recording bin set to PLAYING state");

    // Return the bin as an Element
    Ok(bin.upcast())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Initialize GStreamer
    gst::init()?;
    info!("GStreamer initialized");

    // Get room name from command line
    let args: Vec<String> = std::env::args().collect();
    let room_name = if args.len() > 1 {
        &args[1]
    } else {
        "test_room"
    };

    // Configuration
    let websocket_url = "wss://video.comhairle.scot/xmpp-websocket";
    let xmpp_domain = "meet.jitsi";
    let nickname = "recorder-bot";

    info!("Connecting to {} as {}", websocket_url, nickname);

    // Create XMPP connection
    let (connection, connection_future) = Connection::new(
        websocket_url,
        xmpp_domain,
        Authentication::Anonymous,
        room_name,
        false,
    )
    .await
    .context("Failed to create XMPP connection")?;

    // Spawn connection handler
    tokio::spawn(async move {
        connection_future.await;
    });

    // Connect to XMPP server
    connection.connect().await
        .context("Failed to connect to XMPP server")?;
    info!("Connected to XMPP server");

    // Create conference configuration
    let muc_jid: BareJid = format!("{}@muc.{}", room_name, xmpp_domain)
        .parse()
        .context("Failed to parse MUC JID")?;

    let focus_jid = format!("focus.{}", xmpp_domain)
        .parse()
        .context("Failed to parse focus JID")?;

    let conference_config = JitsiConferenceConfig {
        muc: muc_jid.clone(),
        focus: focus_jid,
        nick: nickname.to_string(),
        region: None,
        video_codec: "vp9".to_string(),
        extra_muc_features: vec![],
        start_bitrate: 800,
        stereo: true,
        recv_video_scale_width: 1280,
        recv_video_scale_height: 720,
        buffer_size: 200,
    };

    // Get glib main context
    let glib_main_context = glib::MainContext::default();

    // Join the conference
    info!("Joining conference room: {}", room_name);
    eprintln!("DEBUG: About to call JitsiConference::join()");
    let conference = JitsiConference::join(
        connection,
        glib_main_context.clone(),
        conference_config,
    )
    .await
    .context("Failed to join conference")?;

    eprintln!("DEBUG: JitsiConference::join() completed successfully!");
    info!("Successfully joined conference: {}", muc_jid);

    // Create recording sink
    eprintln!("DEBUG: Creating recording sink...");
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let filename = format!("recording_{}_{}.wav", room_name, timestamp);
    eprintln!("DEBUG: Filename will be: {}", filename);

    match create_recording_sink(&filename) {
        Ok(sink) => {
            eprintln!("DEBUG: Recording sink created successfully");
            info!("Created recording sink, setting as remote participant audio sink");

            // Set our recording sink as the destination for remote participant audio
            eprintln!("DEBUG: About to call set_remote_participant_audio_sink_element");
            conference.set_remote_participant_audio_sink_element(Some(sink))
                .await;
            eprintln!("DEBUG: set_remote_participant_audio_sink_element completed");

            info!("Recording to: {}", filename);
            info!("Bot is now recording mixed audio from all participants in the room");
        }
        Err(e) => {
            eprintln!("DEBUG: Failed to create recording sink: {:?}", e);
            error!("Failed to create recording sink: {:?}", e);
            return Err(e);
        }
    }

    eprintln!("DEBUG: Recording setup complete, entering main loop");

    // Keep running
    info!("Press Ctrl+C to stop recording and exit");
    tokio::signal::ctrl_c().await?;

    info!("Stopping recording...");

    // Clear the audio sink to flush and close the file properly
    conference.set_remote_participant_audio_sink_element(None).await;

    // Give it a moment to flush
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    info!("Recording saved to: {}", filename);
    info!("Shutting down...");

    Ok(())
}
