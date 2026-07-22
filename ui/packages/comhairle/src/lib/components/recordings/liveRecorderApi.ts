import { apiClient } from '@crownshy/api-client/client';

import type {
	CreateLiveAudioRecordingResponse,
	LiveAudioRecordingDto,
	ProcessRecordingResponse
} from './liveRecorderShared';

type ApiOptions = {
	getConversationId: () => string;
	getEventId: () => string;
};

export class LiveRecorderApi {
	constructor(private options: ApiOptions) {}

	private conversationParams() {
		return {
			conversation_id: this.options.getConversationId(),
			event_id: this.options.getEventId()
		};
	}

	async loadLiveRecordings(): Promise<LiveAudioRecordingDto[]> {
		return await apiClient.ListLiveAudioRecordings({
			params: this.conversationParams()
		});
	}

	async createLiveRecording(name: string): Promise<CreateLiveAudioRecordingResponse> {
		return await apiClient.CreateLiveAudioRecording(
			{
				name,
				fileExtension: 'webm'
			},
			{
				params: this.conversationParams()
			}
		);
	}

	async getRecordingUploadUrl(recordingId: string): Promise<string> {
		const response = await fetch(
			`/api/conversation/${this.options.getConversationId()}/events/${this.options.getEventId()}/audio_recordings/${recordingId}/upload_url`
		);
		if (!response.ok) {
			let errorMessage = `Failed to get upload URL (${response.status})`;
			const payload = await response.json().catch(() => null);
			if (payload && typeof payload === 'object' && 'err' in payload) {
				const err = payload.err;
				if (typeof err === 'string' && err.length > 0) errorMessage = err;
			}
			throw new Error(errorMessage);
		}

		const payload = (await response.json()) as { uploadUrl?: string; upload_url?: string };
		const uploadUrl = payload.uploadUrl ?? payload.upload_url;
		if (!uploadUrl) throw new Error('Upload URL missing from response');
		return uploadUrl;
	}

	async processRecording(recordingId: string): Promise<ProcessRecordingResponse> {
		return await apiClient.ProcessAudioRecording(undefined, {
			params: {
				...this.conversationParams(),
				recording_id: recordingId
			}
		});
	}

	uploadBlobToSignedUrl(
		blob: Blob,
		uploadUrl: string,
		onProgress?: (progressPercent: number) => void
	): Promise<void> {
		return new Promise((resolve, reject) => {
			const xhr = new XMLHttpRequest();
			xhr.open('PUT', uploadUrl, true);
			xhr.upload.onprogress = (event) => {
				if (!event.lengthComputable || !onProgress) return;
				onProgress(Math.round((event.loaded / event.total) * 100));
			};
			xhr.onload = () => {
				if (xhr.status >= 200 && xhr.status < 300) {
					onProgress?.(100);
					resolve();
					return;
				}
				reject(new Error(`Upload failed (${xhr.status})`));
			};
			xhr.onerror = () => reject(new Error('Network error during upload'));
			xhr.send(blob);
		});
	}
}
