<script lang="ts">
	import { beforeNavigate, goto } from '$app/navigation';
	import { page } from '$app/state';
	import type { LayoutProps } from './$types';

	let { children, data }: LayoutProps = $props();
	let preview = $derived(data.preview);

	// The step chrome shows its own preview pill, so the banner is for the other pages.
	let isStepPage = $derived(
		page.route.id ===
			'/(public)/conversations/[conversation_id]/[[preview]]/workflow/[workflow_id]/s/[workflow_step_id]'
	);
	let showPreviewBanner = $derived(preview && !isStepPage);

	beforeNavigate(({ to, cancel }) => {
		const isEmbed = page.url.searchParams.get('embed') === 'true';

		if (isEmbed && to?.url) {
			// If we're in embed mode and navigating within conversation routes, preserve the embed parameter
			const targetUrl = new URL(to.url);
			if (!targetUrl.searchParams.has('embed')) {
				targetUrl.searchParams.set('embed', 'true');
				// Cancel current navigation and redirect with embed param
				cancel();
				// eslint-disable-next-line svelte/no-navigation-without-resolve
				goto(targetUrl.toString());
			}
		}
	});
</script>

{#if showPreviewBanner}
	<div class="bg-sidebar mt-3 w-full py-3 text-center text-white">
		This is a preview of the conversation
	</div>
{/if}
{@render children()}
