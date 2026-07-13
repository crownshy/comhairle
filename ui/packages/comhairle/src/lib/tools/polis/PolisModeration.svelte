<script lang="ts">
	import { invalidate } from '$app/navigation';
	import { Button, LoadingButton } from '$lib/components/ui/button';
	import { Checkbox } from '$lib/components/ui/checkbox';
	import Input from '$lib/components/ui/input/input.svelte';
	import * as Card from '$lib/components/ui/card';
	import { Spinner } from '$lib/components/ui/spinner';
	import { notifications } from '$lib/notifications.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import type { PolisStatementAux } from '@crownshy/api-client/api';
	import { Check, X, RefreshCw, Search } from '@lucide/svelte';
	import AddSeedStatementsDialog from './polis-moderation/AddSeedStatementsDialog.svelte';

	let {
		workflowStepId,
		statements: initialStatements
	}: {
		workflowStepId: string;
		statements: PolisStatementAux[];
	} = $props();

	// Local mutable copy so optimistic accept/reject re-renders without a refetch.
	// Reset whenever the load re-runs (sync/seed invalidate `polis:statement-aux`).
	let statements = $state<PolisStatementAux[]>(initialStatements);
	$effect(() => {
		statements = initialStatements;
	});

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
	const selectedCount = $derived(selectedVisible.length);
	const allVisibleSelected = $derived(visible.length > 0 && selectedCount === visible.length);
	const someVisibleSelected = $derived(selectedCount > 0 && !allVisibleSelected);

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

	// Left accent bar colour keyed on seed/status. Olive-green primary stands in
	// for "accepted"; there is no dedicated success token in the theme.
	function accentFor(row: PolisStatementAux): string {
		if (row.is_seed) return 'bg-accent';
		if (row.moderation_status === 'accepted') return 'bg-primary';
		if (row.moderation_status === 'rejected') return 'bg-destructive';
		return 'bg-muted-foreground/40';
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
	<Card.Root class="gap-0 overflow-hidden py-0">
		<!-- The header row doubles as the bulk-actions bar when rows are selected.
		     Keeping a fixed height means the card (and the list below) never shifts
		     when a selection starts or clears. -->
		<div class="flex min-h-[3.5rem] items-center border-b px-4">
			{#if selectedCount > 0}
				<div class="flex w-full items-center gap-3">
					<Checkbox
						checked={allVisibleSelected}
						indeterminate={someVisibleSelected}
						onCheckedChange={(v) => toggleSelectAll(v === true)}
						aria-label="Select all statements"
					/>
					<span class="text-sm font-medium">{selectedCount} selected</span>
					<div class="ml-auto flex items-center gap-2">
						<LoadingButton
							size="sm"
							loading={bulkAction === 'accepted'}
							disabled={bulkWorking}
							onclick={() => bulkModerate('accepted')}
						>
							<Check class="size-4" />
							Approve
						</LoadingButton>
						<LoadingButton
							size="sm"
							variant="destructive"
							loading={bulkAction === 'rejected'}
							disabled={bulkWorking}
							onclick={() => bulkModerate('rejected')}
						>
							<X class="size-4" />
							Reject
						</LoadingButton>
						<Button
							size="sm"
							variant="ghost"
							disabled={bulkWorking}
							onclick={clearSelection}
						>
							Clear
						</Button>
					</div>
				</div>
			{:else}
				<div
					class="text-muted-foreground grid w-full grid-cols-[2.5rem_3rem_minmax(0,1fr)_auto] items-center gap-4 text-xs font-semibold uppercase"
				>
					<div class="flex items-center">
						<Checkbox
							checked={allVisibleSelected}
							indeterminate={someVisibleSelected}
							onCheckedChange={(v) => toggleSelectAll(v === true)}
							aria-label="Select all statements"
						/>
					</div>
					<div>#</div>
					<div>Statement</div>
					<div class="pr-2">Action</div>
				</div>
			{/if}
		</div>

		<div class="relative">
			<!-- Dim + block the rows while a bulk moderation is in flight. -->
			{#if bulkWorking}
				<div
					class="bg-background/50 absolute inset-0 z-10 flex items-center justify-center backdrop-blur-[1px]"
				>
					<Spinner class="text-muted-foreground size-6" />
				</div>
			{/if}

			{#if visible.length === 0}
				<p class="text-muted-foreground px-4 py-6 text-sm italic">
					No statements match this filter.
				</p>
			{:else}
				{#each visible as row (row.id)}
					<div
						role="button"
						tabindex="0"
						aria-pressed={!!selected[row.id]}
						onclick={(e) => {
							// Ignore clicks that land on the checkbox or the accept/reject controls.
							if (
								bulkWorking ||
								(e.target as HTMLElement).closest('[data-row-control]')
							)
								return;
							toggleSelect(row.id, !selected[row.id]);
						}}
						onkeydown={(e) => {
							if (e.key === 'Enter' || e.key === ' ') {
								e.preventDefault();
								if (!bulkWorking) toggleSelect(row.id, !selected[row.id]);
							}
						}}
						class={`border-border group relative grid cursor-pointer grid-cols-[2.5rem_3rem_minmax(0,1fr)_auto] items-center gap-4 border-b py-4 pl-4 transition-colors last:border-b-0 ${
							selected[row.id] ? 'bg-primary/5' : 'hover:bg-muted/40'
						}`}
					>
						<!-- Left accent bar (status colour) -->
						<span
							class={`absolute top-0 bottom-0 left-0 w-1 transition-all group-hover:w-1.5 ${accentFor(row)}`}
						></span>

						<!-- Select -->
						<div class="flex items-center" data-row-control>
							<Checkbox
								checked={!!selected[row.id]}
								disabled={bulkWorking}
								onCheckedChange={(v) => toggleSelect(row.id, v === true)}
								aria-label="Select statement"
							/>
						</div>

						<!-- # -->
						<div class="text-muted-foreground text-center text-xs tabular-nums">
							{row.polis_statement_id}
						</div>

						<!-- Statement text -->
						<p class="min-w-0 text-base leading-7">{row.statement_text}</p>

						<!-- Actions -->
						<div class="flex items-center gap-1 self-center pr-2" data-row-control>
							<button
								type="button"
								disabled={pending[row.id] ||
									bulkWorking ||
									row.moderation_status === 'accepted'}
								onclick={() => setStatus(row, 'accepted')}
								title="Accept"
								class="text-primary hover:bg-primary/15 inline-flex size-11 cursor-pointer items-center justify-center rounded-full transition-all hover:scale-110 active:scale-95 disabled:cursor-not-allowed disabled:opacity-40 disabled:hover:scale-100 disabled:hover:bg-transparent"
							>
								<Check class="size-6" />
							</button>
							<button
								type="button"
								disabled={pending[row.id] ||
									bulkWorking ||
									row.moderation_status === 'rejected'}
								onclick={() => setStatus(row, 'rejected')}
								title="Reject"
								class="text-destructive hover:bg-destructive/15 inline-flex size-11 cursor-pointer items-center justify-center rounded-full transition-all hover:scale-110 active:scale-95 disabled:cursor-not-allowed disabled:opacity-40 disabled:hover:scale-100 disabled:hover:bg-transparent"
							>
								<X class="size-6" />
							</button>
						</div>
					</div>
				{/each}
			{/if}
		</div>
	</Card.Root>
</div>
