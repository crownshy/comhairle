import { WSConnection } from '$lib/api/websockets.svelte';
import { notifications } from '$lib/notifications.svelte';
import { tryCatchAsync } from '$lib/utils/errorHandling';

import {
	CHUNK_INTERVAL_MS,
	DEFAULT_AUDIO_BITS_PER_SECOND,
	MIN_RECORDING_BYTES,
	TARGET_AUDIO_BITS_PER_SECOND,
	formatDuration,
	formatMb,
	getErrorMessage,
	isLiveRecordingMissingError,
	type AckLiveAudioRecordingPartRequest,
	type AckLiveAudioRecordingPartResponse,
	type LiveAudioRecordingStateResponse,
	type LiveRecordingCompleteRequest,
	type LiveRecordingDeleteRequest,
	type LiveAudioRecordingDto,
	type LiveRecordingDisconnectSessionsRequest as LiveRecordingDisconnectSessionRequest,
	type LiveRecorderPhase,
	type LiveRecordingAcquireRequest,
	type ProcessRecordingResponse,
	type PresignLiveAudioRecordingPartRequest,
	type PresignLiveAudioRecordingPartResponse
} from './liveRecorderShared';

type EngineOptions = {
	getConversationId: () => string;
	getEventId: () => string;
	getActiveLiveRecordingId: () => string | null;
	getPhase: () => LiveRecorderPhase;
	onPausedByDisconnect: () => Promise<void>;
	onMissingRecording: () => Promise<void>;
	onRecordingUpdated: (liveRecording: LiveAudioRecordingDto) => void;
};

export class LiveRecorderEngine {
	audioVolume = $state(0);
	observedAudioBitsPerSecond = $state(0);

	private mediaRecorder: MediaRecorder | null = null;
	private mediaStream: MediaStream | null = null;
	private audioContext: AudioContext | null = null;
	private audioAnalyser: AnalyserNode | null = null;
	private volumeAnimationFrameId: number | null = null;
	private uploadChain: Promise<void> = Promise.resolve();
	private nextPartNumber = 1;
	private pendingBlobs: Blob[] = [];
	private pendingBytes = 0;
	private lastChunkTimestampMs: number | null = null;
	private recordingWebSocket = new WSConnection();
	private unsubscribeRecordingWebSocketClose: () => void;
	private unsubscribeRecordingWebSocketMessages: () => void;

	constructor(private options: EngineOptions) {
		this.unsubscribeRecordingWebSocketClose = this.recordingWebSocket.onClose(() => {
			if (this.options.getPhase() !== 'recording') return;
			void this.options.onPausedByDisconnect();
			notifications.send({
				message: 'Live recording paused because websocket disconnected',
				priority: 'WARNING'
			});
		});

		this.unsubscribeRecordingWebSocketMessages = this.recordingWebSocket.onMessage(
			(message) => {
				if (message.type !== 'custom') return;
				if (message.payload.event !== 'audio_recording:disconnect') return;
				if (message.payload.data?.eventId !== this.options.getEventId()) return;
				if (
					message.payload.data?.liveRecordingId !==
					this.options.getActiveLiveRecordingId()
				)
					return;
				if (this.options.getPhase() !== 'recording') return;

				void this.options.onPausedByDisconnect();
				notifications.send({
					message: 'Live recording paused because another session took over',
					priority: 'WARNING'
				});
			}
		);
	}

	get hasObservedBitrate(): boolean {
		return this.observedAudioBitsPerSecond > 0;
	}

	minRequiredSecondsForSave(): number {
		return Math.ceil((MIN_RECORDING_BYTES * 8) / this.effectiveAudioBitsPerSecond());
	}

	shortRecordingWarningMessage(): string {
		if (!this.hasObservedBitrate) {
			return `Recording is too short to save. Minimum required is ${formatMb(MIN_RECORDING_BYTES)}.`;
		}
		return `Recording is too short to save. Minimum required is ${formatMb(MIN_RECORDING_BYTES)} (about ${formatDuration(this.minRequiredSecondsForSave())} at current rate).`;
	}

	connect(): void {
		this.recordingWebSocket.connect();
	}

	disconnect(): void {
		this.recordingWebSocket.disconnect();
	}

	resetUploadState(nextPartNumber: number): void {
		this.nextPartNumber = nextPartNumber;
		this.pendingBlobs = [];
		this.pendingBytes = 0;
		this.uploadChain = Promise.resolve();
		this.observedAudioBitsPerSecond = 0;
		this.lastChunkTimestampMs = null;
	}

	async acquireRecordingLock(liveRecordingId: string): Promise<void> {
		await this.recordingWebSocket.requestCustom<null>(
			'audio_recording:acquire',
			{
				eventId: this.options.getEventId(),
				liveRecordingId
			} satisfies LiveRecordingAcquireRequest,
			{ responseEvent: 'audio_recording:acquire_result', timeoutMs: 20_000 }
		);
	}

	async disconnectSessionForRecording(liveRecordingId: string): Promise<void> {
		await this.recordingWebSocket.requestCustom<null>(
			'audio_recording:disconnect_sessions',
			{
				eventId: this.options.getEventId(),
				liveRecordingId
			} satisfies LiveRecordingDisconnectSessionRequest,
			{ responseEvent: 'audio_recording:disconnect_sessions_result', timeoutMs: 20_000 }
		);
	}

	async releaseRecordingLockBestEffort(): Promise<void> {
		const releaseResult = await tryCatchAsync(() => this.releaseRecordingLock());
		if (releaseResult.err !== null) {
			console.warn('Failed to release live recording lock:', releaseResult.err);
		}
	}

	async completeLiveRecording(liveRecordingId: string): Promise<ProcessRecordingResponse> {
		return await this.recordingWebSocket.requestCustom<ProcessRecordingResponse>(
			'audio_recording:complete',
			{
				conversationId: this.options.getConversationId(),
				eventId: this.options.getEventId(),
				liveRecordingId
			} satisfies LiveRecordingCompleteRequest,
			{ responseEvent: 'audio_recording:complete_result', timeoutMs: 30_000 }
		);
	}

	async deleteLiveRecording(liveRecordingId: string): Promise<LiveAudioRecordingStateResponse> {
		return await this.recordingWebSocket.requestCustom<LiveAudioRecordingStateResponse>(
			'audio_recording:delete',
			{
				eventId: this.options.getEventId(),
				liveRecordingId
			} satisfies LiveRecordingDeleteRequest,
			{ responseEvent: 'audio_recording:delete_result', timeoutMs: 30_000 }
		);
	}

	async prepareMicrophone(): Promise<MediaStream | null> {
		const microphoneResult = await tryCatchAsync(() =>
			navigator.mediaDevices.getUserMedia({ audio: true })
		);
		if (microphoneResult.err !== null) {
			notifications.send({
				message: getErrorMessage(microphoneResult.err, 'Microphone access denied'),
				priority: 'ERROR'
			});
			return null;
		}
		this.mediaStream = microphoneResult.ok;
		this.startVolumeAnalysis(this.mediaStream);
		return this.mediaStream;
	}

	startRecorder(stream: MediaStream, liveRecordingId: string): void {
		const mimeType = MediaRecorder.isTypeSupported('audio/webm;codecs=opus')
			? 'audio/webm;codecs=opus'
			: 'audio/webm';
		this.mediaRecorder = this.createCompatibleRecorder(stream, mimeType);
		this.mediaRecorder.ondataavailable = (event: BlobEvent) => {
			if (event.data.size <= 0) return;
			this.trackObservedBitrate(event.data.size);
			this.enqueueChunk(liveRecordingId, event.data, false);
		};
		this.mediaRecorder.start(CHUNK_INTERVAL_MS);
	}

	async drainAndStop(liveRecordingId: string): Promise<void> {
		this.mediaRecorder?.requestData();
		this.mediaRecorder?.stop();
		await new Promise<void>((resolve) => setTimeout(resolve, 50));
		this.releaseMediaDevices();
		this.enqueueChunk(liveRecordingId, null, true);
		await this.uploadChain;
	}

	destroy(): void {
		this.unsubscribeRecordingWebSocketClose();
		void this.releaseRecordingLockBestEffort();
		this.recordingWebSocket.disconnect();
		this.releaseMediaDevices();
	}

	private effectiveAudioBitsPerSecond(): number {
		return this.observedAudioBitsPerSecond > 0
			? this.observedAudioBitsPerSecond
			: DEFAULT_AUDIO_BITS_PER_SECOND;
	}

	private trackObservedBitrate(chunkSizeBytes: number): void {
		if (chunkSizeBytes <= 0) return;
		const nowMs = Date.now();
		const intervalMs = this.lastChunkTimestampMs
			? Math.max(250, nowMs - this.lastChunkTimestampMs)
			: CHUNK_INTERVAL_MS;
		this.lastChunkTimestampMs = nowMs;
		const instantaneousBitsPerSecond = (chunkSizeBytes * 8 * 1000) / intervalMs;
		this.observedAudioBitsPerSecond =
			this.observedAudioBitsPerSecond <= 0
				? instantaneousBitsPerSecond
				: this.observedAudioBitsPerSecond * 0.7 + instantaneousBitsPerSecond * 0.3;
	}

	private async releaseRecordingLock(): Promise<void> {
		const activeLiveRecordingId = this.options.getActiveLiveRecordingId();
		if (!activeLiveRecordingId || this.options.getPhase() === 'idle') return;
		await this.recordingWebSocket.requestCustom<null>(
			'audio_recording:release',
			{},
			{ responseEvent: 'audio_recording:release_result', timeoutMs: 20_000 }
		);
	}

	private createCompatibleRecorder(stream: MediaStream, mimeType: string): MediaRecorder {
		try {
			return new MediaRecorder(stream, {
				mimeType,
				audioBitsPerSecond: TARGET_AUDIO_BITS_PER_SECOND
			});
		} catch {
			return new MediaRecorder(stream, { mimeType });
		}
	}

	private startVolumeAnalysis(stream: MediaStream): void {
		this.audioContext = new AudioContext();
		this.audioAnalyser = this.audioContext.createAnalyser();
		this.audioAnalyser.fftSize = 256;
		this.audioContext.createMediaStreamSource(stream).connect(this.audioAnalyser);
		const buffer = new Uint8Array(this.audioAnalyser.frequencyBinCount);
		const tick = () => {
			if (!this.audioAnalyser) return;
			this.audioAnalyser.getByteTimeDomainData(buffer);
			let sum = 0;
			for (const sample of buffer) sum += ((sample - 128) / 128) ** 2;
			this.audioVolume = Math.min(1, Math.sqrt(sum / buffer.length) * 8);
			this.volumeAnimationFrameId = requestAnimationFrame(tick);
		};
		this.volumeAnimationFrameId = requestAnimationFrame(tick);
	}

	private stopVolumeAnalysis(): void {
		if (this.volumeAnimationFrameId !== null) {
			cancelAnimationFrame(this.volumeAnimationFrameId);
			this.volumeAnimationFrameId = null;
		}
		this.audioAnalyser = null;
		void this.audioContext?.close();
		this.audioContext = null;
		this.audioVolume = 0;
	}

	private releaseMediaDevices(): void {
		this.stopVolumeAnalysis();
		this.mediaRecorder?.stop();
		this.mediaRecorder = null;
		if (!this.mediaStream) return;
		for (const track of this.mediaStream.getTracks()) track.stop();
		this.mediaStream = null;
	}

	private enqueueChunk(liveRecordingId: string, blob: Blob | null, forceFlush: boolean): void {
		this.uploadChain = this.uploadChain
			.then(async () => {
				if (blob && blob.size > 0) {
					this.pendingBlobs.push(blob);
					this.pendingBytes += blob.size;
				}
				if (
					this.pendingBytes === 0 ||
					(!forceFlush && this.pendingBytes < MIN_RECORDING_BYTES)
				)
					return;
				const payload = new Blob(this.pendingBlobs, { type: 'audio/webm' });
				const partNumber = this.nextPartNumber;
				this.pendingBlobs = [];
				this.pendingBytes = 0;
				const presignResponse =
					await this.recordingWebSocket.requestCustom<PresignLiveAudioRecordingPartResponse>(
						'audio_recording:presign_part',
						{
							conversationId: this.options.getConversationId(),
							eventId: this.options.getEventId(),
							liveRecordingId,
							partNumber
						} satisfies PresignLiveAudioRecordingPartRequest,
						{ responseEvent: 'audio_recording:presign_part_result', timeoutMs: 20_000 }
					);
				const etag = await this.putPartToSignedUrl(payload, presignResponse.uploadUrl);
				const ackResponse =
					await this.recordingWebSocket.requestCustom<AckLiveAudioRecordingPartResponse>(
						'audio_recording:ack_part',
						{
							conversationId: this.options.getConversationId(),
							eventId: this.options.getEventId(),
							liveRecordingId,
							partNumber,
							etag,
							sizeBytes: payload.size
						} satisfies AckLiveAudioRecordingPartRequest,
						{ responseEvent: 'audio_recording:ack_part_result', timeoutMs: 20_000 }
					);
				this.nextPartNumber = ackResponse.liveAudioRecording.nextPartNumber;
				this.options.onRecordingUpdated(ackResponse.liveAudioRecording);
			})
			.catch(async (err: unknown) => {
				if (isLiveRecordingMissingError(err)) {
					await this.options.onMissingRecording();
					notifications.send({
						message: 'Live recording no longer exists. State has been refreshed.',
						priority: 'WARNING'
					});
					return;
				}
				notifications.send({
					message: getErrorMessage(err, 'Failed to upload recording chunk'),
					priority: 'ERROR'
				});
			});
	}

	private async putPartToSignedUrl(blob: Blob, url: string): Promise<string> {
		const response = await fetch(url, { method: 'PUT', body: blob });
		if (!response.ok) throw new Error(`Part upload failed (${response.status})`);
		const etag = response.headers.get('etag') ?? response.headers.get('ETag') ?? '';
		return etag.replaceAll('"', '');
	}
}
