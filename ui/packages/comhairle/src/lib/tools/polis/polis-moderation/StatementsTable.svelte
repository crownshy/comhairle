<script lang="ts">
	import { Button, LoadingButton } from '$lib/components/ui/button';
	import { Checkbox } from '$lib/components/ui/checkbox';
	import * as Card from '$lib/components/ui/card';
	import { Spinner } from '$lib/components/ui/spinner';
	import Check from '@lucide/svelte/icons/check';
	import Undo2 from '@lucide/svelte/icons/undo-2';
	import X from '@lucide/svelte/icons/x';
	import type { PolisStatementAux, PolisStatementTranslation } from '@crownshy/api-client/api';
	import type { RejectReason } from '$lib/moderation/moderationPolicy';
	import StatementModerationRow from './StatementModerationRow.svelte';
	import RejectDialog from './RejectDialog.svelte';
	import type { Snippet } from 'svelte';

	type ModerationStatus = PolisStatementAux['moderation_status'];

	type Props = {
		editable?: boolean;
		/** The visible (filtered + searched) statements, already ordered. */
		rows: PolisStatementAux[];
		/** Show each row's status, for views that mix statuses. */
		showStatus: boolean;
		/** Names the list in the header, e.g. "10 accepted statements". */
		heading: string;
		/** The conversation's moderation policy reasons, offered on reject. */
		rejectReasons: RejectReason[];
		/** Selection + per-row in-flight state, keyed by aux row id. */
		selected: Record<string, boolean>;
		pending: Record<string, boolean>;
		/** Which bulk action is in flight, or null when idle. */
		bulkAction: ModerationStatus | null;
		onToggleSelect: (id: string, checked: boolean, range: boolean) => void;
		onToggleAll: (checked: boolean) => void;
		onClear: () => void;
		onBulkModerate: (status: ModerationStatus, reason?: string) => void;
		onModerate: (row: PolisStatementAux, status: ModerationStatus, reason?: string) => void;
		/** Per-row lineage strings for derived statements, keyed by aux row id. */
		lineage: Record<string, { editedFrom?: string; replacedBy?: string[] }>;
		/** Stored translations, keyed by aux row id. */
		translationsByStatement: Record<string, PolisStatementTranslation[]>;
		onTranslationChange: (translation: PolisStatementTranslation) => void;
		onSplit: (row: PolisStatementAux) => void;
		onUndoSplit: (row: PolisStatementAux) => void;
		/** Shown in place of the rows when there are none to show. */
		empty: Snippet;
	};

	let {
		editable = true,
		rows,
		showStatus,
		heading,
		rejectReasons,
		selected,
		pending,
		bulkAction,
		onToggleSelect,
		onToggleAll,
		onClear,
		onBulkModerate,
		onModerate,
		lineage,
		translationsByStatement,
		onTranslationChange,
		onSplit,
		onUndoSplit,
		empty
	}: Props = $props();

	const bulkWorking = $derived(bulkAction !== null);

	// Header select-all + bulk-bar state derived from the visible rows.
	const selectedCount = $derived(rows.filter((r) => selected[r.id]).length);
	const allSelected = $derived(rows.length > 0 && selectedCount === rows.length);
	const someSelected = $derived(selectedCount > 0 && !allSelected);
	const anySelectedDecided = $derived(
		rows.some((r) => selected[r.id] && r.moderation_status !== 'pending')
	);
</script>

<Card.Root class="@container gap-0 overflow-hidden py-0">
	<!-- The header row doubles as the bulk-actions bar when rows are selected.
	     Keeping a fixed height means the card (and the list below) never shifts
	     when a selection starts or clears. -->
	<div class="flex min-h-[3.5rem] items-center border-b px-4">
		{#if editable && selectedCount > 0}
			<div class="flex w-full flex-wrap items-center gap-3 py-2">
				<Checkbox
					checked={allSelected}
					indeterminate={someSelected}
					onCheckedChange={(v) => onToggleAll(v === true)}
					aria-label="Select all statements"
					class="border-muted-foreground"
				/>
				<span class="text-base font-medium">{selectedCount} selected</span>
				<div class="ml-auto flex flex-wrap items-center gap-2">
					<LoadingButton
						size="sm"
						loading={bulkAction === 'accepted'}
						disabled={bulkWorking}
						onclick={() => onBulkModerate('accepted')}
					>
						<Check class="size-4" />
						Accept
					</LoadingButton>
					<RejectDialog
						reasons={rejectReasons}
						heading={`Reject ${selectedCount} statement${selectedCount === 1 ? '' : 's'}`}
						disabled={bulkWorking}
						onConfirm={(reason) => onBulkModerate('rejected', reason)}
					>
						{#snippet trigger({ props })}
							<LoadingButton
								{...props}
								size="sm"
								variant="destructive"
								loading={bulkAction === 'rejected'}
								disabled={bulkWorking}
							>
								<X class="size-4" />
								Reject
							</LoadingButton>
						{/snippet}
					</RejectDialog>
					{#if anySelectedDecided}
						<LoadingButton
							size="sm"
							variant="outline"
							loading={bulkAction === 'pending'}
							disabled={bulkWorking}
							onclick={() => onBulkModerate('pending')}
						>
							<Undo2 class="size-4" />
							Move back to waiting
						</LoadingButton>
					{/if}
					<Button size="sm" variant="ghost" disabled={bulkWorking} onclick={onClear}>
						Clear
					</Button>
				</div>
			</div>
		{:else}
			<div
				class="text-muted-foreground grid w-full grid-cols-[2.5rem_minmax(0,1fr)_auto] items-center gap-4 text-base font-medium"
			>
				<div class="flex items-center">
					{#if editable}
						<Checkbox
							checked={allSelected}
							indeterminate={someSelected}
							onCheckedChange={(v) => onToggleAll(v === true)}
							aria-label="Select all statements"
							class="border-muted-foreground"
						/>
					{/if}
				</div>
				<div>{heading}</div>
				<div class="pr-2">{editable ? '' : 'Status'}</div>
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
			{@render empty()}
		{:else}
			{#each rows as row (row.id)}
				<StatementModerationRow
					{editable}
					{row}
					{showStatus}
					{rejectReasons}
					selected={!!selected[row.id]}
					selectionCount={selectedCount}
					pending={!!pending[row.id]}
					{bulkWorking}
					editedFrom={lineage[row.id]?.editedFrom}
					replacedBy={lineage[row.id]?.replacedBy}
					translations={translationsByStatement[row.id] ?? []}
					{onTranslationChange}
					onToggle={(checked, range) => onToggleSelect(row.id, checked, range)}
					onModerate={(status, reason) => onModerate(row, status, reason)}
					onSplit={() => onSplit(row)}
					onUndoSplit={() => onUndoSplit(row)}
				/>
			{/each}
		{/if}
	</div>
</Card.Root>
