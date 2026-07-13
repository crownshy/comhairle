<script lang="ts">
	import { Checkbox } from '$lib/components/ui/checkbox';
	import { Check, X } from '@lucide/svelte';
	import type { PolisStatementAux } from '@crownshy/api-client/api';

	let {
		row,
		selected,
		pending,
		bulkWorking,
		onToggle,
		onModerate
	}: {
		row: PolisStatementAux;
		selected: boolean;
		// This row has an accept/reject request in flight.
		pending: boolean;
		// A bulk moderation is running for the whole table.
		bulkWorking: boolean;
		onToggle: (checked: boolean) => void;
		onModerate: (status: 'accepted' | 'rejected') => void;
	} = $props();

	// Left accent bar colour keyed on seed/status. Olive-green primary stands in
	// for "accepted"; there is no dedicated success token in the theme.
	function accentFor(r: PolisStatementAux): string {
		if (r.is_seed) return 'bg-accent';
		if (r.moderation_status === 'accepted') return 'bg-primary';
		if (r.moderation_status === 'rejected') return 'bg-destructive';
		return 'bg-muted-foreground/40';
	}
</script>

<div
	role="button"
	tabindex="0"
	aria-pressed={selected}
	onclick={(e) => {
		// Ignore clicks that land on the checkbox or the accept/reject controls.
		if (bulkWorking || (e.target as HTMLElement).closest('[data-row-control]')) return;
		onToggle(!selected);
	}}
	onkeydown={(e) => {
		if (e.key === 'Enter' || e.key === ' ') {
			e.preventDefault();
			if (!bulkWorking) onToggle(!selected);
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
			onCheckedChange={(v) => onToggle(v === true)}
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
			disabled={pending || bulkWorking || row.moderation_status === 'accepted'}
			onclick={() => onModerate('accepted')}
			title="Accept"
			class="text-primary hover:bg-primary/15 inline-flex size-11 cursor-pointer items-center justify-center rounded-full transition-all hover:scale-110 active:scale-95 disabled:cursor-not-allowed disabled:opacity-40 disabled:hover:scale-100 disabled:hover:bg-transparent"
		>
			<Check class="size-6" />
		</button>
		<button
			type="button"
			disabled={pending || bulkWorking || row.moderation_status === 'rejected'}
			onclick={() => onModerate('rejected')}
			title="Reject"
			class="text-destructive hover:bg-destructive/15 inline-flex size-11 cursor-pointer items-center justify-center rounded-full transition-all hover:scale-110 active:scale-95 disabled:cursor-not-allowed disabled:opacity-40 disabled:hover:scale-100 disabled:hover:bg-transparent"
		>
			<X class="size-6" />
		</button>
	</div>
</div>
