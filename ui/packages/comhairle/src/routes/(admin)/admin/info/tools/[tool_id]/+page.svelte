<script lang="ts">
	import { page } from '$app/state';
	import GuideArticle from '$lib/components/GuideArticle.svelte';
	import { TOOL_GUIDES } from '$lib/tool_guides';

	// The tool's key is the `[tool_id]` route param, which matches the TOOL_GUIDES keys.
	let guide = $derived(TOOL_GUIDES[page.params.tool_id ?? '']);
</script>

<svelte:head>
	<title>{guide?.title ?? 'Tools'} - Comhairle Tools Guide</title>
</svelte:head>

{#if guide}
	<GuideArticle {guide} />
{:else}
	<p class="text-muted-foreground">Guide not found.</p>
{/if}
