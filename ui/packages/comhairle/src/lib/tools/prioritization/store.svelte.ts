import { notifications } from '$lib/notifications.svelte';
import * as api from './prioritizationApi';
import type { Proposal, ToolConfig } from './types';

/** Reactive store for the admin (Manage) view. Holds the proposal list and
 * wraps the api module — failed writes surface a toast and re-throw so callers
 * can revert their optimistic UI. */

export type LoadState = 'idle' | 'loading' | 'ready' | 'error';

export type PrioritizationStore = ReturnType<typeof createStore>;

export function createStore(opts: {
	workflowStepId: string;
	conversationId: string;
	workflowId: string;
}) {
	let state = $state<LoadState>('idle');
	let proposals = $state<Proposal[]>([]);
	let error = $state<string | null>(null);

	async function refresh() {
		state = 'loading';
		error = null;
		try {
			proposals = await api.listProposals(opts.workflowStepId);
			state = 'ready';
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load proposals';
			state = 'error';
		}
	}

	async function create(input: { title: string; body: string }) {
		const created = await api.createProposal(opts.workflowStepId, input);
		proposals = [...proposals, created];
		return created;
	}

	async function remove(id: string) {
		try {
			await api.deleteProposal(id);
			proposals = proposals.filter((p) => p.id !== id);
		} catch (e) {
			notifications.send({ priority: 'ERROR', message: 'Failed to delete proposal' });
			throw e;
		}
	}

	async function saveToolConfig(toolConfig: ToolConfig) {
		try {
			await api.updateToolConfig({
				conversationId: opts.conversationId,
				workflowId: opts.workflowId,
				workflowStepId: opts.workflowStepId,
				toolConfig
			});
		} catch (e) {
			notifications.send({ priority: 'ERROR', message: 'Failed to update tool config' });
			throw e;
		}
	}

	return {
		get state() {
			return state;
		},
		get proposals() {
			return proposals;
		},
		get error() {
			return error;
		},
		refresh,
		create,
		remove,
		saveToolConfig
	};
}
