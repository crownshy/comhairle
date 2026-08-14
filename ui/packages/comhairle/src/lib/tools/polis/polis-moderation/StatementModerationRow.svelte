<script lang="ts">
	import { Checkbox } from '$lib/components/ui/checkbox';
	import { Badge } from '$lib/components/ui/badge';
	import { Check, Pencil, X } from '@lucide/svelte';
	import type { PolisStatementAux } from '@crownshy/api-client/api';
	import RejectReasonPopover from './RejectReasonPopover.svelte';

	type Props = {
		row: PolisStatementAux;
		selected: boolean;
		/** How many rows are selected overall; drives whether this row's accept/reject
		 * acts on the whole selection and how the reject popover is labelled. */
		selectionCount: number;
		/** This row has an accept/reject request in flight. */
		pending: boolean;
		/** A bulk moderation is running for the whole table. */
		bulkWorking: boolean;
		/** Text of the original this row was derived from (if it is a derived statement). */
		editedFrom?: string;
		/** Texts of the derived statements that replaced this row (if it was split). */
		replacedBy?: string[];
		/** `range` is true when shift was held, requesting a range select. */
		onToggle: (checked: boolean, range: boolean) => void;
		onModerate: (status: 'accepted' | 'rejected', reason?: string) => void;
		/** Open the split/reword dialog for this row. */
		onSplit: () => void;
	};

	let {
		row,
		selected,
		selectionCount,
		pending,
		bulkWorking,
		editedFrom,
		replacedBy,
		onToggle,
		onModerate,
		onSplit
	}: Props = $props();

	// When this row is part of a multi-selection, its accept/reject applies to the
	// whole selection (the parent routes it through the bulk path), so label the
	// controls accordingly instead of implying a single-statement action.
	const actsOnSelection = $derived(selected && selectionCount > 1);
	const acceptTitle = $derived(actsOnSelection ? `Accept ${selectionCount} selected` : 'Accept');
	const rejectTitle = $derived(actsOnSelection ? `Reject ${selectionCount} selected` : 'Reject');
	const rejectHeading = $derived(
		actsOnSelection ? `Reject ${selectionCount} statements` : 'Reject statement'
	);

	// Left accent bar colour keyed on seed/status. Olive-green primary stands in
	// for "accepted"; there is no dedicated success token in the theme.
	function accentFor(r: PolisStatementAux): string {
		if (r.is_seed) return 'bg-accent';
		if (r.moderation_status === 'accepted') return 'bg-primary';
		if (r.moderation_status === 'rejected') return 'bg-destructive';
		return 'bg-muted-foreground/40';
	}

	// The checkbox reports toggles via `onCheckedChange`, which carries no event, so
	// we snapshot whether shift was held on the preceding mousedown (fires before
	// the change) to know if this should start a range select.
	let shiftHeld = false;
</script>

<div
	role="button"
	tabindex="0"
	aria-pressed={selected}
	onmousedown={(e) => {
		// Snapshot shift for the checkbox's event-less onCheckedChange (mousedown
		// fires first), and suppress the browser's text selection on shift-click.
		shiftHeld = e.shiftKey;
		if (e.shiftKey) e.preventDefault();
	}}
	onclick={(e) => {
		// Ignore clicks that land on the checkbox or the accept/reject controls.
		if (bulkWorking || (e.target as HTMLElement).closest('[data-row-control]')) return;
		onToggle(!selected, e.shiftKey);
	}}
	onkeydown={(e) => {
		if (e.key === 'Enter' || e.key === ' ') {
			e.preventDefault();
			if (!bulkWorking) onToggle(!selected, e.shiftKey);
		}
	}}
	class={`border-border group relative grid cursor-pointer grid-cols-[2.5rem_3rem_minmax(0,1fr)_auto] items-center gap-4 border-b py-4 pl-4 transition-colors last:border-b-0 ${
		selected ? 'bg-primary/5' : 'hover:bg-muted/40'
	}`}
>
	<!-- Left accent bar (status colour) -->
	<span
		class={`absolute top-0 bottom-0 left-0 w-1 transition-all group-hover:w-1.5 ${accentFor(row)}`}
	></span>

	<!-- Select -->
	<div class="flex items-center" data-row-control>
		<Checkbox
			checked={selected}
			disabled={bulkWorking}
			onCheckedChange={(v) => onToggle(v === true, shiftHeld)}
			aria-label="Select statement"
		/>
	</div>

	<!-- # -->
	<div class="text-muted-foreground text-center text-xs tabular-nums">
		{row.polis_statement_id}
	</div>

	<!-- Statement text (+ derived-statement badge / lineage) -->
	<div class="flex min-w-0 flex-col gap-1">
		<div class="flex items-start gap-2">
			{#if row.original_statement_id}
				<Badge variant="secondary" class="mt-1 shrink-0">Edited</Badge>
			{/if}
			<p class="min-w-0 text-base leading-7">{row.statement_text}</p>
		</div>
		{#if editedFrom}
			<p class="text-muted-foreground text-sm">
				Edited from: <span class="italic">{editedFrom}</span>
			</p>
		{/if}
		{#if replacedBy && replacedBy.length}
			<p class="text-muted-foreground text-sm">
				Replaced by {replacedBy.length} statement{replacedBy.length === 1 ? '' : 's'}
			</p>
		{/if}
		{#if row.moderation_status === 'rejected' && row.moderation_reason}
			<p class="text-muted-foreground text-sm">
				Reason: <span class="italic">{row.moderation_reason}</span>
			</p>
		{/if}
	</div>

	<!-- Actions -->
	<div class="flex items-center gap-1 self-center pr-2" data-row-control>
		{#if !row.is_seed}
			<button
				type="button"
				disabled={pending || bulkWorking}
				onclick={onSplit}
				title="Split or reword"
				class="text-muted-foreground hover:bg-muted inline-flex size-11 cursor-pointer items-center justify-center rounded-full transition-all hover:scale-110 active:scale-95 disabled:cursor-not-allowed disabled:opacity-40 disabled:hover:scale-100 disabled:hover:bg-transparent"
			>
				<Pencil class="size-5" />
			</button>
		{/if}
		<button
			type="button"
			disabled={pending ||
				bulkWorking ||
				(!actsOnSelection && row.moderation_status === 'accepted')}
			onclick={() => onModerate('accepted')}
			title={acceptTitle}
			class="text-primary hover:bg-primary/15 inline-flex size-11 cursor-pointer items-center justify-center rounded-full transition-all hover:scale-110 active:scale-95 disabled:cursor-not-allowed disabled:opacity-40 disabled:hover:scale-100 disabled:hover:bg-transparent"
		>
			<Check class="size-6" />
		</button>
		<RejectReasonPopover
			heading={rejectHeading}
			disabled={pending ||
				bulkWorking ||
				(!actsOnSelection && row.moderation_status === 'rejected')}
			onConfirm={(reason) => onModerate('rejected', reason)}
		>
			{#snippet trigger()}
				<button
					type="button"
					disabled={pending ||
						bulkWorking ||
						(!actsOnSelection && row.moderation_status === 'rejected')}
					title={rejectTitle}
					class="text-destructive hover:bg-destructive/15 inline-flex size-11 cursor-pointer items-center justify-center rounded-full transition-all hover:scale-110 active:scale-95 disabled:cursor-not-allowed disabled:opacity-40 disabled:hover:scale-100 disabled:hover:bg-transparent"
				>
					<X class="size-6" />
				</button>
			{/snippet}
		</RejectReasonPopover>
	</div>
</div>
