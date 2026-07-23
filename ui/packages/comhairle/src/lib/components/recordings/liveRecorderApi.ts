import { apiClient } from '@crownshy/api-client/client';
import { tryCatchAsync } from '$lib/utils/errorHandling';

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

	async loadLiveRecordings(): Promise<LiveAudioRecordingDto[]> {
		return await apiClient.ListLiveAudioRecordings({
			params: {
				conversation_id: this.options.getConversationId(),
				event_id: this.options.getEventId()
			}
		});
	}

	async createLiveRecording(name: string): Promise<CreateLiveAudioRecordingResponse> {
		return await apiClient.CreateLiveAudioRecording(
			{
				name,
				fileExtension: 'webm'
			},
			{
				params: {
					conversation_id: this.options.getConversationId(),
					event_id: this.options.getEventId()
				}
			}
		);
	}

	async deleteLiveRecording(liveRecordingId: string): Promise<unknown> {
		return await apiClient.DeleteLiveAudioRecording(undefined, {
			params: {
				conversation_id: this.options.getConversationId(),
				event_id: this.options.getEventId(),
				live_recording_id: liveRecordingId
			}
		});
	}

	async completeLiveRecording(liveRecordingId: string): Promise<ProcessRecordingResponse> {
		return await apiClient.CompleteLiveAudioRecording(undefined, {
			params: {
				conversation_id: this.options.getConversationId(),
				event_id: this.options.getEventId(),
				live_recording_id: liveRecordingId
			}
		});
	}
}
