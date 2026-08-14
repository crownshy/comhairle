<script lang="ts">
	import { Button, LoadingButton } from '$lib/components/ui/button';
	import { Checkbox } from '$lib/components/ui/checkbox';
	import * as Card from '$lib/components/ui/card';
	import { Spinner } from '$lib/components/ui/spinner';
	import { Check, X } from '@lucide/svelte';
	import type { PolisStatementAux } from '@crownshy/api-client/api';
	import StatementModerationRow from './StatementModerationRow.svelte';

	type Props = {
		/** The visible (filtered + searched) statements, already ordered. */
		rows: PolisStatementAux[];
		/** Selection + per-row in-flight state, keyed by aux row id. */
		selected: Record<string, boolean>;
		pending: Record<string, boolean>;
		/** Which bulk action is in flight, or null when idle. */
		bulkAction: 'accepted' | 'rejected' | null;
		onToggleSelect: (id: string, checked: boolean) => void;
		onToggleAll: (checked: boolean) => void;
		onClear: () => void;
		onBulkModerate: (status: 'accepted' | 'rejected') => void;
		onModerate: (row: PolisStatementAux, status: 'accepted' | 'rejected') => void;
		/** Per-row lineage strings for derived statements, keyed by aux row id. */
		lineage: Record<string, { editedFrom?: string; replacedBy?: string[] }>;
		onSplit: (row: PolisStatementAux) => void;
	};

	let {
		rows,
		selected,
		pending,
		bulkAction,
		onToggleSelect,
		onToggleAll,
		onClear,
		onBulkModerate,
		onModerate,
		lineage,
		onSplit
	}: Props = $props();

	const bulkWorking = $derived(bulkAction !== null);

	// Header select-all + bulk-bar state derived from the visible rows.
	const selectedCount = $derived(rows.filter((r) => selected[r.id]).length);
	const allSelected = $derived(rows.length > 0 && selectedCount === rows.length);
	const someSelected = $derived(selectedCount > 0 && !allSelected);
</script>

<Card.Root class="gap-0 overflow-hidden py-0">
	<!-- The header row doubles as the bulk-actions bar when rows are selected.
	     Keeping a fixed height means the card (and the list below) never shifts
	     when a selection starts or clears. -->
	<div class="flex min-h-[3.5rem] items-center border-b px-4">
		{#if selectedCount > 0}
			<div class="flex w-full items-center gap-3">
				<Checkbox
					checked={allSelected}
					indeterminate={someSelected}
					onCheckedChange={(v) => onToggleAll(v === true)}
					aria-label="Select all statements"
				/>
				<span class="text-sm font-medium">{selectedCount} selected</span>
				<div class="ml-auto flex items-center gap-2">
					<LoadingButton
						size="sm"
						loading={bulkAction === 'accepted'}
						disabled={bulkWorking}
						onclick={() => onBulkModerate('accepted')}
					>
						<Check class="size-4" />
						Approve
					</LoadingButton>
					<LoadingButton
						size="sm"
						variant="destructive"
						loading={bulkAction === 'rejected'}
						disabled={bulkWorking}
						onclick={() => onBulkModerate('rejected')}
					>
						<X class="size-4" />
						Reject
					</LoadingButton>
					<Button size="sm" variant="ghost" disabled={bulkWorking} onclick={onClear}>
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
						checked={allSelected}
						indeterminate={someSelected}
						onCheckedChange={(v) => onToggleAll(v === true)}
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

		{#if rows.length === 0}
			<p class="text-muted-foreground px-4 py-6 text-sm italic">
				No statements match this filter.
			</p>
		{:else}
			{#each rows as row (row.id)}
				<StatementModerationRow
					{row}
					selected={!!selected[row.id]}
					pending={!!pending[row.id]}
					{bulkWorking}
					editedFrom={lineage[row.id]?.editedFrom}
					replacedBy={lineage[row.id]?.replacedBy}
					onToggle={(checked) => onToggleSelect(row.id, checked)}
					onModerate={(status) => onModerate(row, status)}
					onSplit={() => onSplit(row)}
				/>
			{/each}
		{/if}
	</div>
</Card.Root>
