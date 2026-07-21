<script lang="ts">
	import { Mic, Square, Loader2, Pause, Play, Trash2 } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { WSConnection } from '$lib/api/websockets.svelte';
	import { invalidateAll } from '$app/navigation';
	import { notifications } from '$lib/notifications.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { onDestroy, onMount } from 'svelte';
	import type { AudioRecordingDto } from '@crownshy/api-client/api';

	type Props = {
		conversation_id: string;
		event_id: string;
		recordings: AudioRecordingDto[];
		onComplete?: () => void | Promise<void>;
		mode?: 'admin' | 'participant';
	};

	let { conversation_id, event_id, recordings, onComplete, mode = 'admin' }: Props = $props();

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

	type PresignLiveAudioRecordingPartRequest = {
		conversationId: string;
		eventId: string;
		liveRecordingId: string;
		partNumber: number;
	};

	type AckLiveAudioRecordingPartResponse = {
		liveAudioRecording: LiveAudioRecordingDto;
	};

	type AckLiveAudioRecordingPartRequest = {
		conversationId: string;
		eventId: string;
		liveRecordingId: string;
		partNumber: number;
		etag: string;
		sizeBytes: number;
	};

	type ProcessRecordingResponse = {
		message: string;
		jobId: string;
	};

	type LiveRecordingAcquireRequest = {
		eventId: string;
		liveRecordingId: string;
	};

	const CHUNK_INTERVAL_MS = 10_000;
	const MIN_PART_BYTES = 5 * 1024 * 1024;
	const MIN_RECORDING_BYTES = 5 * 1024 * 1024;
	const TARGET_AUDIO_BITS_PER_SECOND = 768_000;
	const DEFAULT_AUDIO_BITS_PER_SECOND = 128_000;

	let recordingName = $state('');
	let phase = $state<'idle' | 'starting' | 'recording' | 'stopping'>('idle');
	let activeLiveRecordingId = $state<string | null>(null);
	let finalisingLiveRecordingId = $state<string | null>(null);
	let liveRecordings = $state<LiveAudioRecordingDto[]>([]);

	const isIdle = $derived(phase === 'idle');
	const isStarting = $derived(phase === 'starting');
	const isRecording = $derived(phase === 'recording');
	const isStopping = $derived(phase === 'stopping');
	const hasActiveLiveRecording = $derived(
		activeLiveRecordingId !== null &&
			liveRecordings.some((liveRecording) => liveRecording.id === activeLiveRecordingId)
	);
	const isParticipantMode = $derived(mode === 'participant');
	const currentLiveRecording = $derived(liveRecordings.at(0) ?? null);
	const canStartNewRecording = $derived(
		!hasActiveLiveRecording && (!isParticipantMode || liveRecordings.length === 0)
	);

	let mediaRecorder: MediaRecorder | null = null;
	let mediaStream: MediaStream | null = null;
	let uploadChain: Promise<void> = Promise.resolve();
	let nextPartNumber = 1;
	let pendingBlobs: Blob[] = [];
	let pendingBytes = 0;
	let observedAudioBitsPerSecond = $state(0);
	let lastChunkTimestampMs: number | null = null;
	const hasObservedBitrate = $derived(observedAudioBitsPerSecond > 0);

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
			if (parseResult.err === null) {
				if (typeof parseResult.ok?.message === 'string') {
					message = parseResult.ok.message;
				} else if (typeof parseResult.ok?.err === 'string') {
					message = parseResult.ok.err;
				}
			}
			throw new Error(message);
		}

		return response.json() as Promise<T>;
	}

	async function loadLiveRecordings(): Promise<void> {
		const result = await tryCatchAsync(() => callLiveApi<LiveAudioRecordingDto[]>('GET', ''));

		if (result.err !== null) {
			notifications.send({
				message: 'Failed to load live recordings',
				priority: 'ERROR'
			});
			return;
		}

		liveRecordings = result.ok;
	}

	function liveRecordingName(liveRecording: LiveAudioRecordingDto): string {
		return (
			recordings.find((recording) => recording.id === liveRecording.audioRecordingId)?.name ??
			'Untitled live recording'
		);
	}

	function totalUploadedBytes(liveRecording: LiveAudioRecordingDto): number {
		return liveRecording.uploadedParts.reduce((sum, part) => sum + part.sizeBytes, 0);
	}

	function isRecordingLargeEnough(liveRecording: LiveAudioRecordingDto): boolean {
		return totalUploadedBytes(liveRecording) >= MIN_RECORDING_BYTES;
	}

	function formatMb(bytes: number): string {
		return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
	}

	function effectiveAudioBitsPerSecond(): number {
		return observedAudioBitsPerSecond > 0
			? observedAudioBitsPerSecond
			: DEFAULT_AUDIO_BITS_PER_SECOND;
	}

	function resetObservedBitrate(): void {
		observedAudioBitsPerSecond = 0;
		lastChunkTimestampMs = null;
	}

	function trackObservedBitrate(chunkSizeBytes: number): void {
		if (chunkSizeBytes <= 0) return;

		const nowMs = Date.now();
		const intervalMs = lastChunkTimestampMs
			? Math.max(250, nowMs - lastChunkTimestampMs)
			: CHUNK_INTERVAL_MS;
		lastChunkTimestampMs = nowMs;

		const instantaneousBitsPerSecond = (chunkSizeBytes * 8 * 1000) / intervalMs;
		observedAudioBitsPerSecond =
			observedAudioBitsPerSecond <= 0
				? instantaneousBitsPerSecond
				: observedAudioBitsPerSecond * 0.7 + instantaneousBitsPerSecond * 0.3;
	}

	function minRequiredSecondsForSave(): number {
		return Math.ceil((MIN_RECORDING_BYTES * 8) / effectiveAudioBitsPerSecond());
	}

	function secondsForUploadedBytes(bytes: number): number {
		return Math.floor((bytes * 8) / effectiveAudioBitsPerSecond());
	}

	function formatDuration(totalSeconds: number): string {
		const minutes = Math.floor(totalSeconds / 60);
		const seconds = totalSeconds % 60;
		if (minutes === 0) return `${seconds}s`;
		if (seconds === 0) return `${minutes}m`;
		return `${minutes}m ${seconds}s`;
	}

	function createCompatibleRecorder(stream: MediaStream, mimeType: string): MediaRecorder {
		try {
			return new MediaRecorder(stream, {
				mimeType,
				audioBitsPerSecond: TARGET_AUDIO_BITS_PER_SECOND
			});
		} catch {
			return new MediaRecorder(stream, { mimeType });
		}
	}

	function shortRecordingWarningMessage(): string {
		if (!hasObservedBitrate) {
			return `Recording is too short to save. Minimum required is ${formatMb(MIN_RECORDING_BYTES)}.`;
		}

		return `Recording is too short to save. Minimum required is ${formatMb(MIN_RECORDING_BYTES)} (about ${formatDuration(minRequiredSecondsForSave())} at current rate).`;
	}

	function isLiveRecordingMissingError(err: unknown): boolean {
		if (!(err instanceof Error)) return false;
		const message = err.message.toLowerCase();
		return (
			message.includes('live audio recording not found') ||
			message.includes('recording not found')
		);
	}

	async function acquireRecordingLock(liveRecordingId: string): Promise<void> {
		await recordingWebSocket.requestCustom<null>(
			'audio_recording:acquire',
			{
				eventId: event_id,
				liveRecordingId
			} satisfies LiveRecordingAcquireRequest,
			{ responseEvent: 'audio_recording:acquire_result', timeoutMs: 20_000 }
		);
	}

	async function releaseRecordingLock(): Promise<void> {
		if (!activeLiveRecordingId || phase === 'idle') return;

		await recordingWebSocket.requestCustom<null>(
			'audio_recording:release',
			{},
			{ responseEvent: 'audio_recording:release_result', timeoutMs: 20_000 }
		);
	}

	async function releaseRecordingLockBestEffort(): Promise<void> {
		const releaseResult = await tryCatchAsync(() => releaseRecordingLock());
		if (releaseResult.err !== null) {
			console.warn('Failed to release live recording lock:', releaseResult.err);
		}
	}

	async function recoverFromMissingLiveRecording(): Promise<void> {
		pendingBlobs = [];
		pendingBytes = 0;
		activeLiveRecordingId = null;
		phase = 'idle';
		recordingWebSocket.disconnect();
		releaseMediaDevices();
		await loadLiveRecordings();
	}

	async function putPartToSignedUrl(blob: Blob, url: string): Promise<string> {
		const response = await fetch(url, { method: 'PUT', body: blob });
		if (!response.ok) throw new Error(`Part upload failed (${response.status})`);
		const etag = response.headers.get('etag') ?? response.headers.get('ETag') ?? '';
		return etag.replaceAll('"', '');
	}

	function enqueueChunk(liveRecordingId: string, blob: Blob | null, forceFlush: boolean): void {
		uploadChain = uploadChain
			.then(async () => {
				if (blob && blob.size > 0) {
					pendingBlobs.push(blob);
					pendingBytes += blob.size;
				}

				if (pendingBytes === 0) return;
				if (!forceFlush && pendingBytes < MIN_RECORDING_BYTES) return;

				const payload = new Blob(pendingBlobs, { type: 'audio/webm' });
				const partNumber = nextPartNumber;
				pendingBlobs = [];
				pendingBytes = 0;

				const presignResponse =
					await recordingWebSocket.requestCustom<PresignLiveAudioRecordingPartResponse>(
						'audio_recording:presign_part',
						{
							conversationId: conversation_id,
							eventId: event_id,
							liveRecordingId,
							partNumber
						} satisfies PresignLiveAudioRecordingPartRequest,
						{ responseEvent: 'audio_recording:presign_part_result', timeoutMs: 20_000 }
					);

				const etag = await putPartToSignedUrl(payload, presignResponse.uploadUrl);

				const ackResponse =
					await recordingWebSocket.requestCustom<AckLiveAudioRecordingPartResponse>(
						'audio_recording:ack_part',
						{
							conversationId: conversation_id,
							eventId: event_id,
							liveRecordingId,
							partNumber,
							etag,
							sizeBytes: payload.size
						} satisfies AckLiveAudioRecordingPartRequest,
						{ responseEvent: 'audio_recording:ack_part_result', timeoutMs: 20_000 }
					);

				nextPartNumber = ackResponse.liveAudioRecording.nextPartNumber;
				liveRecordings = liveRecordings.map((liveRecording) =>
					liveRecording.id === ackResponse.liveAudioRecording.id
						? ackResponse.liveAudioRecording
						: liveRecording
				);
			})
			.catch(async (err: unknown) => {
				if (isLiveRecordingMissingError(err)) {
					await recoverFromMissingLiveRecording();
					notifications.send({
						message: 'Live recording no longer exists. State has been refreshed.',
						priority: 'WARNING'
					});
					return;
				}

				notifications.send({
					message:
						err instanceof Error ? err.message : 'Failed to upload recording chunk',
					priority: 'ERROR'
				});
			});
	}

	let audioVolume = $state(0);
	let audioContext: AudioContext | null = null;
	let audioAnalyser: AnalyserNode | null = null;
	let volumeAnimationFrameId: number | null = null;
	const recordingWebSocket = new WSConnection();
	const unsubscribeRecordingWebSocketClose = recordingWebSocket.onClose(() => {
		if (phase !== 'recording') return;
		void pauseRecording();
		notifications.send({
			message: 'Live recording paused because websocket disconnected',
			priority: 'WARNING'
		});
	});

	function startVolumeAnalysis(stream: MediaStream): void {
		audioContext = new AudioContext();
		audioAnalyser = audioContext.createAnalyser();
		audioAnalyser.fftSize = 256;
		audioContext.createMediaStreamSource(stream).connect(audioAnalyser);
		const buffer = new Uint8Array(audioAnalyser.frequencyBinCount);

		function tick(): void {
			if (!audioAnalyser) return;
			audioAnalyser.getByteTimeDomainData(buffer);
			let sum = 0;
			for (const sample of buffer) {
				const deviation = (sample - 128) / 128;
				sum += deviation * deviation;
			}
			audioVolume = Math.min(1, Math.sqrt(sum / buffer.length) * 8);
			volumeAnimationFrameId = requestAnimationFrame(tick);
		}

		volumeAnimationFrameId = requestAnimationFrame(tick);
	}

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

	function releaseMediaDevices(): void {
		stopVolumeAnalysis();
		mediaRecorder?.stop();
		mediaRecorder = null;
		if (mediaStream) {
			for (const track of mediaStream.getTracks()) track.stop();
			mediaStream = null;
		}
	}

	async function drainAndStop(liveRecordingId: string): Promise<void> {
		mediaRecorder?.requestData();
		mediaRecorder?.stop();
		await new Promise<void>((resolve) => setTimeout(resolve, 50));
		releaseMediaDevices();
		enqueueChunk(liveRecordingId, null, true);
		await uploadChain;
	}

	async function startRecording(): Promise<void> {
		if (!canStartNewRecording) {
			notifications.send({
				message: 'Resume or finalise your recording first',
				priority: 'WARNING'
			});
			return;
		}

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
			notifications.send({
				message:
					createResult.err instanceof Error
						? createResult.err.message
						: 'Failed to start recording',
				priority: 'ERROR'
			});
			phase = 'idle';
			return;
		}

		const created = createResult.ok;
		const liveRecordingId = created.liveAudioRecording.id;
		activeLiveRecordingId = liveRecordingId;
		liveRecordings = [...liveRecordings, created.liveAudioRecording];
		void invalidateAll();

		nextPartNumber = created.liveAudioRecording.nextPartNumber;
		pendingBlobs = [];
		pendingBytes = 0;
		uploadChain = Promise.resolve();
		resetObservedBitrate();

		recordingWebSocket.connect();
		const acquireResult = await tryCatchAsync(() => acquireRecordingLock(liveRecordingId));
		if (acquireResult.err !== null) {
			notifications.send({
				message:
					acquireResult.err instanceof Error
						? acquireResult.err.message
						: 'Failed to start recording session',
				priority: 'ERROR'
			});
			recordingWebSocket.disconnect();
			const cleanupResult = await tryCatchAsync(() =>
				callLiveApi('DELETE', `/${liveRecordingId}`)
			);
			if (cleanupResult.err === null) void invalidateAll();
			liveRecordings = liveRecordings.filter(
				(liveRecording) => liveRecording.id !== liveRecordingId
			);
			activeLiveRecordingId = null;
			phase = 'idle';
			return;
		}

		const micResult = await tryCatchAsync(() =>
			navigator.mediaDevices.getUserMedia({ audio: true })
		);
		if (micResult.err !== null) {
			notifications.send({
				message:
					micResult.err instanceof Error
						? micResult.err.message
						: 'Microphone access denied',
				priority: 'ERROR'
			});
			await releaseRecordingLockBestEffort();
			const cleanupResult = await tryCatchAsync(() =>
				callLiveApi('DELETE', `/${liveRecordingId}`)
			);
			if (cleanupResult.err === null) void invalidateAll();
			recordingWebSocket.disconnect();
			liveRecordings = liveRecordings.filter(
				(liveRecording) => liveRecording.id !== liveRecordingId
			);
			activeLiveRecordingId = null;
			phase = 'idle';
			return;
		}

		mediaStream = micResult.ok;
		startVolumeAnalysis(mediaStream);

		const mimeType = MediaRecorder.isTypeSupported('audio/webm;codecs=opus')
			? 'audio/webm;codecs=opus'
			: 'audio/webm';
		const recorder = createCompatibleRecorder(mediaStream, mimeType);
		mediaRecorder = recorder;
		// Do not use MediaRecorder-reported bitrate for estimates: container/codec compression
		// and browser encoder behavior can make actual output bitrate differ significantly.
		recorder.ondataavailable = (event: BlobEvent) => {
			if (event.data.size > 0) {
				trackObservedBitrate(event.data.size);
				enqueueChunk(liveRecordingId, event.data, false);
			}
		};

		recorder.start(CHUNK_INTERVAL_MS);
		recordingName = '';
		phase = 'recording';
	}

	async function resumeLiveRecording(liveRecordingId: string): Promise<void> {
		if (phase !== 'idle') {
			notifications.send({
				message: 'Pause or stop the current recording before resuming another one',
				priority: 'WARNING'
			});
			return;
		}

		const liveRecording = liveRecordings.find((recording) => recording.id === liveRecordingId);
		if (!liveRecording) {
			notifications.send({
				message: 'Could not find that live recording',
				priority: 'ERROR'
			});
			return;
		}

		phase = 'starting';
		activeLiveRecordingId = liveRecording.id;
		nextPartNumber = liveRecording.nextPartNumber;
		pendingBlobs = [];
		pendingBytes = 0;
		uploadChain = Promise.resolve();
		resetObservedBitrate();

		recordingWebSocket.connect();
		const acquireResult = await tryCatchAsync(() => acquireRecordingLock(liveRecording.id));
		if (acquireResult.err !== null) {
			activeLiveRecordingId = null;
			phase = 'idle';
			notifications.send({
				message:
					acquireResult.err instanceof Error
						? acquireResult.err.message
						: 'Failed to resume recording session',
				priority: 'ERROR'
			});
			recordingWebSocket.disconnect();
			return;
		}

		const microphoneResult = await tryCatchAsync(() =>
			navigator.mediaDevices.getUserMedia({ audio: true })
		);
		if (microphoneResult.err !== null) {
			await releaseRecordingLockBestEffort();
			recordingWebSocket.disconnect();
			activeLiveRecordingId = null;
			phase = 'idle';
			notifications.send({
				message:
					microphoneResult.err instanceof Error
						? microphoneResult.err.message
						: 'Microphone access denied',
				priority: 'ERROR'
			});
			return;
		}

		mediaStream = microphoneResult.ok;
		startVolumeAnalysis(mediaStream);

		const mimeType = MediaRecorder.isTypeSupported('audio/webm;codecs=opus')
			? 'audio/webm;codecs=opus'
			: 'audio/webm';
		const recorder = createCompatibleRecorder(mediaStream, mimeType);
		mediaRecorder = recorder;
		// Do not use MediaRecorder-reported bitrate for estimates: container/codec compression
		// and browser encoder behavior can make actual output bitrate differ significantly.
		recorder.ondataavailable = (event: BlobEvent) => {
			if (event.data.size > 0) {
				trackObservedBitrate(event.data.size);
				enqueueChunk(liveRecording.id, event.data, false);
			}
		};

		recorder.start(CHUNK_INTERVAL_MS);
		phase = 'recording';
	}

	async function discardLiveRecording(liveRecordingId: string): Promise<void> {
		if (activeLiveRecordingId === liveRecordingId) {
			notifications.send({
				message: 'Stop or pause this recording before discarding it',
				priority: 'WARNING'
			});
			return;
		}

		const confirmed = window.confirm(
			'Delete this recording? Uploaded parts will be permanently removed.'
		);
		if (!confirmed) return;

		await releaseRecordingLockBestEffort();
		recordingWebSocket.disconnect();
		const result = await tryCatchAsync(() => callLiveApi('DELETE', `/${liveRecordingId}`));
		if (result.err !== null) {
			notifications.send({
				message:
					result.err instanceof Error
						? result.err.message
						: 'Failed to discard live recording',
				priority: 'ERROR'
			});
			return;
		}

		liveRecordings = liveRecordings.filter(
			(liveRecording) => liveRecording.id !== liveRecordingId
		);
		void invalidateAll();
	}

	async function pauseRecording(): Promise<void> {
		const liveRecordingId = activeLiveRecordingId;
		if (!mediaRecorder || phase !== 'recording' || !liveRecordingId) return;

		phase = 'stopping';
		const pauseResult = await tryCatchAsync(() => drainAndStop(liveRecordingId));
		if (pauseResult.err === null) {
			await releaseRecordingLockBestEffort();
		}
		recordingWebSocket.disconnect();

		if (pauseResult.err !== null) {
			notifications.send({
				message:
					pauseResult.err instanceof Error
						? pauseResult.err.message
						: 'Failed to pause recording',
				priority: 'ERROR'
			});
			releaseMediaDevices();
			activeLiveRecordingId = null;
			phase = 'idle';
			return;
		}

		activeLiveRecordingId = null;
		phase = 'idle';
	}

	async function stopRecording(): Promise<void> {
		const liveRecordingId = activeLiveRecordingId;
		if (!liveRecordingId) return;

		phase = 'stopping';
		const stopResult = await tryCatchAsync(() => drainAndStop(liveRecordingId));
		if (stopResult.err === null) {
			await releaseRecordingLockBestEffort();
		}
		recordingWebSocket.disconnect();

		if (stopResult.err !== null) {
			notifications.send({
				message:
					stopResult.err instanceof Error
						? stopResult.err.message
						: 'Failed to flush recording data',
				priority: 'ERROR'
			});
			releaseMediaDevices();
			activeLiveRecordingId = null;
			phase = 'idle';
			return;
		}

		const latestLiveRecording = liveRecordings.find(
			(recording) => recording.id === liveRecordingId
		);
		if (!latestLiveRecording || !isRecordingLargeEnough(latestLiveRecording)) {
			activeLiveRecordingId = null;
			phase = 'idle';
			notifications.send({
				message: shortRecordingWarningMessage(),
				priority: 'WARNING'
			});
			return;
		}

		const completeResult = await tryCatchAsync(() =>
			callLiveApi<ProcessRecordingResponse>('POST', `/${liveRecordingId}/complete`)
		);

		activeLiveRecordingId = null;
		phase = 'idle';
		liveRecordings = liveRecordings.filter(
			(liveRecording) => liveRecording.id !== liveRecordingId
		);

		if (completeResult.err !== null) {
			notifications.send({
				message:
					completeResult.err instanceof Error
						? completeResult.err.message
						: 'Failed to finalise recording',
				priority: 'ERROR'
			});
			return;
		}

		notifications.send({
			message: completeResult.ok.message || 'Recording saved — transcription started',
			priority: 'SUCCESS'
		});
		await onComplete?.();
		void invalidateAll();
	}

	async function finaliseExistingRecording(liveRecordingId: string): Promise<void> {
		if (!isIdle || hasActiveLiveRecording) {
			notifications.send({
				message: 'Pause or stop the current recording first',
				priority: 'WARNING'
			});
			return;
		}

		const liveRecording = liveRecordings.find((recording) => recording.id === liveRecordingId);
		if (!liveRecording || !isRecordingLargeEnough(liveRecording)) {
			notifications.send({
				message: shortRecordingWarningMessage(),
				priority: 'WARNING'
			});
			return;
		}

		finalisingLiveRecordingId = liveRecordingId;
		const completeResult = await tryCatchAsync(() =>
			callLiveApi<ProcessRecordingResponse>('POST', `/${liveRecordingId}/complete`)
		);
		finalisingLiveRecordingId = null;

		if (completeResult.err !== null) {
			notifications.send({
				message:
					completeResult.err instanceof Error
						? completeResult.err.message
						: 'Failed to finalise recording',
				priority: 'ERROR'
			});
			return;
		}

		liveRecordings = liveRecordings.filter(
			(liveRecording) => liveRecording.id !== liveRecordingId
		);
		notifications.send({
			message: completeResult.ok.message || 'Recording saved — transcription started',
			priority: 'SUCCESS'
		});
		await onComplete?.();
		void invalidateAll();
	}

	onDestroy(() => {
		unsubscribeRecordingWebSocketClose();
		if (activeLiveRecordingId) {
			void releaseRecordingLockBestEffort();
		}
		recordingWebSocket.disconnect();
		releaseMediaDevices();
		if (activeLiveRecordingId) {
			void tryCatchAsync(() => callLiveApi('DELETE', `/${activeLiveRecordingId}`));
		}
	});

	onMount(() => {
		void loadLiveRecordings();
	});
</script>

<div class="flex flex-col gap-4">
	{#if currentLiveRecording}
		{@const liveRecording = currentLiveRecording}
		{@const isActiveRow = activeLiveRecordingId === liveRecording.id}
		{@const uploadedBytes = totalUploadedBytes(liveRecording)}
		{@const uploadedDurationSeconds = secondsForUploadedBytes(uploadedBytes)}
		{@const minRequiredSeconds = minRequiredSecondsForSave()}
		<div class="border-border rounded-lg border p-3">
			<div class="mb-2 text-sm font-semibold">Live recording</div>
			<div
				class="bg-muted/40 flex items-center justify-between gap-3 rounded-md p-2"
				class:opacity-50={hasActiveLiveRecording && !isActiveRow}
			>
				<div class="min-w-0">
					<div class="truncate text-sm font-medium">
						{liveRecordingName(liveRecording)}
					</div>
					<div class="text-muted-foreground text-sm">
						Next part: {liveRecording.nextPartNumber}, uploaded: {liveRecording
							.uploadedParts.length}, total: {formatMb(uploadedBytes)}
					</div>
					{#if !isRecordingLargeEnough(liveRecording)}
						<div class="text-xs text-amber-700">
							Minimum required before save: {formatMb(MIN_RECORDING_BYTES)}
						</div>
						{#if hasObservedBitrate}
							<div class="text-xs text-amber-700">
								Estimated minimum duration: {formatDuration(minRequiredSeconds)}
							</div>
						{/if}
					{/if}
				</div>
				<div class="flex items-center gap-2">
					{#if isActiveRow}
						{#if isStarting}
							<div class="text-muted-foreground flex items-center gap-2 text-sm">
								<Loader2 class="h-4 w-4 animate-spin" />
								Starting…
							</div>
						{:else if isRecording}
							<div
								class="bg-destructive/10 flex h-8 w-8 items-center justify-center rounded-full transition-transform duration-75"
								style="transform: scale({1 + audioVolume * 0.35})"
								aria-hidden="true"
							>
								<Mic class="text-destructive h-4 w-4" />
							</div>
							<Button variant="outline" size="sm" onclick={pauseRecording}>
								<Pause class="mr-2 h-4 w-4" />
								Pause
							</Button>
							<Button variant="destructive" size="sm" onclick={stopRecording}>
								<Square class="mr-2 h-4 w-4" />
								Stop
							</Button>
						{:else if isStopping}
							<div class="text-muted-foreground flex items-center gap-2 text-sm">
								<Loader2 class="h-4 w-4 animate-spin" />
								Saving…
							</div>
						{/if}
					{:else}
						<Button
							variant="outline"
							size="sm"
							disabled={hasActiveLiveRecording || finalisingLiveRecordingId !== null}
							onclick={() => resumeLiveRecording(liveRecording.id)}
						>
							<Play class="mr-2 h-4 w-4" />
							Resume
						</Button>
						{#if isParticipantMode}
							<Button
								variant="destructive"
								size="sm"
								disabled={hasActiveLiveRecording ||
									finalisingLiveRecordingId !== null}
								onclick={() => finaliseExistingRecording(liveRecording.id)}
							>
								<Square class="mr-2 h-4 w-4" />
								{finalisingLiveRecordingId === liveRecording.id
									? 'Saving…'
									: 'Stop'}
							</Button>
							<Button
								variant="ghost"
								size="sm"
								disabled={hasActiveLiveRecording ||
									finalisingLiveRecordingId !== null}
								onclick={() => discardLiveRecording(liveRecording.id)}
							>
								<Trash2 class="h-4 w-4" />
								<span class="sr-only">Delete recording</span>
							</Button>
						{:else}
							<Button
								variant="ghost"
								size="sm"
								disabled={hasActiveLiveRecording ||
									finalisingLiveRecordingId !== null}
								onclick={() => discardLiveRecording(liveRecording.id)}
							>
								<Trash2 class="h-4 w-4" />
							</Button>
						{/if}
					{/if}
				</div>
			</div>
		</div>
	{/if}

	{#if isParticipantMode && liveRecordings.length > 0 && !hasActiveLiveRecording}
		<p class="text-muted-foreground text-sm">
			Resume or finalise your recording before starting a new one.
		</p>
	{/if}

	{#if liveRecordings.length === 0}
		<div class="flex items-center gap-4" class:opacity-50={hasActiveLiveRecording}>
			<div class="bg-muted flex h-10 w-10 shrink-0 items-center justify-center rounded-full">
				<Mic class="h-5 w-5" />
			</div>
			<Input
				class="max-w-xs"
				placeholder="Recording name"
				disabled={hasActiveLiveRecording || !canStartNewRecording}
				bind:value={recordingName}
				onkeydown={(e) => {
					if (e.key === 'Enter') startRecording();
				}}
			/>
			<Button
				onclick={startRecording}
				disabled={!canStartNewRecording || !recordingName.trim()}>Start recording</Button
			>
		</div>
	{/if}
</div>
