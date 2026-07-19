<script lang="ts">
	import SubTabStrip from '$lib/components/SubTabStrip.svelte';

	let { data, children } = $props();

	let subtabItems = $derived(data.subtabItems);
	let basePath = $derived(
		`/admin/conversations/${data.conversation.id}/design/step/${data.step_id}`
	);
	let pageTitle = $derived(`Edit Step: ${data.step?.name ?? 'Step'}`);
</script>

<svelte:head>
	<title>{pageTitle} - Comhairle Admin</title>
</svelte:head>

<!-- Row 4: the step's sub-tab strip, real routes (Configure/Setup/Moderation/Insights).
	 Rendered here from `data` (server-rendered) so a hard refresh of /design/step/* paints it
	 immediately, rather than flashing in after hydration. The parent conversation layout gives
	 the step route a full-bleed region (no padded wrapper) so this strip sits flush under Row 3. -->
<SubTabStrip items={subtabItems} {basePath} />

<div class="bg-muted pt-page-top px-gutter grow pb-8 sm:pr-8 sm:pb-12 lg:pr-16">
	<div class="h-full w-full max-w-[1200px]">
		{@render children()}
	</div>
</div>
