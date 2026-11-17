import { invalidate } from '$app/navigation';
import { key } from '$lib/utils/invalidationKey';
import { notifications } from '$lib/notifications.svelte';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { apiClient } from '@crownshy/api-client/client';

/** Skips a sync on tab focus if the last one finished more recently than this. */
const FOCUS_SYNC_COOLDOWN_MS = 30_000;

/**
 * Pulls new participant statements from Polis without the moderator asking: once when the
 * page opens and again when they come back to the tab. Aux rows only exist after a sync, so a
 * moderator who forgot to press a button used to see a stale queue.
 */
export class StatementSync {
	/** True from the sync request until the statements have reloaded. */
	syncing = false;
	/** Epoch milliseconds of the last finished sync, failed or not. */
	private lastSyncedAt: number | null = null;
	private lastFailed = false;

	constructor(private workflowStepId: () => string) {}

	private async sync() {
		if (this.syncing) return;
		this.syncing = true;

		const res = await tryCatchAsync(() =>
			apiClient.PolisSyncStatementAux({ workflow_step_id: this.workflowStepId() })
		);
		if (res.err === null) await invalidate(key('admin/polis/statements'));
		// Tell the moderator once, not on every return to the tab while Polis stays down.
		else if (!this.lastFailed) {
			notifications.send({
				priority: 'ERROR',
				message: 'Couldn’t check Polis for new statements'
			});
		}

		this.lastFailed = res.err !== null;
		this.lastSyncedAt = Date.now();
		this.syncing = false;
	}

	/** Syncs now and on every return to the tab. Returns the cleanup for `onMount`. */
	start(): () => void {
		this.sync();
		const onVisible = () => {
			if (document.visibilityState !== 'visible') return;
			const sinceLast = this.lastSyncedAt ? Date.now() - this.lastSyncedAt : Infinity;
			if (sinceLast > FOCUS_SYNC_COOLDOWN_MS) this.sync();
		};
		document.addEventListener('visibilitychange', onVisible);
		return () => document.removeEventListener('visibilitychange', onVisible);
	}
}
