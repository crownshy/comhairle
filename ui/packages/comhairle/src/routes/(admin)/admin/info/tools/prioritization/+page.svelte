<script lang="ts">
	import { page } from '$app/state';
	import GuideArticle from '$lib/components/GuideArticle.svelte';
	import { TOOL_GUIDES } from '$lib/tool_guides';

	let key = $derived(page.url.pathname.split('/').filter(Boolean).pop() ?? '');
	let guide = $derived(TOOL_GUIDES[key]);
</script>

<svelte:head>
	<title>{guide?.title ?? 'Tools'} - Comhairle Tools Guide</title>
</svelte:head>

{#if guide}
	<GuideArticle {guide} />
{:else}
	<p class="text-muted-foreground">Guide not found.</p>
{/if}
