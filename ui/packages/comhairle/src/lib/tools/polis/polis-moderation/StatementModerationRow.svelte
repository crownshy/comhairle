<script lang="ts">
	import { Checkbox } from '$lib/components/ui/checkbox';
	import { Badge } from '$lib/components/ui/badge';
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { cn } from '$lib/utils';
	import { Check, ChevronDown, Ellipsis, Pencil, Undo2, X } from '@lucide/svelte';
	import type { PolisStatementAux } from '@crownshy/api-client/api';
	import type { RejectReason } from '$lib/moderation/moderationPolicy';
	import RejectDialog from './RejectDialog.svelte';
	import TranslationsPanel from './TranslationsPanel.svelte';
	import { pluralise } from '$lib/utils/pluralise';

	type ModerationStatus = PolisStatementAux['moderation_status'];

	type Props = {
		editable?: boolean;
		row: PolisStatementAux;
		/** Show the status in the meta line, for views that mix statuses. */
		showStatus: boolean;
		/** The conversation's moderation policy reasons, offered on reject. */
		rejectReasons: RejectReason[];
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
		onModerate: (status: ModerationStatus, reason?: string) => void;
		/** Open the split/reword dialog for this row. */
		onSplit: () => void;
	};

	let {
		editable = true,
		row,
		showStatus,
		rejectReasons,
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
	const acceptLabel = $derived(actsOnSelection ? `Accept ${selectionCount}` : 'Accept');
	const rejectLabel = $derived(actsOnSelection ? `Reject ${selectionCount}` : 'Reject');
	const controlsDisabled = $derived(pending || bulkWorking);
	// A decided row swaps its spent action for the way back, so every button does something.
	const showsAccepted = $derived(!actsOnSelection && row.moderation_status === 'accepted');
	const showsRejected = $derived(!actsOnSelection && row.moderation_status === 'rejected');
	// Decided rows are for reading back, so their actions stay quiet icons until hovered.
	const quiet = $derived(showsAccepted || showsRejected);
	const ACTION_SIZE = 'h-11 min-w-11 px-3 text-base';
	const acceptClass = $derived(
		cn(
			ACTION_SIZE,
			'hover:bg-primary/15 hover:text-primary',
			quiet ? 'text-muted-foreground' : 'text-primary'
		)
	);
	const rejectClass = $derived(
		cn(
			ACTION_SIZE,
			'hover:bg-destructive/15 hover:text-destructive',
			quiet ? 'text-muted-foreground' : 'text-destructive'
		)
	);
	const labelClass = $derived(quiet ? 'sr-only' : 'sr-only lg:not-sr-only');
	const rejectHeading = $derived(
		actsOnSelection ? `Reject ${selectionCount} statements` : 'Reject statement'
	);

	const hasMoreActions = $derived(editable && (quiet || actsOnSelection));

	// Left accent bar colour keyed on status, so a seed statement shows its status too.
	// Olive-green primary stands in for "accepted"; there is no dedicated success token.
	function accentFor(r: PolisStatementAux): string {
		if (r.moderation_status === 'accepted') return 'bg-primary';
		if (r.moderation_status === 'rejected') return 'bg-destructive';
		return 'bg-muted-foreground/40';
	}

	const STATUS_LABELS: Record<ModerationStatus, string> = {
		pending: 'Waiting',
		accepted: 'Accepted',
		rejected: 'Rejected'
	};

	// Whether shift was held, for range-select. Every toggle path reads this one
	// snapshot: we capture it in the capture phase, which runs before the checkbox's
	// event-less `onCheckedChange` and also covers a keyboard toggle of the checkbox
	// (Space fires no mousedown). Deliberately a plain `let`, not `$state`: it is a
	// transient input snapshot and must not drive reactivity.
	let shiftHeld = false;
	const snapshotShift = (e: MouseEvent | KeyboardEvent) => {
		shiftHeld = e.shiftKey;
	};

	let translationsOpen = $state(false);
</script>

<!-- Clicking the row is a mouse shortcut for its checkbox, which stays the keyboard and
     screen reader control, so the row itself is not a button. -->
<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
<div
	onmousedowncapture={(e) => {
		snapshotShift(e);
		// Suppress the browser's text selection on shift-click.
		if (e.shiftKey) e.preventDefault();
	}}
	onkeydowncapture={snapshotShift}
	onclick={(e) => {
		// Clicking the row toggles selection, except on its own controls (checkbox, accept/reject, translations).
		if (!editable || bulkWorking || (e.target as HTMLElement).closest('[data-skip-row-toggle]'))
			return;
		onToggle(!selected, shiftHeld);
	}}
	class:cursor-pointer={editable}
	class={`border-border group relative grid grid-cols-[2.5rem_minmax(0,1fr)] items-center gap-x-4 gap-y-2 border-b py-4 pl-4 transition-colors last:border-b-0 @2xl:grid-cols-[2.5rem_minmax(0,1fr)_auto] ${
		selected ? 'bg-primary/5' : 'hover:bg-muted/40'
	}`}
>
	<!-- Status colour, only where statuses mix. In a filtered tab every bar would match. -->
	{#if showStatus}
		<span
			class={`absolute top-0 bottom-0 left-0 w-1 transition-all group-hover:w-1.5 ${accentFor(row)}`}
		></span>
	{/if}

	<!-- Select -->
	<div class="flex items-center" data-skip-row-toggle>
		{#if editable}
			<Checkbox
				checked={selected}
				disabled={bulkWorking}
				onCheckedChange={(v) => onToggle(v === true, shiftHeld)}
				aria-label="Select statement"
				class="border-muted-foreground"
			/>
		{/if}
	</div>

	<!-- Statement text, then its meta line and lineage -->
	<div class="flex min-w-0 flex-col gap-1">
		<p class="min-w-0 text-base leading-7 font-medium">{row.statement_text}</p>
		<div class="text-muted-foreground flex flex-wrap items-center gap-2 text-base">
			<span class="tabular-nums">#{row.polis_statement_id}</span>
			{#if editable && showStatus}
				<span>{STATUS_LABELS[row.moderation_status]}</span>
			{/if}
			{#if row.is_seed}
				<Badge variant="secondary">Seed</Badge>
			{/if}
			{#if editedFrom}
				<span>Edited from <span class="italic">“{editedFrom}”</span></span>
			{:else if row.original_statement_id}
				<Badge variant="secondary">Edited</Badge>
			{/if}
			<button
				type="button"
				data-skip-row-toggle
				aria-expanded={translationsOpen}
				onclick={() => (translationsOpen = !translationsOpen)}
				class="hover:text-foreground focus-visible:ring-ring/50 inline-flex min-h-6 items-center gap-1 rounded-sm underline-offset-4 outline-none hover:underline focus-visible:ring-[3px]"
			>
				Translations
				<ChevronDown
					class={cn('size-4 transition-transform', translationsOpen && 'rotate-180')}
				/>
			</button>
		</div>
		{#if replacedBy && replacedBy.length}
			<p class="text-muted-foreground text-base">
				Replaced by {replacedBy.length}
				{pluralise(replacedBy.length, 'statement')}
			</p>
		{/if}
		{#if row.moderation_status === 'rejected' && row.moderation_reason}
			<p class="text-muted-foreground text-base">
				Reason: <span class="italic">{row.moderation_reason}</span>
			</p>
		{/if}
	</div>

	<div
		class="col-start-2 -ml-3 flex items-center gap-1 self-center pr-2 @2xl:col-start-auto @2xl:ml-0"
		data-skip-row-toggle
	>
		{#if editable}
			{#snippet acceptButton()}
				<Button
					variant="ghost"
					disabled={controlsDisabled}
					onclick={() => onModerate('accepted')}
					title={quiet ? acceptLabel : undefined}
					class={acceptClass}
				>
					<Check class="size-5" />
					<span class={labelClass}>{acceptLabel}</span>
				</Button>
			{/snippet}

			{#if !showsAccepted && !showsRejected}
				{@render acceptButton()}
			{/if}
			<!-- Seeds can't be split, but keep the slot so every row's actions line up. -->
			<Button
				variant="ghost"
				disabled={controlsDisabled || row.is_seed}
				onclick={onSplit}
				title={quiet ? 'Split or reword' : undefined}
				aria-hidden={row.is_seed}
				tabindex={row.is_seed ? -1 : undefined}
				class={cn(
					ACTION_SIZE,
					'text-muted-foreground hover:text-foreground',
					row.is_seed && 'invisible'
				)}
			>
				<Pencil class="size-5" />
				<span class={labelClass}>Split or reword</span>
			</Button>
			{#if showsRejected}
				{@render acceptButton()}
			{:else}
				<RejectDialog
					reasons={rejectReasons}
					heading={rejectHeading}
					statementText={actsOnSelection ? undefined : row.statement_text}
					disabled={controlsDisabled}
					onConfirm={(reason) => onModerate('rejected', reason)}
				>
					{#snippet trigger({ props })}
						<Button
							{...props}
							variant="ghost"
							disabled={controlsDisabled}
							title={quiet ? rejectLabel : undefined}
							class={rejectClass}
						>
							<X class="size-5" />
							<span class={labelClass}>{rejectLabel}</span>
						</Button>
					{/snippet}
				</RejectDialog>
			{/if}
		{:else}
			<Badge variant="outline" class="text-base">{STATUS_LABELS[row.moderation_status]}</Badge
			>
		{/if}
		{#if hasMoreActions}
			<DropdownMenu.Root>
				<DropdownMenu.Trigger
					aria-label="More actions"
					title="More actions"
					class={cn(buttonVariants({ variant: 'ghost', size: 'icon' }), 'size-11')}
				>
					<Ellipsis class="size-5" />
				</DropdownMenu.Trigger>
				<DropdownMenu.Content align="end" class="min-w-56">
					{#if quiet}
						<DropdownMenu.Item
							disabled={controlsDisabled}
							onSelect={() => onModerate('pending')}
							class="text-base"
						>
							<Undo2 class="size-4" />
							Move back to waiting
						</DropdownMenu.Item>
					{/if}
					{#if actsOnSelection}
						<DropdownMenu.Item
							disabled={controlsDisabled}
							onSelect={() => onModerate('pending')}
							class="text-base"
						>
							<Undo2 class="size-4" />
							Move {selectionCount} back to waiting
						</DropdownMenu.Item>
					{/if}
				</DropdownMenu.Content>
			</DropdownMenu.Root>
		{/if}
	</div>

	{#if translationsOpen}
		<div class="col-span-full pr-2 pl-[3.5rem]" data-skip-row-toggle>
			<TranslationsPanel polisStatementAuxId={row.id} />
		</div>
	{/if}
</div>
