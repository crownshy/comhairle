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

	loadLiveRecordings(): Promise<LiveAudioRecordingDto[]> {
		return this.call<LiveAudioRecordingDto[]>('GET', '');
	}

	createLiveRecording(name: string): Promise<CreateLiveAudioRecordingResponse> {
		return this.call<CreateLiveAudioRecordingResponse>('POST', '', {
			name,
			fileExtension: 'webm'
		});
	}

	deleteLiveRecording(liveRecordingId: string): Promise<unknown> {
		return this.call('DELETE', `/${liveRecordingId}`);
	}

	completeLiveRecording(liveRecordingId: string): Promise<ProcessRecordingResponse> {
		return this.call<ProcessRecordingResponse>('POST', `/${liveRecordingId}/complete`);
	}

	private basePath(): string {
		return `/api/conversation/${this.options.getConversationId()}/events/${this.options.getEventId()}/audio_recordings/live`;
	}

	private async call<T>(method: string, path: string, body?: unknown): Promise<T> {
		const response = await fetch(`${this.basePath()}${path}`, {
			method,
			credentials: 'include',
			headers: body ? { 'content-type': 'application/json' } : undefined,
			body: body ? JSON.stringify(body) : undefined
		});

		if (!response.ok) {
			let message = `${method} ${path} failed (${response.status})`;
			const parseResult = await tryCatchAsync(() => response.json());
			if (parseResult.err === null) {
				if (typeof parseResult.ok?.message === 'string') message = parseResult.ok.message;
				else if (typeof parseResult.ok?.err === 'string') message = parseResult.ok.err;
			}
			throw new Error(message);
		}

		return response.json() as Promise<T>;
	}
}
