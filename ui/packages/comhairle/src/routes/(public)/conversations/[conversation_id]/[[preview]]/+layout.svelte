<script lang="ts">
	import { beforeNavigate, goto } from '$app/navigation';
	import { page } from '$app/state';
	import type { LayoutProps } from './$types';

	let { children, data }: LayoutProps = $props();
	let preview = $derived(data.preview);

	// Participant chrome pages (the landing page and the steps) carry a compact preview pill in
	// their header, so this full-width banner covers the conversation's other pages only.
	let showPreviewBanner = $derived(preview && !page.data.participantChrome);

	beforeNavigate(({ to, cancel }) => {
		const isEmbed = $derived(page.url.searchParams.get('embed') === 'true');

		if (isEmbed && to?.url) {
			// If we're in embed mode and navigating within conversation routes, preserve the embed parameter
			const targetUrl = new URL(to.url);
			if (!targetUrl.searchParams.has('embed')) {
				targetUrl.searchParams.set('embed', 'true');
				// Cancel current navigation and redirect with embed param
				cancel();
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
