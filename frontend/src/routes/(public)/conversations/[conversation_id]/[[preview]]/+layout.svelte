<script lang="ts">
	import { beforeNavigate, goto } from '$app/navigation';
	import { page } from '$app/state';
	import type { LayoutProps } from './$types';

	let { children, data }: LayoutProps = $props();
	let preview = $derived(data.preview);

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

{#if preview}
	<div class="bg-secondary mt-3 w-full py-3 text-center text-white">
		This is a preview of the conversation
	</div>
{/if}
{@render children()}
