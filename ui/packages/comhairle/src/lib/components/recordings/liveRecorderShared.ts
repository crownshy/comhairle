import type { AudioRecordingDto } from '@crownshy/api-client/api';

export type LiveRecorderMode = 'admin' | 'participant';
export type LiveRecorderPhase = 'idle' | 'starting' | 'recording' | 'stopping';

export type LiveRecorderProps = {
	conversation_id: string;
	event_id: string;
	recordings: AudioRecordingDto[];
	onComplete?: () => void | Promise<void>;
	mode?: LiveRecorderMode;
};

export type UploadedPart = {
	partNumber: number;
	etag: string;
	sizeBytes: number;
};

export type LiveAudioRecordingDto = {
	id: string;
	audioRecordingId: string;
	multipartUploadId: string;
	nextPartNumber: number;
	uploadedParts: UploadedPart[];
};

export type CreateLiveAudioRecordingResponse = {
	recording: AudioRecordingDto;
	liveAudioRecording: LiveAudioRecordingDto;
};

export type PresignLiveAudioRecordingPartResponse = {
	uploadUrl: string;
	partNumber: number;
};

export type PresignLiveAudioRecordingPartRequest = {
	conversationId: string;
	eventId: string;
	liveRecordingId: string;
	partNumber: number;
};

export type AckLiveAudioRecordingPartResponse = {
	liveAudioRecording: LiveAudioRecordingDto;
};

export type AckLiveAudioRecordingPartRequest = {
	conversationId: string;
	eventId: string;
	liveRecordingId: string;
	partNumber: number;
	etag: string;
	sizeBytes: number;
};

export type ProcessRecordingResponse = {
	message: string;
	jobId: string;
};

export type LiveRecordingAcquireRequest = {
	eventId: string;
	liveRecordingId: string;
};

export type LiveRecordingDisconnectSessionsRequest = {
	eventId: string;
	liveRecordingId: string;
};

export type LiveRecordingDisconnectSessionsResponse = {
	disconnectedSessions: number;
};

export type LiveRecordingCompleteRequest = {
	conversationId: string;
	eventId: string;
	liveRecordingId: string;
};

export type LiveRecordingDeleteRequest = {
	eventId: string;
	liveRecordingId: string;
};

export type LiveAudioRecordingStateResponse = {
	liveAudioRecording: LiveAudioRecordingDto;
};

export const CHUNK_INTERVAL_MS = 10_000;
export const MIN_RECORDING_BYTES = 5 * 1024 * 1024;
export const TARGET_AUDIO_BITS_PER_SECOND = 768_000;
export const DEFAULT_AUDIO_BITS_PER_SECOND = 128_000;

export function totalUploadedBytes(liveRecording: LiveAudioRecordingDto): number {
	return liveRecording.uploadedParts.reduce((sum, part) => sum + part.sizeBytes, 0);
}

export function isRecordingLargeEnough(liveRecording: LiveAudioRecordingDto): boolean {
	return totalUploadedBytes(liveRecording) >= MIN_RECORDING_BYTES;
}

export function formatMb(bytes: number): string {
	return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
}

export function formatDuration(totalSeconds: number): string {
	const minutes = Math.floor(totalSeconds / 60);
	const seconds = totalSeconds % 60;
	if (minutes === 0) return `${seconds}s`;
	if (seconds === 0) return `${minutes}m`;
	return `${minutes}m ${seconds}s`;
}

export function getErrorMessage(err: unknown, fallback: string): string {
	return err instanceof Error ? err.message : fallback;
}

export function isLiveRecordingMissingError(err: unknown): boolean {
	if (!(err instanceof Error)) return false;
	const message = err.message.toLowerCase();
	return (
		message.includes('live audio recording not found') ||
		message.includes('recording not found')
	);
}
