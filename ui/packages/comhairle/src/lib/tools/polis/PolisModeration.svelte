<script lang="ts">
	import { onMount } from 'svelte';
	import { invalidate } from '$app/navigation';
	import { key } from '$lib/utils/invalidationKey';
	import { Button } from '$lib/components/ui/button';
	import Input from '$lib/components/ui/input/input.svelte';
	import * as Tabs from '$lib/components/ui/tabs';
	import { notifications } from '$lib/notifications.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { downloadCsv } from '$lib/utils/csv';
	import { splitReason, type RejectReason } from '$lib/moderation/moderationPolicy';
	import { apiClient } from '@crownshy/api-client/client';
	import type { PolisStatementAux } from '@crownshy/api-client/api';
	import { Download, Search } from '@lucide/svelte';
	import SplitStatementDialog from './polis-moderation/SplitStatementDialog.svelte';
	import StatementsPagination from './polis-moderation/StatementsPagination.svelte';
	import StatementsTable from './polis-moderation/StatementsTable.svelte';
	import { StatementSync } from './polis-moderation/statementSync';
	import { participantOriginal } from './polis-moderation/participantOriginal';
	import { buildStatementsCsv, moderationReportFilename } from './polis-moderation/statementsCsv';
	import { permissions } from '$lib/permissions.svelte';
	import { pluralise } from '$lib/utils/pluralise';

	type Props = {
		conversationId: string;
		conversationTitle: string;
		stepName: string;
		workflowStepId: string;
		statements: PolisStatementAux[];
		/** The step's moderation policy reasons (ADR-0038), offered on reject. */
		rejectReasons: RejectReason[];
		/** The API's default reasons. Export always matches them, see `reasonLabels`. */
		defaultRejectReasons: RejectReason[];
	};

	let {
		conversationId,
		conversationTitle,
		stepName,
		workflowStepId,
		statements: initialStatements,
		rejectReasons,
		defaultRejectReasons
	}: Props = $props();
	type ModerationStatus = PolisStatementAux['moderation_status'];
	const DECISIONS = { accepted: 'accept', rejected: 'reject', pending: 'pending' } as const;
	const DONE_LABELS: Record<ModerationStatus, string> = {
		accepted: 'accepted',
		rejected: 'rejected',
		pending: 'moved back to waiting'
	};

	const canModerate = $derived(
		permissions.can('conversation', 'conversation_moderate', conversationId)
	);
	const canEdit = $derived(
		permissions.can('conversation', 'conversation_update', conversationId)
	);
	const canExport = $derived(
		permissions.can('conversation', 'conversation_export', conversationId)
	);

	// Local optimistic copy so accept/reject re-renders without a refetch. A writable
	// `$derived` seeds from the prop and lets optimistic assignments below override it,
	// then resyncs on its own when the load re-runs (sync/seed invalidate
	// the Polis statements key and a fresh `initialStatements` flows back down).
	let statements = $derived(initialStatements);

	// --- Keeping the list fresh ---
	const canSync = $derived(canModerate || canEdit);
	const statementSync = new StatementSync(() => workflowStepId);

	onMount(() => (canSync ? statementSync.start() : undefined));

	// --- Status views + search ---
	// Opens on the waiting queue, since clearing it is why moderators come here.
	type Filter = 'pending' | 'accepted' | 'rejected' | 'all';
	let filter = $state<Filter>('pending');
	let search = $state('');

	const counts = $derived({
		pending: statements.filter((s) => s.moderation_status === 'pending').length,
		accepted: statements.filter((s) => s.moderation_status === 'accepted').length,
		rejected: statements.filter((s) => s.moderation_status === 'rejected').length,
		all: statements.length
	});

	const filtered = $derived.by(() => {
		let list = statements;
		if (filter !== 'all') list = list.filter((s) => s.moderation_status === filter);
		const q = search.trim().toLowerCase();
		if (q) list = list.filter((s) => s.statement_text.toLowerCase().includes(q));
		return [...list].sort((a, b) => b.polis_statement_id - a.polis_statement_id);
	});

	const LIST_STATUS_WORDS: Record<Filter, string> = {
		pending: 'waiting ',
		accepted: 'accepted ',
		rejected: 'rejected ',
		all: ''
	};
	const listHeading = $derived(
		`${filtered.length} ${LIST_STATUS_WORDS[filter]}${pluralise(filtered.length, 'statement')}`
	);

	const filters: { key: Filter; label: string }[] = [
		{ key: 'pending', label: 'Waiting' },
		{ key: 'accepted', label: 'Accepted' },
		{ key: 'rejected', label: 'Rejected' },
		{ key: 'all', label: 'All' }
	];
	const isFilter = (value: string): value is Filter => filters.some((f) => f.key === value);

	// --- Pages ---
	// Live polls run to hundreds of statements; one long list made range-select and scrolling slow.
	const PAGE_SIZE = 50;
	// Writable derived: goes back to page 1 whenever the view or search changes.
	let pageNumber = $derived.by(() => {
		void filter;
		void search;
		return 1;
	});
	// Deciding the last rows on the last page would otherwise leave an empty page.
	const pageCount = $derived(Math.max(1, Math.ceil(filtered.length / PAGE_SIZE)));
	const currentPage = $derived(Math.min(pageNumber, pageCount));
	const visible = $derived(
		filtered.slice((currentPage - 1) * PAGE_SIZE, currentPage * PAGE_SIZE)
	);

	function goToPage(page: number) {
		pageNumber = page;
		clearSelection();
	}

	// --- Download ---
	// Built from the rows already loaded, so it reflects the last sync. The default labels are
	// included so reasons recorded before the policy was edited still land in the reason column.
	const reasonLabels = $derived([
		...new Set([...rejectReasons, ...defaultRejectReasons].map((reason) => reason.label))
	]);

	function downloadStatements() {
		if (!canExport) return;
		downloadCsv(
			moderationReportFilename({ conversationTitle, stepName, date: new Date() }),
			buildStatementsCsv(statements, reasonLabels)
		);
	}

	// --- Multi-select + bulk moderation ---
	// Selection is keyed by aux row id. Select-all and the bulk actions operate
	// on the rows on the current page only.
	let selected = $state<Record<string, boolean>>({});
	const selectedVisible = $derived(visible.filter((r) => selected[r.id]));

	// The last plainly-clicked row, used as the anchor for shift-click range
	// selection. Ranges are computed against `visible` so they follow the current
	// filter/search/sort order, not the raw data order.
	let anchorId = $state<string | null>(null);

	function toggleSelect(id: string, checked: boolean, range = false) {
		if (!canModerate) return;
		// Shift-click: select every visible row between the anchor and this row
		// (inclusive). Falls back to a plain toggle if there's no anchor or it's
		// no longer visible (e.g. filtered out since it was clicked).
		if (range && anchorId !== null && anchorId !== id) {
			const order = visible.map((r) => r.id);
			const a = order.indexOf(anchorId);
			const b = order.indexOf(id);
			if (a !== -1 && b !== -1) {
				const [lo, hi] = a < b ? [a, b] : [b, a];
				const next = { ...selected };
				for (let i = lo; i <= hi; i++) next[order[i]] = true;
				selected = next;
				// Keep the anchor put so the range can be re-adjusted with another
				// shift-click, matching standard file-list behaviour.
				return;
			}
		}
		selected = { ...selected, [id]: checked };
		anchorId = id;
	}
	function toggleSelectAll(checked: boolean) {
		if (!canModerate) return;
		const next = { ...selected };
		for (const r of visible) next[r.id] = checked;
		selected = next;
	}
	function clearSelection() {
		selected = {};
		anchorId = null;
	}

	// Which bulk action is in flight (drives the per-button spinner); null when idle.
	let bulkAction = $state<ModerationStatus | null>(null);
	const bulkWorking = $derived(bulkAction !== null);

	async function bulkModerate(status: ModerationStatus, reason?: string) {
		if (!canModerate) return;
		const decision = DECISIONS[status];
		// Skip rows already in the target status.
		const targets = selectedVisible.filter((r) => r.moderation_status !== status);
		if (!targets.length || bulkWorking) return;
		bulkAction = status;

		// Optimistic: flip all targets at once, applying the shared reason (accept
		// clears it, matching the backend, ADR-0015).
		const ids = targets.map((t) => t.id);
		const idSet = new Set(ids);
		statements = statements.map((s) =>
			idSet.has(s.id)
				? { ...s, moderation_status: status, moderation_reason: reason ?? null }
				: s
		);

		// One request: the backend logs in to Polis once, moderates every id, and
		// reports per-row failures rather than failing the whole batch.
		const result = await tryCatchAsync(() =>
			apiClient.PolisModerateStatementAuxBatch({
				ids,
				decision,
				moderation_reason: reason ?? null
			})
		);

		bulkAction = null;
		clearSelection();
		if (result.err !== null) {
			notifications.send({ priority: 'ERROR', message: 'Failed to update statements' });
		} else if (result.ok.failed.length) {
			notifications.send({
				priority: 'ERROR',
				message: `${result.ok.failed.length} of ${targets.length} failed to update`
			});
		} else {
			offerUndo(targets, status, reason);
		}
		// Reconcile with server truth (also corrects any partial failures).
		await invalidate(key('admin/polis/statements'));
	}

	// --- Accept / reject ---
	// Track in-flight requests per aux row so the buttons can disable mid-call.
	let pending = $state<Record<string, boolean>>({});

	async function setStatus(row: PolisStatementAux, status: ModerationStatus, reason?: string) {
		if (!canModerate) return;
		// If the clicked row is part of an active selection, the per-row accept/reject
		// applies to the whole selection (same as the bulk bar). Clicking a row that
		// isn't selected stays a single-row action.
		if (selected[row.id]) return bulkModerate(status, reason);
		const before = await moderateRow(row, status, reason);
		if (before) offerUndo([before], status, reason);
	}

	/** Moderates one row, ignoring any selection. Returns the row as it was, or null on failure. */
	async function moderateRow(row: PolisStatementAux, status: ModerationStatus, reason?: string) {
		if (pending[row.id] || row.moderation_status === status) return null;
		const decision = DECISIONS[status];
		pending = { ...pending, [row.id]: true };

		// Optimistic update; roll back on failure. Accept clears any prior reason
		// (matches the backend, ADR-0015); reject shows the new one immediately.
		const prev = row;
		statements = statements.map((s) =>
			s.id === row.id
				? { ...s, moderation_status: status, moderation_reason: reason ?? null }
				: s
		);

		const res = await tryCatchAsync(() =>
			apiClient.PolisModerateStatementAux(
				{ decision, moderation_reason: reason ?? null },
				{ params: { id: row.id } }
			)
		);
		pending = { ...pending, [row.id]: false };

		if (res.err !== null) {
			statements = statements.map((s) => (s.id === row.id ? prev : s));
			notifications.send({ priority: 'ERROR', message: 'Failed to update statement' });
			return null;
		}
		statements = statements.map((s) => (s.id === row.id ? res.ok : s));
		// A sync that started before this decision reached the server would load the old row.
		if (statementSync.syncing) await invalidate(key('admin/polis/statements'));
		return prev;
	}

	// A slip is one click to fix, instead of an "are you sure?" before every decision.
	function offerUndo(before: PolisStatementAux[], status: ModerationStatus, reason?: string) {
		notifications.send({
			priority: 'INFO',
			message: doneMessage(before.length, status, reason),
			duration: 6000,
			action: { label: 'Undo', onClick: () => restore(before) }
		});
	}

	function doneMessage(count: number, status: ModerationStatus, reason?: string) {
		const subject = count === 1 ? 'Statement' : `${count} statements`;
		const label = splitReason(reason, reasonLabels).label;
		return label
			? `${subject} ${DONE_LABELS[status]}: ${label}`
			: `${subject} ${DONE_LABELS[status]}`;
	}

	async function restore(before: PolisStatementAux[]) {
		if (before.length === 1) {
			const current = statements.find((s) => s.id === before[0].id);
			if (current)
				await moderateRow(
					current,
					before[0].moderation_status,
					before[0].moderation_reason ?? undefined
				);
			return;
		}

		const beforeById = new Map(before.map((row) => [row.id, row]));
		statements = statements.map((s) => beforeById.get(s.id) ?? s);

		// The batch endpoint takes one decision and reason, so rows go back in groups.
		const groups: Record<string, PolisStatementAux[]> = {};
		for (const row of before) {
			(groups[`${row.moderation_status}:${row.moderation_reason ?? ''}`] ??= []).push(row);
		}
		const results = await Promise.all(
			Object.values(groups).map((rows) =>
				tryCatchAsync(() =>
					apiClient.PolisModerateStatementAuxBatch({
						ids: rows.map((row) => row.id),
						decision: DECISIONS[rows[0].moderation_status],
						moderation_reason: rows[0].moderation_reason ?? null
					})
				)
			)
		);
		if (results.some((result) => result.err !== null || result.ok.failed.length > 0)) {
			notifications.send({
				priority: 'ERROR',
				message: 'Failed to undo for some statements'
			});
		}
		await invalidate(key('admin/polis/statements'));
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
		const byId: Record<string, PolisStatementAux> = {};
		const replacementsByOriginal: Record<string, string[]> = {};
		for (const s of statements) {
			byId[s.id] = s;
			if (s.original_statement_id) {
				(replacementsByOriginal[s.original_statement_id] ??= []).push(s.statement_text);
			}
		}
		const result: Record<string, { editedFrom?: string; replacedBy?: string[] }> = {};
		for (const s of statements) {
			const editedFrom = s.original_statement_id
				? byId[s.original_statement_id]?.statement_text
				: undefined;
			const replacedBy = replacementsByOriginal[s.id];
			if (editedFrom || replacedBy) result[s.id] = { editedFrom, replacedBy };
		}
		return result;
	});

	let splitTarget = $state<PolisStatementAux | null>(null);
	let splitOpen = $state(false);

	// What the participant first wrote, when the target is already a moderator's edit.
	const splitParticipantStatement = $derived(
		splitTarget ? participantOriginal(splitTarget, statements) : null
	);

	// The statement the participant was viewing when they wrote it, if we can resolve it.
	// `visible_statement_when_submitted` stores the Polis tid, and only the participant's
	// own statement carries it.
	const splitContext = $derived.by(() => {
		const tid = splitParticipantStatement?.visible_statement_when_submitted;
		if (!tid) return undefined;
		const n = Number(tid);
		return statements.find((s) => s.polis_statement_id === n)?.statement_text;
	});

	function openSplit(row: PolisStatementAux) {
		if (!canModerate) return;
		splitTarget = row;
		splitOpen = true;
	}
</script>

<!-- pb-24 keeps the last row clear of the fixed Help button. -->
<div class="flex flex-col gap-6 pb-24">
	<div class="flex flex-wrap items-start justify-between gap-x-6 gap-y-3">
		<div class="flex min-w-0 flex-col gap-1">
			<h2 class="text-2xl font-bold">{stepName}</h2>
		</div>
		<Button
			data-tour="moderation-actions"
			variant="outline"
			disabled={!canExport || statements.length === 0}
			onclick={downloadStatements}
			title="Every statement and decision, as a CSV"
		>
			<Download class="size-4" />
			Download report
		</Button>
	</div>

	<div class="flex flex-col gap-3 md:flex-row md:items-center md:justify-between">
		<Tabs.Root
			value={filter}
			onValueChange={(value) => {
				if (isFilter(value)) filter = value;
			}}
			class="min-w-0"
		>
			<div class="-mx-1 overflow-x-auto px-1">
				<Tabs.List data-tour="moderation-filters" class="h-11">
					{#each filters as f (f.key)}
						<Tabs.Trigger value={f.key} class="gap-2 px-3 text-base">
							{f.label}
							<span class="text-muted-foreground tabular-nums">{counts[f.key]}</span>
						</Tabs.Trigger>
					{/each}
				</Tabs.List>
			</div>
		</Tabs.Root>
		<div class="relative w-full md:ml-auto md:w-72">
			<Search
				class="text-muted-foreground pointer-events-none absolute top-1/2 left-3 size-4 -translate-y-1/2"
			/>
			<Input
				bind:value={search}
				placeholder="Search statements"
				aria-label="Search statements"
				class="pl-9"
			/>
		</div>
	</div>

	<!-- Statements list -->
	<div data-tour="moderation-statements">
		<StatementsTable
			editable={canModerate}
			rows={visible}
			showStatus={filter === 'all'}
			heading={listHeading}
			{rejectReasons}
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
		>
			{#snippet empty()}
				{#if statements.length === 0}
					<p class="text-muted-foreground px-4 py-8 text-base">
						No statements yet. New statements from participants appear here as they come
						in.
					</p>
				{:else if search.trim()}
					<p class="text-muted-foreground px-4 py-8 text-base">
						No statements match "{search.trim()}".
					</p>
				{:else if filter === 'pending'}
					<p class="text-muted-foreground px-4 py-8 text-base">
						All caught up. Nothing is waiting for a decision.
					</p>
				{:else}
					<p class="text-muted-foreground px-4 py-8 text-base">No statements here yet.</p>
				{/if}
			{/snippet}
		</StatementsTable>
	</div>

	<StatementsPagination
		count={filtered.length}
		perPage={PAGE_SIZE}
		page={currentPage}
		onPageChange={goToPage}
	/>
</div>

{#if canModerate}
	<SplitStatementDialog
		{conversationId}
		bind:open={splitOpen}
		original={splitTarget}
		participantText={splitParticipantStatement?.id !== splitTarget?.id
			? splitParticipantStatement?.statement_text
			: undefined}
		viewedContext={splitContext}
		onDone={() => invalidate(key('admin/polis/statements'))}
	/>
{/if}
