<script lang="ts">
	import { cn } from '$lib/utils';
	import type { Snippet } from 'svelte';

	type Props = {
		/** Page title, rendered as the page's single h1. */
		title: string;
		/**
		 * Use `'h2'` under a section header that already provides the page's h1 (e.g. a
		 * Configure tab), which also renders the title one size smaller.
		 */
		level?: 'h1' | 'h2';
		/** Optional supporting line shown beneath the title. */
		description?: string;
		/** Optional controls aligned to the right of the title row (e.g. a template picker). */
		actions?: Snippet;
	};

	let { title, description, level = 'h1', actions }: Props = $props();
</script>

<!-- Shared heading block for admin pages: fixes the title/description sizes and the
	 spacing beneath the header so every route reads as one system. Top spacing comes
	 from the surrounding content wrapper, not from here, so pages stay vertically aligned. -->
<div class="mb-8 flex flex-col gap-2 sm:flex-row sm:items-start sm:justify-between">
	<div class="flex flex-col gap-1">
		<svelte:element
			this={level}
			class={cn('font-bold', level === 'h1' ? 'text-2xl' : 'text-xl')}
		>
			{title}
		</svelte:element>
		{#if description}
			<p class="text-muted-foreground text-base">{description}</p>
		{/if}
	</div>
	{#if actions}
		<div class="shrink-0">
			{@render actions()}
		</div>
	{/if}
</div>
