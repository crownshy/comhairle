<script lang="ts">
	import { page } from '$app/state';
	import ArrowRight from '@lucide/svelte/icons/arrow-right';
	import ToolGuideContent from '$lib/components/AdminGuide/ToolGuideContent.svelte';
	import { GUIDE_NAV, TOOL_GUIDES } from '$lib/tool_guides';

	// The tool's key is the `[tool_id]` route param, which matches the TOOL_GUIDES keys.
	// Laid out like the How-to Comhairle topic pages: eyebrow, title, screenshot, sections,
	// and a Next link.
	let guide = $derived(TOOL_GUIDES[page.params.tool_id ?? '']);

	let next = $derived.by(() => {
		const index = GUIDE_NAV.findIndex((tool) => tool.key === guide?.key);
		return index >= 0 ? GUIDE_NAV[index + 1] : undefined;
	});
</script>

<svelte:head>
	<title>{guide?.title ?? 'Engagement tools'} - Comhairle Handbook</title>
</svelte:head>

<div class="w-full max-w-6xl min-w-0 rounded-xl p-6 md:p-10">
	{#if guide}
		<article class="max-w-3xl">
			<ToolGuideContent {guide} />

			<nav class="border-border mt-10 border-t pt-6" aria-label="Next">
				<a
					href={next ? `/admin/info/tools/${next.key}` : '/admin/info/tools'}
					class="group hover:bg-muted -mx-3 flex items-center justify-between rounded-lg px-3 py-3"
				>
					<span>
						<span class="text-muted-foreground block text-sm"
							>{next ? 'Next tool' : 'Back to'}</span
						>
						<span class="text-foreground font-semibold"
							>{next ? next.title : 'All engagement tools'}</span
						>
					</span>
					<ArrowRight
						class="text-primary size-5 transition-transform group-hover:translate-x-1"
						aria-hidden="true"
					/>
				</a>
			</nav>
		</article>
	{:else}
		<p class="text-muted-foreground">Guide not found.</p>
	{/if}
</div>
