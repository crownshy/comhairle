<script lang="ts">
	import { invalidate } from '$app/navigation';
	import { LoadingButton } from '$lib/components/ui/button';
	import Input from '$lib/components/ui/input/input.svelte';
	import { notifications } from '$lib/notifications.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { apiClient } from '@crownshy/api-client/client';
	import type { PolisStatementAux } from '@crownshy/api-client/api';
	import { RefreshCw, Search } from '@lucide/svelte';
	import AddSeedStatementsDialog from './polis-moderation/AddSeedStatementsDialog.svelte';
	import SplitStatementDialog from './polis-moderation/SplitStatementDialog.svelte';
	import StatementsTable from './polis-moderation/StatementsTable.svelte';

	let {
		workflowStepId,
		statements: initialStatements
	}: {
		workflowStepId: string;
		statements: PolisStatementAux[];
	} = $props();

	// Local optimistic copy so accept/reject re-renders without a refetch. A writable
	// `$derived` seeds from the prop and lets optimistic assignments below override it,
	// then resyncs on its own when the load re-runs (sync/seed invalidate
	// `polis:statement-aux` and a fresh `initialStatements` flows back down).
	let statements = $derived(initialStatements);

	// --- Sync from Polis ---
	// Participant submissions only appear here after a sync — aux rows are synced
	// from Polis, not created live. Existing moderation/themes are preserved.
	let syncing = $state(false);

	async function syncFromPolis() {
		if (syncing) return;
		syncing = true;

		const res = await tryCatchAsync(() =>
			apiClient.PolisSyncStatementAux({ workflow_step_id: workflowStepId })
		);
		if (res.err !== null) {
			console.error('PolisSyncStatementAux failed', res.err);
			notifications.send({
				priority: 'ERROR',
				message: 'Failed to sync statements from Polis'
			});
			syncing = false;
			return;
		}

		notifications.send({
			priority: 'INFO',
			message: `Synced ${res.ok.synced} statement${res.ok.synced === 1 ? '' : 's'} from Polis${
				res.ok.skipped_invalid_xid ? ` (${res.ok.skipped_invalid_xid} skipped)` : ''
			}`
		});
		// Keep the spinner up through the reload so the button doesn't flicker.
		await invalidate('polis:statement-aux');
		syncing = false;
	}

	// --- Status filters + search ---
	type Filter = 'all' | 'seeded' | 'accepted' | 'pending' | 'rejected';
	let filter = $state<Filter>('all');
	let search = $state('');

	const counts = $derived({
		all: statements.length,
		seeded: statements.filter((s) => s.is_seed).length,
		accepted: statements.filter((s) => s.moderation_status === 'accepted').length,
		pending: statements.filter((s) => s.moderation_status === 'pending').length,
		rejected: statements.filter((s) => s.moderation_status === 'rejected').length
	});

	const visible = $derived.by(() => {
		let list = statements;
		if (filter === 'seeded') list = list.filter((s) => s.is_seed);
		else if (filter !== 'all') list = list.filter((s) => s.moderation_status === filter);
		const q = search.trim().toLowerCase();
		if (q) list = list.filter((s) => s.statement_text.toLowerCase().includes(q));
		return [...list].sort((a, b) => b.polis_statement_id - a.polis_statement_id);
	});

	const filters: { key: Filter; label: string }[] = [
		{ key: 'all', label: 'All' },
		{ key: 'seeded', label: 'Seeded' },
		{ key: 'accepted', label: 'Accepted' },
		{ key: 'pending', label: 'Pending' },
		{ key: 'rejected', label: 'Rejected' }
	];

	// --- Multi-select + bulk moderation ---
	// Selection is keyed by aux row id. Select-all and the bulk actions operate
	// on the currently visible (filtered + searched) rows only.
	let selected = $state<Record<string, boolean>>({});
	const selectedVisible = $derived(visible.filter((r) => selected[r.id]));

	function toggleSelect(id: string, checked: boolean) {
		selected = { ...selected, [id]: checked };
	}
	function toggleSelectAll(checked: boolean) {
		const next = { ...selected };
		for (const r of visible) next[r.id] = checked;
		selected = next;
	}
	function clearSelection() {
		selected = {};
	}

	// Which bulk action is in flight (drives the per-button spinner); null when idle.
	let bulkAction = $state<'accepted' | 'rejected' | null>(null);
	const bulkWorking = $derived(bulkAction !== null);

	async function bulkModerate(status: 'accepted' | 'rejected') {
		const decision = status === 'accepted' ? 'accept' : 'reject';
		// Skip rows already in the target status.
		const targets = selectedVisible.filter((r) => r.moderation_status !== status);
		if (!targets.length || bulkWorking) return;
		bulkAction = status;

		// Optimistic: flip all targets at once.
		const ids = targets.map((t) => t.id);
		const idSet = new Set(ids);
		statements = statements.map((s) =>
			idSet.has(s.id) ? { ...s, moderation_status: status } : s
		);

		// One request: the backend logs in to Polis once, moderates every id, and
		// reports per-row failures rather than failing the whole batch.
		const result = await tryCatchAsync(() =>
			apiClient.PolisModerateStatementAuxBatch({ ids, decision })
		);

		bulkAction = null;
		clearSelection();
		if (result.err !== null) {
			console.error('Bulk moderate failed', result.err);
			notifications.send({ priority: 'ERROR', message: 'Failed to update statements' });
		} else if (result.ok.failed.length) {
			notifications.send({
				priority: 'ERROR',
				message: `${result.ok.failed.length} of ${targets.length} failed to update`
			});
		} else {
			notifications.send({
				priority: 'INFO',
				message: `${targets.length} statement${targets.length === 1 ? '' : 's'} ${status}`
			});
		}
		// Reconcile with server truth (also corrects any partial failures).
		await invalidate('polis:statement-aux');
	}

	// --- Accept / reject ---
	// Track in-flight requests per aux row so the buttons can disable mid-call.
	let pending = $state<Record<string, boolean>>({});

	async function setStatus(row: PolisStatementAux, status: 'accepted' | 'rejected') {
		if (pending[row.id] || row.moderation_status === status) return;
		const decision = status === 'accepted' ? 'accept' : 'reject';
		pending = { ...pending, [row.id]: true };

		// Optimistic update; roll back on failure.
		const prevStatus = row.moderation_status;
		statements = statements.map((s) =>
			s.id === row.id ? { ...s, moderation_status: status } : s
		);

		const res = await tryCatchAsync(() =>
			apiClient.PolisModerateStatementAux({ decision }, { params: { id: row.id } })
		);
		pending = { ...pending, [row.id]: false };

		if (res.err !== null) {
			console.error('PolisModerateStatementAux failed', res.err);
			statements = statements.map((s) =>
				s.id === row.id ? { ...s, moderation_status: prevStatus } : s
			);
			notifications.send({ priority: 'ERROR', message: 'Failed to update statement' });
			return;
		}
		statements = statements.map((s) => (s.id === row.id ? res.ok : s));
	}

	// --- Split / reword ---
	// A derived statement carries `original_statement_id`; the rejected original is
	// linked back to its replacements. Both directions are resolved here from the
	// full (unfiltered) list so the row can show "Edited from" / "Replaced by".
	const lineage = $derived.by(() => {
		// First pass builds both indexes. The result pass below has to stay
		// separate: it reads `replacementsByOriginal[s.id]`, which isn't complete
		// until every statement has been seen (a replacement may sit before or
		// after its original in the list).
		const byId = new Map<string, PolisStatementAux>();
		const replacementsByOriginal: Record<string, string[]> = {};
		for (const s of statements) {
			byId.set(s.id, s);
			if (s.original_statement_id) {
				(replacementsByOriginal[s.original_statement_id] ??= []).push(s.statement_text);
			}
		}
		const result: Record<string, { editedFrom?: string; replacedBy?: string[] }> = {};
		for (const s of statements) {
			const editedFrom = s.original_statement_id
				? byId.get(s.original_statement_id)?.statement_text
				: undefined;
			const replacedBy = replacementsByOriginal[s.id];
			if (editedFrom || replacedBy) result[s.id] = { editedFrom, replacedBy };
		}
		return result;
	});

	let splitTarget = $state<PolisStatementAux | null>(null);
	let splitOpen = $state(false);

	// The statement the participant was viewing when they submitted the target, if we
	// can resolve it. `visible_statement_when_submitted` stores the Polis tid.
	const splitContext = $derived.by(() => {
		const tid = splitTarget?.visible_statement_when_submitted;
		if (!tid) return undefined;
		const n = Number(tid);
		return statements.find((s) => s.polis_statement_id === n)?.statement_text;
	});

	function openSplit(row: PolisStatementAux) {
		splitTarget = row;
		splitOpen = true;
	}
</script>

<div class="flex flex-col gap-6 rounded-xl">
	<!-- Heading + actions -->
	<div class="flex items-start justify-between gap-4">
		<div class="flex max-w-3xl flex-col gap-1">
			<h2 class="text-2xl font-bold">Statements moderation</h2>
			<p class="text-muted-foreground text-sm">Moderate and view all statements.</p>
		</div>
		<div class="flex shrink-0 items-center gap-2">
			<LoadingButton
				loading={syncing}
				variant="outline"
				onclick={syncFromPolis}
				title="Pull the latest submitted statements from Polis"
			>
				<RefreshCw class="size-4" />
				Sync from Polis
			</LoadingButton>
			<AddSeedStatementsDialog
				{workflowStepId}
				onSeeded={() => invalidate('polis:statement-aux')}
			/>
		</div>
	</div>

	<!-- Search -->
	<div class="relative max-w-sm">
		<Search
			class="text-muted-foreground pointer-events-none absolute top-1/2 left-3 size-4 -translate-y-1/2"
		/>
		<Input bind:value={search} placeholder="Search statements…" class="pl-9" />
	</div>

	<!-- Status filter chips -->
	<div class="flex flex-wrap items-center gap-2">
		{#each filters as f (f.key)}
			<button
				type="button"
				onclick={() => (filter = f.key)}
				class={`inline-flex cursor-pointer items-center rounded-full px-3.5 py-2 text-sm font-medium transition-colors ${
					filter === f.key
						? 'bg-primary text-primary-foreground'
						: 'bg-secondary text-secondary-foreground hover:bg-secondary/80'
				}`}
			>
				{f.label} · {counts[f.key]}
			</button>
		{/each}
	</div>

	<!-- Statements list -->
	<StatementsTable
		rows={visible}
		{selected}
		{pending}
		{bulkAction}
		{lineage}
		onToggleSelect={toggleSelect}
		onToggleAll={toggleSelectAll}
		onClear={clearSelection}
		onBulkModerate={bulkModerate}
		onModerate={setStatus}
		onSplit={openSplit}
	/>
</div>

<SplitStatementDialog
	bind:open={splitOpen}
	original={splitTarget}
	viewedContext={splitContext}
	onDone={() => invalidate('polis:statement-aux')}
/>
