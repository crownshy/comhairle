import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { apiClient } from '@crownshy/api-client/client';
import { useJob } from './use-job.svelte';

vi.mock('@crownshy/api-client/client', () => ({ apiClient: { GetJob: vi.fn() } }));

describe('useJob', () => {
	beforeEach(() => {
		vi.useFakeTimers();
		vi.mocked(apiClient.GetJob).mockReset();
	});

	afterEach(() => {
		vi.useRealTimers();
	});

	it('keeps polling across repeated identical "running" statuses until completed', async () => {
		vi.mocked(apiClient.GetJob)
			.mockResolvedValueOnce({ status: 'running' })
			.mockResolvedValueOnce({ status: 'running' })
			.mockResolvedValueOnce({ status: 'running' })
			.mockResolvedValueOnce({ status: 'completed' });

		const onComplete = vi.fn();
		const job = useJob({ onComplete });

		job.start('job-1', 10);
		expect(job.status).toBe('pending');
		expect(job.isWorking).toBe(true);

		for (let i = 0; i < 4; i++) {
			await vi.advanceTimersByTimeAsync(10);
		}

		expect(apiClient.GetJob).toHaveBeenCalledTimes(4);
		expect(job.status).toBe('completed');
		expect(job.isWorking).toBe(false);
		expect(onComplete).toHaveBeenCalledOnce();

		// Nothing left to poll once completed - no more GetJob calls should show up.
		await vi.advanceTimersByTimeAsync(100);
		expect(apiClient.GetJob).toHaveBeenCalledTimes(4);
	});

	it('surfaces the error and stops polling on an error status', async () => {
		vi.mocked(apiClient.GetJob).mockResolvedValueOnce({ status: 'error', error: 'boom' });

		const onError = vi.fn();
		const job = useJob({ onError });

		job.start('job-2', 10);
		await vi.advanceTimersByTimeAsync(10);

		expect(job.status).toBe('error');
		expect(job.error).toBe('boom');
		expect(job.isWorking).toBe(false);
		expect(onError).toHaveBeenCalledWith('boom');

		await vi.advanceTimersByTimeAsync(100);
		expect(apiClient.GetJob).toHaveBeenCalledTimes(1);
	});
});
