<script lang="ts">
	import { cn } from '$lib/utils';

	let {
		index,
		count
	}: {
		index: number;
		count: number;
	} = $props();

	let dots = $derived(Array.from({ length: count }, (_, i) => i));
</script>

<!-- Position in a slide deck. A single slide has no position to report, so it draws nothing.
	Decorative: the count also reaches assistive tech through the progress bar's label. -->
{#if count > 1}
	<div class="flex items-center gap-2" aria-hidden="true">
		{#each dots as dot (dot)}
			<span class={cn('size-2 rounded-full', dot === index ? 'bg-primary' : 'bg-accent')}
			></span>
		{/each}
	</div>
{/if}
