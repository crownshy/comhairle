import { apiClient } from '@crownshy/api-client/client';
import { tryCatchAsync } from '$lib/utils/errorHandling';

const DEFAULT_POLL_INTERVAL_MS = 5000;

type JobCallbacks = {
	onComplete?: () => void | Promise<void>;
	onError?: (message: string) => void;
};

/**
 * Polls `GET /jobs/{job_id}` on a fixed interval until the job completes or
 * errors, then fires the matching callback. Call `start(jobId)` to begin a job;
 * `status`/`error`/`isWorking` stay reactive for the duration.
 *
 * Reschedules by reading the job id as a plain value each tick rather than
 * depending on `status` changing between polls: a job sits at "running" across
 * most of them, and re-assigning a $state variable the same value it already
 * holds doesn't retrigger a reactive effect, so an effect-driven reschedule would
 * stop after the first status change and never poll again.
 *
 * NOTE: Do NOT destructure the returned object - access fields via the instance
 * (e.g. `job.status`) to keep reactivity.
 */
export function useJob({ onComplete, onError }: JobCallbacks = {}) {
	let jobId = $state<string | null>(null);
	let status = $state<string | null>(null);
	let error = $state<string | null>(null);

	function start(id: string, intervalMs = DEFAULT_POLL_INTERVAL_MS) {
		jobId = id;
		status = 'pending';
		error = null;
		scheduleNext(intervalMs);
	}

	function scheduleNext(intervalMs: number) {
		if (!jobId) return;
		setTimeout(async () => {
			await poll();
			scheduleNext(intervalMs);
		}, intervalMs);
	}

	async function poll() {
		if (!jobId) return;

		const res = await tryCatchAsync(() => apiClient.GetJob({ params: { job_id: jobId! } }));

		if (res.err !== null) {
			console.error('GetJob failed', res.err);
			return;
		}

		status = res.ok.status ?? null;

		if (status === 'completed') {
			jobId = null;
			await onComplete?.();
			return;
		}

		if (status === 'error') {
			error = res.ok.error ?? 'Job failed';
			jobId = null;
			onError?.(error);
		}
	}

	return {
		get status() {
			return status;
		},
		get error() {
			return error;
		},
		get isWorking() {
			return jobId !== null && status !== 'completed' && status !== 'error';
		},
		start
	};
}
