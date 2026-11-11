<script lang="ts">
	import { cn } from '$lib/utils';
	import { STATE_LABELS, type LabState } from '../mockData';

	let { state, class: className = '' }: { state: LabState; class?: string } = $props();

	const DOT: Record<LabState, string> = {
		live: 'bg-green-600',
		closing: 'bg-amber-500',
		upcoming: 'bg-muted-foreground',
		results: 'bg-primary'
	};
</script>

<!-- The one status style used on every card, row and page. -->
<span
	class={cn(
		'bg-background/90 text-foreground flex w-fit items-center gap-2 rounded-full border px-3 py-1 text-base font-semibold',
		className
	)}
>
	<span class={cn('size-2.5 rounded-full', DOT[state])}></span>
	{STATE_LABELS[state]}
</span>
