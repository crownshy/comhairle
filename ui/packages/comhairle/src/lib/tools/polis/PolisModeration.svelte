<script lang="ts">
	import { invalidate } from '$app/navigation';
	import { LoadingButton } from '$lib/components/ui/button';
	import Input from '$lib/components/ui/input/input.svelte';
	import { notifications } from '$lib/notifications.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import type { PolisStatementAux } from '@crownshy/api-client/api';
	import { RefreshCw, Search } from '@lucide/svelte';
	import AddSeedStatementsDialog from './polis-moderation/AddSeedStatementsDialog.svelte';
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
		try {
			const res = await apiClient.PolisSyncStatementAux({ workflow_step_id: workflowStepId });
			notifications.send({
				priority: 'INFO',
				message: `Synced ${res.synced} statement${res.synced === 1 ? '' : 's'} from Polis${
					res.skipped_invalid_xid ? ` (${res.skipped_invalid_xid} skipped)` : ''
				}`
			});
			await invalidate('polis:statement-aux');
		} catch (e) {
			console.error('PolisSyncStatementAux failed', e);
			notifications.send({
				priority: 'ERROR',
				message: 'Failed to sync statements from Polis'
			});
		} finally {
			syncing = false;
		}
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
		const ids = new Set(targets.map((t) => t.id));
		statements = statements.map((s) =>
			ids.has(s.id) ? { ...s, moderation_status: status } : s
		);

		const results = await Promise.allSettled(
			targets.map((t) =>
				apiClient.PolisModerateStatementAux({ decision }, { params: { id: t.id } })
			)
		);
		const failed = results.filter((r) => r.status === 'rejected').length;

		bulkAction = null;
		clearSelection();
		if (failed) {
			console.error(`Bulk moderate: ${failed}/${targets.length} failed`);
			notifications.send({
				priority: 'ERROR',
				message: `${failed} of ${targets.length} failed to update`
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

		try {
			const updated = await apiClient.PolisModerateStatementAux(
				{ decision },
				{ params: { id: row.id } }
			);
			statements = statements.map((s) => (s.id === row.id ? updated : s));
		} catch (e) {
			console.error('PolisModerateStatementAux failed', e);
			statements = statements.map((s) =>
				s.id === row.id ? { ...s, moderation_status: prevStatus } : s
			);
			notifications.send({ priority: 'ERROR', message: 'Failed to update statement' });
		} finally {
			pending = { ...pending, [row.id]: false };
		}
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
		onToggleSelect={toggleSelect}
		onToggleAll={toggleSelectAll}
		onClear={clearSelection}
		onBulkModerate={bulkModerate}
		onModerate={setStatus}
	/>
</div>
