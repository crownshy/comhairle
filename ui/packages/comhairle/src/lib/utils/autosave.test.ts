import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { notifications } from '$lib/notifications.svelte';
import { Autosave } from './autosave.svelte';

vi.mock('$lib/notifications.svelte', () => ({ notifications: { send: vi.fn() } }));

/** A save the test finishes by hand. `calls` counts how many saves have started. */
function manualSave() {
	const finishers: { resolve: () => void; reject: () => void }[] = [];
	const save = vi.fn(
		() =>
			new Promise<void>((resolve, reject) => {
				finishers.push({ resolve, reject: () => reject(new Error('save failed')) });
			})
	);
	return { save, finishers };
}

describe('Autosave', () => {
	beforeEach(() => {
		vi.useFakeTimers();
	});

	afterEach(() => {
		vi.useRealTimers();
	});

	it('waits for typing to stop, then saves once', async () => {
		const { save, finishers } = manualSave();
		const autosave = new Autosave(save);

		autosave.schedule();
		await vi.advanceTimersByTimeAsync(400);
		autosave.schedule();
		await vi.advanceTimersByTimeAsync(700);
		expect(save).toHaveBeenCalledTimes(1);
		expect(autosave.status).toBe('saving');

		finishers[0].resolve();
		await vi.advanceTimersByTimeAsync(500);
		expect(autosave.status).toBe('saved');
		expect(autosave.dirty).toBe(false);
		expect(autosave.hasSaved).toBe(true);
	});

	it('starts the next save only after the one in flight finishes', async () => {
		const { save, finishers } = manualSave();
		const autosave = new Autosave(save);

		autosave.schedule();
		await vi.advanceTimersByTimeAsync(700);
		autosave.schedule();
		await vi.advanceTimersByTimeAsync(700);
		expect(save).toHaveBeenCalledTimes(1);

		finishers[0].resolve();
		await vi.advanceTimersByTimeAsync(500);
		expect(save).toHaveBeenCalledTimes(2);
	});

	it('stays dirty when an edit is made while a save is out', async () => {
		const { save, finishers } = manualSave();
		const autosave = new Autosave(save);

		autosave.schedule();
		await vi.advanceTimersByTimeAsync(700);
		autosave.schedule();
		finishers[0].resolve();
		await vi.advanceTimersByTimeAsync(500);

		expect(autosave.dirty).toBe(true);
		expect(autosave.status).toBe('saving');
	});

	it('flushes a scheduled edit straight away and settles once it lands', async () => {
		const { save, finishers } = manualSave();
		const autosave = new Autosave(save);
		const settled = vi.fn();

		autosave.schedule();
		autosave.flush();
		await vi.advanceTimersByTimeAsync(0);
		expect(save).toHaveBeenCalledTimes(1);

		autosave.settled().then(settled);
		await vi.advanceTimersByTimeAsync(0);
		expect(settled).not.toHaveBeenCalled();

		finishers[0].resolve();
		await vi.advanceTimersByTimeAsync(500);
		expect(settled).toHaveBeenCalled();
	});

	it('does nothing on flush when no edit is waiting', async () => {
		const { save } = manualSave();
		const autosave = new Autosave(save);

		autosave.flush();
		await vi.advanceTimersByTimeAsync(1000);
		expect(save).not.toHaveBeenCalled();
	});

	it('shows the error and stays dirty when a save fails', async () => {
		const { save, finishers } = manualSave();
		const autosave = new Autosave(save);

		autosave.schedule();
		await vi.advanceTimersByTimeAsync(700);
		finishers[0].reject();
		await vi.advanceTimersByTimeAsync(0);

		expect(autosave.status).toBe('error');
		expect(autosave.dirty).toBe(true);
		expect(autosave.hasSaved).toBe(false);
	});

	it('reports a failed save with a Retry that saves again', async () => {
		const { save, finishers } = manualSave();
		const autosave = new Autosave(save);

		autosave.schedule();
		await vi.advanceTimersByTimeAsync(700);
		finishers[0].reject();
		await vi.advanceTimersByTimeAsync(0);
		expect(autosave.status).toBe('error');

		const sent = vi.mocked(notifications.send).mock.calls.at(-1)?.[0];
		expect(sent?.priority).toBe('ERROR');
		const retry = sent?.action;
		if (!retry || !('onClick' in retry)) throw new Error('expected a Retry action');
		// The node test environment has no MouseEvent, and Retry doesn't read the event.
		retry.onClick(new Event('click') as Parameters<typeof retry.onClick>[0]);
		await vi.advanceTimersByTimeAsync(0);
		expect(save).toHaveBeenCalledTimes(2);

		finishers[1].resolve();
		await vi.advanceTimersByTimeAsync(500);
		expect(autosave.status).toBe('saved');
		expect(autosave.dirty).toBe(false);
	});
});
