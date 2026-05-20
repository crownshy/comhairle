import { getAdapter } from './context';
import type { Proposal, ProposalResponse, ToolConfig } from './types';

/** Reactive store. Reads/writes go through the adapter exclusively — the store knows nothing about the network. */

export type LoadState = 'idle' | 'loading' | 'ready' | 'error';

export type PrioritizationStore = ReturnType<typeof createStore>;

export function createStore() {
	const adapter = getAdapter();

	let state = $state<LoadState>('idle');
	let proposals = $state<Proposal[]>([]);
	let error = $state<string | null>(null);

	async function refresh() {
		state = 'loading';
		error = null;
		try {
			proposals = await adapter.listProposals();
			state = 'ready';
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load proposals';
			state = 'error';
		}
	}

	async function create(input: { title: string; body: string }) {
		const created = await adapter.createProposal(input);
		proposals = [...proposals, created];
		return created;
	}

	async function remove(id: string) {
		await adapter.deleteProposal(id);
		proposals = proposals.filter((p) => p.id !== id);
	}

	/** Translation edits flow through TranslatableField directly (it writes to the translations endpoints itself), so the store no longer needs an editTranslation method. Callers should refresh() if they want the list to reflect the latest server state. */

	async function loadResponses(proposalId: string): Promise<ProposalResponse[]> {
		return adapter.listResponses(proposalId);
	}

	async function saveToolConfig(toolConfig: ToolConfig) {
		await adapter.updateToolConfig(toolConfig);
		/** The wrapper rebuilds StepContext from props after the page invalidates, so no local state to update here. */
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
		loadResponses,
		saveToolConfig
	};
}
