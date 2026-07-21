<script lang="ts">
	import { Mic, Square, Loader2 } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { notifications } from '$lib/notifications.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { onDestroy } from 'svelte';
	import type { AudioRecordingDto } from '@crownshy/api-client/api';

	// ── Props ─────────────────────────────────────────────────────────────────

	/**
	 * Props for LiveRecorder.
	 *
	 * @param conversation_id - The parent conversation id.
	 * @param event_id - The event being recorded under.
	 * @param recordings - Existing recordings, used to prevent duplicate names.
	 * @param onComplete - Called after the multipart upload is completed and processing is enqueued.
	 */
	type Props = {
		conversation_id: string;
		event_id: string;
		recordings: AudioRecordingDto[];
		onComplete?: () => void | Promise<void>;
	};

	let { conversation_id, event_id, recordings, onComplete }: Props = $props();

	// ── API response types ────────────────────────────────────────────────────
	// These mirror the server DTOs and will be replaced by generated client types
	// once the API spec is published.

	type UploadedPart = {
		partNumber: number;
		etag: string;
		sizeBytes: number;
	};

	type LiveAudioRecordingDto = {
		id: string;
		audioRecordingId: string;
		multipartUploadId: string;
		nextPartNumber: number;
		uploadedParts: UploadedPart[];
	};

	type CreateLiveAudioRecordingResponse = {
		recording: AudioRecordingDto;
		liveAudioRecording: LiveAudioRecordingDto;
	};

	type PresignLiveAudioRecordingPartResponse = {
		uploadUrl: string;
		partNumber: number;
	};

	type AckLiveAudioRecordingPartResponse = {
		liveAudioRecording: LiveAudioRecordingDto;
	};

	type ProcessRecordingResponse = {
		message: string;
		jobId: string;
	};

	// ── Constants ─────────────────────────────────────────────────────────────

	/** How often (ms) the MediaRecorder emits a `dataavailable` event. */
	const CHUNK_INTERVAL_MS = 10_000;

	/**
	 * Minimum accumulated bytes before a multipart part is flushed to S3.
	 * S3 requires all parts except the final one to be at least 5 MB.
	 */
	const MIN_PART_BYTES = 5 * 1024 * 1024;

	// ── Reactive state ────────────────────────────────────────────────────────

	let recordingName = $state('');
	let phase = $state<'idle' | 'starting' | 'recording' | 'stopping'>('idle');
	let activeLiveRecordingId = $state<string | null>(null);

	const isIdle = $derived(phase === 'idle');
	const isStarting = $derived(phase === 'starting');
	const isRecording = $derived(phase === 'recording');
	const isStopping = $derived(phase === 'stopping');

	// ── MediaRecorder internals (not reactive — mutated directly) ─────────────

	let mediaRecorder: MediaRecorder | null = null;
	let mediaStream: MediaStream | null = null;

	/** Sequentially-executed promise chain ensuring chunk uploads do not race. */
	let uploadChain: Promise<void> = Promise.resolve();

	/** Next expected part number, kept in sync with server ack responses. */
	let nextPartNumber = 1;

	/** Accumulated blobs waiting to be flushed as a single multipart part. */
	let pendingBlobs: Blob[] = [];
	let pendingBytes = 0;

	// ── API helper ────────────────────────────────────────────────────────────

	function liveApiBasePath(): string {
		return `/api/conversation/${conversation_id}/events/${event_id}/audio_recordings/live`;
	}

	async function callLiveApi<T>(method: string, path: string, body?: unknown): Promise<T> {
		const response = await fetch(`${liveApiBasePath()}${path}`, {
			method,
			credentials: 'include',
			headers: body ? { 'content-type': 'application/json' } : undefined,
			body: body ? JSON.stringify(body) : undefined
		});

		if (!response.ok) {
			let message = `${method} ${path} failed (${response.status})`;
			const parseResult = await tryCatchAsync(() => response.json());
			if (parseResult.err === null && typeof parseResult.ok?.message === 'string') {
				message = parseResult.ok.message;
			}
			throw new Error(message);
		}

		return response.json() as Promise<T>;
	}

	// ── Chunk upload logic ────────────────────────────────────────────────────

	async function putPartToSignedUrl(blob: Blob, url: string): Promise<string> {
		const response = await fetch(url, { method: 'PUT', body: blob });
		if (!response.ok) {
			throw new Error(`Part upload failed (${response.status})`);
		}
		const etag = response.headers.get('etag') ?? response.headers.get('ETag') ?? '';
		return etag.replaceAll('"', '');
	}

	/**
	 * Appends a blob to the pending buffer and flushes it as a multipart part when
	 * the buffer reaches the minimum part size, or when `forceFlush` is true.
	 *
	 * Enqueued operations run serially via `uploadChain` so parts are always
	 * acknowledged in order.
	 *
	 * @param liveRecordingId - The id of the active live_audio_recording row.
	 * @param blob - The audio blob to append, or null to trigger a flush with no new data.
	 * @param forceFlush - When true the buffer is flushed regardless of size (used at stop time).
	 */
	function enqueueChunk(liveRecordingId: string, blob: Blob | null, forceFlush: boolean): void {
		uploadChain = uploadChain.then(async () => {
			if (blob && blob.size > 0) {
				pendingBlobs.push(blob);
				pendingBytes += blob.size;
			}

			if (pendingBytes === 0) return;
			if (!forceFlush && pendingBytes < MIN_PART_BYTES) return;

			const payload = new Blob(pendingBlobs, { type: 'audio/webm' });
			const partNumber = nextPartNumber;
			pendingBlobs = [];
			pendingBytes = 0;

			const presignResult = await tryCatchAsync(() =>
				callLiveApi<PresignLiveAudioRecordingPartResponse>(
					'POST',
					`/${liveRecordingId}/presign`,
					{ partNumber }
				)
			);
			if (presignResult.err !== null) throw presignResult.err;

			const etag = await putPartToSignedUrl(payload, presignResult.ok.uploadUrl);

			const ackResult = await tryCatchAsync(() =>
				callLiveApi<AckLiveAudioRecordingPartResponse>('POST', `/${liveRecordingId}/ack`, {
					partNumber,
					etag,
					sizeBytes: payload.size
				})
			);
			if (ackResult.err !== null) throw ackResult.err;

			nextPartNumber = ackResult.ok.liveAudioRecording.nextPartNumber;
		});
	}

	// ── Volume analysis (Web Audio API) ─────────────────────────────────────────

	/**
	 * Normalized volume level in the range 0..1, sampled at the rAF rate.
	 * Drives the scale of the mic icon circle during recording.
	 */
	let audioVolume = $state(0);

	let audioContext: AudioContext | null = null;
	let audioAnalyser: AnalyserNode | null = null;
	let volumeAnimationFrameId: number | null = null;

	/**
	 * Connects the active media stream to an AnalyserNode and starts a
	 * requestAnimationFrame loop that updates `audioVolume` with the RMS of the
	 * time-domain signal.
	 *
	 * @param stream - The MediaStream that is being recorded.
	 */
	function startVolumeAnalysis(stream: MediaStream): void {
		audioContext = new AudioContext();
		audioAnalyser = audioContext.createAnalyser();
		audioAnalyser.fftSize = 256;
		audioContext.createMediaStreamSource(stream).connect(audioAnalyser);

		const buffer = new Uint8Array(audioAnalyser.frequencyBinCount);

		function tick(): void {
			if (!audioAnalyser) return;
			audioAnalyser.getByteTimeDomainData(buffer);

			// Compute RMS of the normalised time-domain deviations (silence = 128).
			let sum = 0;
			for (const sample of buffer) {
				const deviation = (sample - 128) / 128;
				sum += deviation * deviation;
			}
			// Boost by 8 so typical conversational levels map close to 1.
			audioVolume = Math.min(1, Math.sqrt(sum / buffer.length) * 8);
			volumeAnimationFrameId = requestAnimationFrame(tick);
		}

		volumeAnimationFrameId = requestAnimationFrame(tick);
	}

	/** Cancels the rAF loop and closes the AudioContext. */
	function stopVolumeAnalysis(): void {
		if (volumeAnimationFrameId !== null) {
			cancelAnimationFrame(volumeAnimationFrameId);
			volumeAnimationFrameId = null;
		}
		audioAnalyser = null;
		void audioContext?.close();
		audioContext = null;
		audioVolume = 0;
	}

	// ── Recorder lifecycle ────────────────────────────────────────────────────

	function releaseMediaDevices(): void {
		stopVolumeAnalysis();
		mediaRecorder?.stop();
		mediaRecorder = null;
		if (mediaStream) {
			for (const track of mediaStream.getTracks()) track.stop();
			mediaStream = null;
		}
	}

	/** Requests any buffered audio, stops the recorder, and waits for the upload chain. */
	async function drainAndStop(liveRecordingId: string): Promise<void> {
		mediaRecorder?.requestData();
		mediaRecorder?.stop();
		// Give the browser a tick to fire the final dataavailable event.
		await new Promise<void>((resolve) => setTimeout(resolve, 50));
		releaseMediaDevices();

		// Force-flush any remaining sub-threshold bytes as the final part.
		enqueueChunk(liveRecordingId, null, true);
		await uploadChain;
	}

	// ── Public actions ────────────────────────────────────────────────────────

	async function startRecording(): Promise<void> {
		const trimmedName = recordingName.trim();
		if (!trimmedName) {
			notifications.send({
				message: 'Enter a name for the recording first',
				priority: 'ERROR'
			});
			return;
		}
		if (recordings.some((recording) => recording.name === trimmedName)) {
			notifications.send({
				message: `A recording named "${trimmedName}" already exists`,
				priority: 'ERROR'
			});
			return;
		}

		phase = 'starting';

		const createResult = await tryCatchAsync(() =>
			callLiveApi<CreateLiveAudioRecordingResponse>('POST', '', {
				name: trimmedName,
				fileExtension: 'webm'
			})
		);

		if (createResult.err !== null) {
			const error = createResult.err;
			notifications.send({
				message: error instanceof Error ? error.message : 'Failed to start recording',
				priority: 'ERROR'
			});
			phase = 'idle';
			return;
		}

		const created = createResult.ok;
		const liveRecordingId = created.liveAudioRecording.id;
		activeLiveRecordingId = liveRecordingId;

		nextPartNumber = created.liveAudioRecording.nextPartNumber;
		pendingBlobs = [];
		pendingBytes = 0;
		uploadChain = Promise.resolve();

		const micResult = await tryCatchAsync(() =>
			navigator.mediaDevices.getUserMedia({ audio: true })
		);

		if (micResult.err !== null) {
			const error = micResult.err;
			notifications.send({
				message: error instanceof Error ? error.message : 'Microphone access denied',
				priority: 'ERROR'
			});
			// Clean up the DB/S3 row created above.
			void tryCatchAsync(() => callLiveApi('DELETE', `/${liveRecordingId}`));
			activeLiveRecordingId = null;
			phase = 'idle';
			return;
		}

		mediaStream = micResult.ok;
		startVolumeAnalysis(mediaStream);

		const mimeType = MediaRecorder.isTypeSupported('audio/webm;codecs=opus')
			? 'audio/webm;codecs=opus'
			: 'audio/webm';

		const recorder = new MediaRecorder(mediaStream, { mimeType });
		mediaRecorder = recorder;

		recorder.ondataavailable = (event: BlobEvent) => {
			if (event.data.size > 0) {
				enqueueChunk(liveRecordingId, event.data, false);
			}
		};

		recorder.start(CHUNK_INTERVAL_MS);
		recordingName = '';
		phase = 'recording';
	}

	async function stopRecording(): Promise<void> {
		const liveRecordingId = activeLiveRecordingId;
		if (!liveRecordingId) return;

		phase = 'stopping';

		const stopResult = await tryCatchAsync(() => drainAndStop(liveRecordingId));

		if (stopResult.err !== null) {
			const error = stopResult.err;
			notifications.send({
				message: error instanceof Error ? error.message : 'Failed to flush recording data',
				priority: 'ERROR'
			});
			releaseMediaDevices();
			activeLiveRecordingId = null;
			phase = 'idle';
			return;
		}

		const completeResult = await tryCatchAsync(() =>
			callLiveApi<ProcessRecordingResponse>('POST', `/${liveRecordingId}/complete`)
		);

		activeLiveRecordingId = null;
		phase = 'idle';

		if (completeResult.err !== null) {
			const error = completeResult.err;
			notifications.send({
				message: error instanceof Error ? error.message : 'Failed to finalise recording',
				priority: 'ERROR'
			});
			return;
		}

		notifications.send({
			message: completeResult.ok.message || 'Recording saved — transcription started',
			priority: 'SUCCESS'
		});

		await onComplete?.();
	}

	// ── Cleanup on component destroy ──────────────────────────────────────────

	onDestroy(() => {
		releaseMediaDevices();

		if (activeLiveRecordingId) {
			// Fire-and-forget: abort the multipart upload so S3 does not accumulate orphaned parts.
			void tryCatchAsync(() => callLiveApi('DELETE', `/${activeLiveRecordingId}`));
		}
	});
</script>

<div class="flex items-center gap-4">
	{#if isIdle}
		<!--
			Mic icon acts as the visual anchor for the live recording action.
			The icon itself is not interactive; the labelled Start button is the trigger.
		-->
		<div class="bg-muted flex h-10 w-10 shrink-0 items-center justify-center rounded-full">
			<Mic class="h-5 w-5" />
		</div>
		<Input
			class="max-w-xs"
			placeholder="Recording name"
			bind:value={recordingName}
			onkeydown={(e) => {
				if (e.key === 'Enter') startRecording();
			}}
		/>
		<Button onclick={startRecording} disabled={!recordingName.trim()}>Start recording</Button>
	{:else if isStarting}
		<div class="bg-muted flex h-10 w-10 shrink-0 items-center justify-center rounded-full">
			<Loader2 class="h-5 w-5 animate-spin" />
		</div>
		<span class="text-muted-foreground text-sm">Starting…</span>
	{:else if isRecording}
		<!--
			Non-interactive mic icon signals the microphone is active.
			The circle scales subtly with input volume via Web Audio RMS analysis.
		-->
		<div
			class="bg-destructive/10 flex h-10 w-10 shrink-0 items-center justify-center rounded-full transition-transform duration-75"
			style="transform: scale({1 + audioVolume * 0.4})"
			aria-hidden="true"
		>
			<Mic class="text-destructive h-5 w-5" />
		</div>
		<div class="flex items-center gap-2">
			<span
				class="bg-destructive inline-block h-2.5 w-2.5 animate-pulse rounded-full"
				aria-hidden="true"
			></span>
			<span class="text-sm font-medium">Recording</span>
		</div>
		<Button variant="destructive" onclick={stopRecording}>
			<Square class="mr-2 h-4 w-4" />
			Stop &amp; save
		</Button>
	{:else if isStopping}
		<div class="bg-muted flex h-10 w-10 shrink-0 items-center justify-center rounded-full">
			<Loader2 class="h-5 w-5 animate-spin" />
		</div>
		<span class="text-muted-foreground text-sm">Saving recording…</span>
	{/if}
</div>
