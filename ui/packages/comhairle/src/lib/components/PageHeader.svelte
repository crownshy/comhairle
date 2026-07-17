<script lang="ts">
	import type { Snippet } from 'svelte';

	type Props = {
		/** Page title, rendered as the page's single h1. */
		title: string;
		/** Optional supporting line shown beneath the title. */
		description?: string;
		/** Optional controls aligned to the right of the title row (e.g. a template picker). */
		actions?: Snippet;
	};

	let { title, description, actions }: Props = $props();
</script>

<!-- Shared heading block for admin pages: fixes the title/description sizes and the
	 spacing beneath the header so every route reads as one system. Top spacing comes
	 from the surrounding content wrapper, not from here, so pages stay vertically aligned. -->
<div class="mb-8 flex flex-col gap-2 sm:flex-row sm:items-start sm:justify-between">
	<div class="flex flex-col gap-1">
		<h1 class="text-2xl font-bold">{title}</h1>
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
