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
}
