import { invalidateAll } from '$app/navigation';
import { notifications } from '$lib/notifications.svelte';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { AudioRecordingDto } from '@crownshy/api-client/api';

import {
	getErrorMessage,
	type LiveAudioRecordingDto,
	type LiveRecorderMode,
	type LiveRecorderPhase
} from './liveRecorderShared';
import { LiveRecorderApi } from './liveRecorderApi';
import { LiveRecorderEngine } from './liveRecorderEngine.svelte';

type ControllerContext = {
	conversationId: string;
	eventId: string;
	recordings: AudioRecordingDto[];
	onComplete?: () => void | Promise<void>;
	mode: LiveRecorderMode;
};

export class LiveRecorderController {
	recordingName = $state('');
	phase = $state<LiveRecorderPhase>('idle');
	activeLiveRecordingId = $state<string | null>(null);
	finalisingLiveRecordingId = $state<string | null>(null);
	liveRecordings = $state<LiveAudioRecordingDto[]>([]);

	private conversationId = $state('');
	private eventId = $state('');
	private recordings = $state<AudioRecordingDto[]>([]);
	private mode = $state<LiveRecorderMode>('admin');
	private onComplete?: () => void | Promise<void>;
	private api: LiveRecorderApi;
	private engine: LiveRecorderEngine;

	constructor(context: ControllerContext) {
		this.api = new LiveRecorderApi({
			getConversationId: () => this.conversationId,
			getEventId: () => this.eventId
		});
		this.engine = new LiveRecorderEngine({
			getConversationId: () => this.conversationId,
			getEventId: () => this.eventId,
			getActiveLiveRecordingId: () => this.activeLiveRecordingId,
			getPhase: () => this.phase,
			onPausedByDisconnect: () => this.pauseRecording(),
			onMissingRecording: () => this.recoverFromMissingLiveRecording(),
			onRecordingUpdated: (liveRecording) => this.updateLiveRecording(liveRecording)
		});
		this.updateContext(context);
	}

	updateContext(context: ControllerContext): void {
		const idsChanged =
			this.conversationId !== context.conversationId || this.eventId !== context.eventId;
		this.conversationId = context.conversationId;
		this.eventId = context.eventId;
		this.recordings = context.recordings;
		this.mode = context.mode;
		this.onComplete = context.onComplete;
		if (idsChanged && this.phase === 'idle') void this.loadLiveRecordings();
	}

	get hasActiveLiveRecording(): boolean {
		return (
			this.activeLiveRecordingId !== null &&
			this.liveRecordings.some((recording) => recording.id === this.activeLiveRecordingId)
		);
	}

	get isParticipantMode(): boolean {
		return this.mode === 'participant';
	}

	get currentLiveRecording(): LiveAudioRecordingDto | null {
		return this.liveRecordings.at(0) ?? null;
	}

	get canStartNewRecording(): boolean {
		return (
			!this.hasActiveLiveRecording &&
			(!this.isParticipantMode || this.liveRecordings.length === 0)
		);
	}

	get audioVolume(): number {
		return this.engine.audioVolume;
	}

	liveRecordingName(liveRecording: LiveAudioRecordingDto): string {
		return (
			this.recordings.find((recording) => recording.id === liveRecording.audioRecordingId)
				?.name ?? 'Untitled live recording'
		);
	}

	async loadLiveRecordings(): Promise<void> {
		const result = await tryCatchAsync(() => this.api.loadLiveRecordings());
		if (result.err !== null) {
			notifications.send({ message: 'Failed to load live recordings', priority: 'ERROR' });
			return;
		}
		this.liveRecordings = result.ok;
	}

	async startRecording(): Promise<void> {
		if (!this.canStartNewRecording) {
			notifications.send({
				message: 'Resume or finalise your recording first',
				priority: 'WARNING'
			});
			return;
		}
		const trimmedName = this.recordingName.trim();
		if (!trimmedName) {
			notifications.send({
				message: 'Enter a name for the recording first',
				priority: 'ERROR'
			});
			return;
		}

		if (this.recordings.some((recording) => recording.name === trimmedName)) {
			notifications.send({
				message: `A recording named "${trimmedName}" already exists`,
				priority: 'ERROR'
			});
			return;
		}

		this.phase = 'starting';
		const createResult = await tryCatchAsync(() => this.api.createLiveRecording(trimmedName));
		if (createResult.err !== null) {
			notifications.send({
				message: getErrorMessage(createResult.err, 'Failed to start recording'),
				priority: 'ERROR'
			});
			this.phase = 'idle';
			return;
		}

		const liveRecordingId = createResult.ok.liveAudioRecording.id;
		this.activeLiveRecordingId = liveRecordingId;
		this.liveRecordings = [...this.liveRecordings, createResult.ok.liveAudioRecording];
		void invalidateAll();
		this.engine.prepareUploadStateForRecording(
			liveRecordingId,
			createResult.ok.liveAudioRecording.nextPartNumber
		);
		this.engine.connect();

		const acquireResult = await tryCatchAsync(() =>
			this.engine.acquireRecordingLock(liveRecordingId)
		);
		if (acquireResult.err !== null) {
			await this.abortCreatedRecording(
				liveRecordingId,
				'Failed to start recording session',
				acquireResult.err
			);
			return;
		}

		const microphone = await this.engine.prepareMicrophone();
		if (!microphone) {
			await this.abortCreatedRecording(liveRecordingId, 'Microphone access denied');
			return;
		}

		this.engine.startRecorder(microphone, liveRecordingId);
		this.recordingName = '';
		this.phase = 'recording';
	}

	async resumeLiveRecording(liveRecordingId: string): Promise<void> {
		if (this.phase !== 'idle') {
			notifications.send({
				message: 'Pause or stop the current recording before resuming another one',
				priority: 'WARNING'
			});
			return;
		}
		const liveRecording = this.liveRecordings.find(
			(recording) => recording.id === liveRecordingId
		);
		if (!liveRecording) {
			notifications.send({
				message: 'Could not find that live recording',
				priority: 'ERROR'
			});
			return;
		}

		this.phase = 'starting';
		this.activeLiveRecordingId = liveRecording.id;
		this.engine.prepareUploadStateForRecording(liveRecording.id, liveRecording.nextPartNumber);
		this.engine.connect();

		const disconnectResult = await tryCatchAsync(() =>
			this.engine.disconnectSessionForRecording(liveRecording.id)
		);
		if (disconnectResult.err !== null) {
			notifications.send({
				message: getErrorMessage(
					disconnectResult.err,
					'Failed to disconnect existing recording sessions before resume'
				),
				priority: 'WARNING'
			});
		}

		const acquireResult = await tryCatchAsync(() =>
			this.engine.acquireRecordingLock(liveRecording.id)
		);
		if (acquireResult.err !== null) {
			this.activeLiveRecordingId = null;
			this.phase = 'idle';
			notifications.send({
				message: getErrorMessage(acquireResult.err, 'Failed to resume recording session'),
				priority: 'ERROR'
			});
			this.engine.disconnect();
			return;
		}

		const microphone = await this.engine.prepareMicrophone();
		if (!microphone) {
			await this.engine.releaseRecordingLockBestEffort();
			this.engine.disconnect();
			this.activeLiveRecordingId = null;
			this.phase = 'idle';
			return;
		}

		this.engine.startRecorder(microphone, liveRecording.id);
		this.phase = 'recording';
	}

	async discardLiveRecording(liveRecordingId: string): Promise<void> {
		if (this.activeLiveRecordingId === liveRecordingId) {
			notifications.send({
				message: 'Stop or pause this recording before discarding it',
				priority: 'WARNING'
			});
			return;
		}
		if (!window.confirm('Delete this recording? Uploaded parts will be permanently removed.'))
			return;
		this.engine.connect();
		const result = await tryCatchAsync(() => this.engine.deleteLiveRecording(liveRecordingId));
		if (result.err !== null) {
			notifications.send({
				message: getErrorMessage(result.err, 'Failed to discard live recording'),
				priority: 'ERROR'
			});
			return;
		}
		if (this.activeLiveRecordingId === liveRecordingId) {
			this.engine.disconnect();
			this.activeLiveRecordingId = null;
		}
		this.liveRecordings = this.liveRecordings.filter(
			(recording) => recording.id !== liveRecordingId
		);
		void invalidateAll();
	}

	async pauseRecording(): Promise<void> {
		const liveRecordingId = this.activeLiveRecordingId;
		if (this.phase !== 'recording' || !liveRecordingId) return;
		this.phase = 'stopping';
		const pauseResult = await tryCatchAsync(() =>
			this.engine.drainAndStop(liveRecordingId, 'pause')
		);
		if (pauseResult.err === null) await this.engine.releaseRecordingLockBestEffort();
		this.engine.disconnect();
		if (pauseResult.err !== null) {
			notifications.send({
				message: getErrorMessage(pauseResult.err, 'Failed to pause recording'),
				priority: 'ERROR'
			});
			this.engine.destroy();
			this.activeLiveRecordingId = null;
			this.phase = 'idle';
			return;
		}
		this.activeLiveRecordingId = null;
		this.phase = 'idle';
	}

	async stopRecording(): Promise<void> {
		const liveRecordingId = this.activeLiveRecordingId;
		if (!liveRecordingId) return;
		this.phase = 'stopping';
		const stopResult = await tryCatchAsync(() =>
			this.engine.drainAndStop(liveRecordingId, 'stop')
		);
		if (stopResult.err === null) await this.engine.releaseRecordingLockBestEffort();
		this.engine.disconnect();
		if (stopResult.err !== null) {
			notifications.send({
				message: getErrorMessage(stopResult.err, 'Failed to flush recording data'),
				priority: 'ERROR'
			});
			this.engine.destroy();
			this.activeLiveRecordingId = null;
			this.phase = 'idle';
			return;
		}

		if (stopResult.ok.strategy === 'regular_upload_fallback') {
			const liveRecording = this.liveRecordings.find(
				(recording) => recording.id === liveRecordingId
			);
			this.activeLiveRecordingId = null;
			this.phase = 'idle';
			await this.finaliseWithRegularFallback(
				liveRecordingId,
				liveRecording?.audioRecordingId ?? null,
				stopResult.ok.bufferedBlob
			);
			return;
		}

		const completeResult = await tryCatchAsync(() =>
			this.engine.completeLiveRecording(liveRecordingId)
		);
		this.activeLiveRecordingId = null;
		this.phase = 'idle';
		this.liveRecordings = this.liveRecordings.filter(
			(recording) => recording.id !== liveRecordingId
		);
		if (completeResult.err !== null) {
			notifications.send({
				message: getErrorMessage(completeResult.err, 'Failed to finalise recording'),
				priority: 'ERROR'
			});
			return;
		}
		this.engine.clearBufferedAudioState(liveRecordingId);
		notifications.send({
			message: completeResult.ok.message || 'Recording saved — transcription started',
			priority: 'SUCCESS'
		});
		await this.onComplete?.();
		void invalidateAll();
	}

	async finaliseExistingRecording(liveRecordingId: string): Promise<void> {
		if (this.phase !== 'idle' || this.hasActiveLiveRecording) {
			notifications.send({
				message: 'Pause or stop the current recording first',
				priority: 'WARNING'
			});
			return;
		}
		const liveRecording = this.liveRecordings.find(
			(recording) => recording.id === liveRecordingId
		);
		if (!liveRecording) {
			notifications.send({
				message: 'Could not find that live recording',
				priority: 'ERROR'
			});
			return;
		}

		if (liveRecording.uploadedParts.length === 0) {
			this.finalisingLiveRecordingId = liveRecordingId;
			const bufferedBlob = this.engine.consumeBufferedAudioForRecording(liveRecordingId);
			await this.finaliseWithRegularFallback(
				liveRecordingId,
				liveRecording.audioRecordingId,
				bufferedBlob
			);
			this.finalisingLiveRecordingId = null;
			return;
		}

		this.finalisingLiveRecordingId = liveRecordingId;
		this.engine.connect();
		const disconnectResult = await tryCatchAsync(() =>
			this.engine.disconnectSessionForRecording(liveRecordingId)
		);
		if (disconnectResult.err !== null) {
			notifications.send({
				message: getErrorMessage(
					disconnectResult.err,
					'Failed to disconnect existing recording sessions before finalising'
				),
				priority: 'WARNING'
			});
		}

		const acquireResult = await tryCatchAsync(() =>
			this.engine.acquireRecordingLock(liveRecordingId)
		);
		if (acquireResult.err !== null) {
			this.finalisingLiveRecordingId = null;
			notifications.send({
				message: getErrorMessage(acquireResult.err, 'Failed to acquire recording lock'),
				priority: 'ERROR'
			});
			return;
		}

		const completeResult = await tryCatchAsync(() =>
			this.engine.completeLiveRecording(liveRecordingId)
		);
		this.finalisingLiveRecordingId = null;
		if (completeResult.err !== null) {
			await this.engine.releaseRecordingLockBestEffort();
			notifications.send({
				message: getErrorMessage(completeResult.err, 'Failed to finalise recording'),
				priority: 'ERROR'
			});
			return;
		}
		this.engine.clearBufferedAudioState(liveRecordingId);
		this.liveRecordings = this.liveRecordings.filter(
			(recording) => recording.id !== liveRecordingId
		);
		notifications.send({
			message: completeResult.ok.message || 'Recording saved — transcription started',
			priority: 'SUCCESS'
		});
		await this.onComplete?.();
		void invalidateAll();
	}

	destroy(): void {
		this.engine.destroy();
		if (this.activeLiveRecordingId) {
			void tryCatchAsync(() => this.engine.deleteLiveRecording(this.activeLiveRecordingId!));
		}
	}

	private updateLiveRecording(liveRecording: LiveAudioRecordingDto): void {
		this.liveRecordings = this.liveRecordings.map((recording) =>
			recording.id === liveRecording.id ? liveRecording : recording
		);
	}

	private async recoverFromMissingLiveRecording(): Promise<void> {
		this.activeLiveRecordingId = null;
		this.phase = 'idle';
		this.engine.clearBufferedAudioState();
		this.engine.disconnect();
		await this.loadLiveRecordings();
	}

	private async abortCreatedRecording(
		liveRecordingId: string,
		fallback: string,
		err?: unknown
	): Promise<void> {
		notifications.send({ message: getErrorMessage(err, fallback), priority: 'ERROR' });
		await this.engine.releaseRecordingLockBestEffort();
		const cleanupResult = await tryCatchAsync(() =>
			this.engine.deleteLiveRecording(liveRecordingId)
		);
		if (cleanupResult.err === null) void invalidateAll();
		this.engine.clearBufferedAudioState(liveRecordingId);
		this.engine.disconnect();
		this.liveRecordings = this.liveRecordings.filter(
			(recording) => recording.id !== liveRecordingId
		);
		this.activeLiveRecordingId = null;
		this.phase = 'idle';
	}

	private async finaliseWithRegularFallback(
		liveRecordingId: string,
		audioRecordingId: string | null,
		bufferedBlob: Blob | null
	): Promise<void> {
		if (!bufferedBlob || bufferedBlob.size <= 0) {
			this.engine.connect();
			const deleteResult = await tryCatchAsync(() =>
				this.engine.deleteLiveRecording(liveRecordingId)
			);
			this.engine.disconnect();
			if (deleteResult.err !== null) {
				notifications.send({
					message: getErrorMessage(
						deleteResult.err,
						'Failed to clean up empty live recording'
					),
					priority: 'ERROR'
				});
				return;
			}
			this.liveRecordings = this.liveRecordings.filter(
				(recording) => recording.id !== liveRecordingId
			);
			this.engine.clearBufferedAudioState(liveRecordingId);
			notifications.send({ message: 'No audio captured to save', priority: 'WARNING' });
			void invalidateAll();
			return;
		}

		if (!audioRecordingId) {
			notifications.send({
				message: 'Could not resolve the recording to save. Please refresh and try again.',
				priority: 'ERROR'
			});
			return;
		}

		const uploadUrlResult = await tryCatchAsync(() =>
			this.api.getRecordingUploadUrl(audioRecordingId)
		);
		if (uploadUrlResult.err !== null) {
			notifications.send({
				message: getErrorMessage(uploadUrlResult.err, 'Failed to prepare recording upload'),
				priority: 'ERROR'
			});
			return;
		}

		const uploadResult = await tryCatchAsync(() =>
			this.api.uploadBlobToSignedUrl(bufferedBlob, uploadUrlResult.ok)
		);
		if (uploadResult.err !== null) {
			notifications.send({
				message: getErrorMessage(uploadResult.err, 'Failed to upload recording audio'),
				priority: 'ERROR'
			});
			return;
		}

		const processResult = await tryCatchAsync(() =>
			this.api.processRecording(audioRecordingId)
		);
		if (processResult.err !== null) {
			notifications.send({
				message: getErrorMessage(processResult.err, 'Failed to start recording processing'),
				priority: 'ERROR'
			});
			return;
		}

		this.engine.connect();
		const deleteResult = await tryCatchAsync(() =>
			this.engine.deleteLiveRecording(liveRecordingId)
		);
		this.engine.disconnect();
		if (deleteResult.err !== null) {
			notifications.send({
				message: getErrorMessage(
					deleteResult.err,
					'Saved recording, but failed to clean up live draft'
				),
				priority: 'WARNING'
			});
		}
		this.liveRecordings = this.liveRecordings.filter(
			(recording) => recording.id !== liveRecordingId
		);

		this.engine.clearBufferedAudioState(liveRecordingId);
		notifications.send({
			message: processResult.ok.message || 'Recording saved — transcription started',
			priority: 'SUCCESS'
		});
		await this.onComplete?.();
		void invalidateAll();
	}
}
