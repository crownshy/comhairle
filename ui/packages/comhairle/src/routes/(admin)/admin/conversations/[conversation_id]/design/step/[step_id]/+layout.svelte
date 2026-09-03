<script lang="ts">
	import { page } from '$app/state';
	import SubTabStrip from '$lib/components/SubTabStrip.svelte';

	const { data, children, params } = $props();
	const { step_id, conversation_id } = $derived(params);

	// The heyform Setup tab embeds the full HeyForm builder (a desktop iframe app). It owns the
	// whole content region full-bleed, with no reading-column padding or max-width, so the frame
	// can fill the space and scale itself to fit. Every other step page keeps the padded column.
	let isHeyformSetup = $derived(
		page.url.pathname.replace(/\/+$/, '').endsWith('/setup') &&
			data.toolConfig?.type === 'heyform'
	);
</script>

<svelte:head>
	<title>Edit Step: {data.step?.name ?? 'Step'} - Comhairle Admin</title>
</svelte:head>

<!-- Row 4: the step's sub-tab strip, real routes (Configure/Setup/Moderation/Insights).
	 Rendered here from `data` (server-rendered) so a hard refresh of /design/step/* paints it
	 immediately, rather than flashing in after hydration. The parent conversation layout gives
	 the step route a full-bleed region (no padded wrapper) so this strip sits flush under Row 3. -->
<SubTabStrip
	items={data.subtabItems}
	basePath={`/admin/conversations/${conversation_id}/design/step/${step_id}`}
/>

{#if isHeyformSetup}
	<div class="bg-admin-background min-h-0 grow overflow-hidden">
		{@render children()}
	</div>
{:else}
	<div class="bg-admin-background pt-page-top px-gutter grow pb-8 sm:pr-8 sm:pb-12 lg:pr-16">
		<div class="h-full w-full max-w-300">
			{@render children()}
		</div>
	</div>
{/if}
