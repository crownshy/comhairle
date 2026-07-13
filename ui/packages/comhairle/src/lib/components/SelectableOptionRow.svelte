<script lang="ts">
	import { cn } from '$lib/utils';

	/**
	 * A single selectable radio-row for the two-column picker dialogs (choose-a-template
	 * and add-a-step). Renders a radio dot + name + description, with selected styling.
	 */
	let {
		selected = false,
		name,
		description,
		onSelect,
		class: className = ''
	}: {
		/** Whether this row is the currently-selected option. */
		selected?: boolean;
		/** Option title. */
		name: string;
		/** Option supporting line. */
		description: string;
		/** Called when the row is clicked. */
		onSelect: () => void;
		class?: string;
	} = $props();
</script>

<button
	type="button"
	onclick={onSelect}
	aria-pressed={selected}
	class={cn(
		'flex w-full items-start gap-3 rounded-lg border p-4 text-left transition-colors',
		selected ? 'border-primary bg-accent' : 'border-border bg-card hover:bg-accent/50',
		className
	)}
>
	<span
		class={cn(
			'mt-0.5 flex size-4 shrink-0 items-center justify-center rounded-full border',
			selected ? 'border-primary' : 'border-border'
		)}
	>
		{#if selected}
			<span class="bg-primary size-2 rounded-full"></span>
		{/if}
	</span>
	<span class="flex flex-col gap-1 overflow-hidden">
		<span class={cn('text-sm', selected ? 'text-primary font-medium' : 'text-foreground')}>
			{name}
		</span>
		<span class="text-muted-foreground text-sm">{description}</span>
	</span>
</button>
