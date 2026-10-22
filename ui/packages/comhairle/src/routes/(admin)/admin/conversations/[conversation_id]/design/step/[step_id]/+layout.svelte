<script lang="ts">
	import { page } from '$app/state';
	import SubTabStrip from '$lib/components/SubTabStrip.svelte';
	import { permissions } from '$lib/permissions.svelte';
	import { useTour } from '$lib/tours/useTour.svelte';
	import { stepTour } from './tour';

	useTour(() => stepTour);

	let { data, children } = $props();
	const canEdit = $derived(
		permissions.can('conversation', 'conversation_update', data.conversation.id)
	);

	let subtabItems = $derived(data.subtabItems);
	let basePath = $derived(
		`/admin/conversations/${data.conversation.id}/design/step/${data.step_id}`
	);
	let pageTitle = $derived(`${canEdit ? 'Edit Step' : 'Step'}: ${data.step?.name ?? 'Step'}`);

	// The heyform Setup tab embeds the full HeyForm builder (a desktop iframe app). It owns the
	// whole content region full-bleed, with no reading-column padding or max-width, so the frame
	// can fill the space and scale itself to fit. Every other step page keeps the padded column.
	let isHeyformSetup = $derived(
		canEdit &&
			page.url.pathname.replace(/\/+$/, '').endsWith('/setup') &&
			data.toolConfig?.type === 'heyform'
	);
</script>

<svelte:head>
	<title>{pageTitle} - Comhairle Admin</title>
</svelte:head>

<!-- The step's own tabs (Setup/Configure/Insights), the only strip under the step
	 header. Rendered from `data` so a hard refresh of /design/step/* paints it straight away. -->
<div data-tour="step-subtabs" class="shrink-0">
	<SubTabStrip tone="primary" items={subtabItems} {basePath} />
</div>

{#if isHeyformSetup}
	<div class="bg-admin-background min-h-0 grow overflow-hidden">
		{@render children()}
	</div>
{:else}
	<div
		class="bg-admin-background pt-page-top px-gutter grow pb-8 sm:pb-12 md:min-h-0 md:overflow-y-auto"
	>
		<div class="max-w-admin mx-auto min-h-full w-full">
			{@render children()}
		</div>
	</div>
{/if}
