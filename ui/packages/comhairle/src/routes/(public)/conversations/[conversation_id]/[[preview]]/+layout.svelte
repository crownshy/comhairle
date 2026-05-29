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
	<div
		class="bg-sidebar lg:bg-sidebar/85 mt-3 w-full py-3 text-center text-white lg:fixed lg:top-6 lg:left-1/2 lg:z-50 lg:mt-0 lg:w-auto lg:-translate-x-1/2 lg:rounded-full lg:px-6 lg:py-2 lg:text-base lg:font-medium lg:shadow-lg lg:backdrop-blur"
	>
		This is a preview of the conversation
	</div>
{/if}
{@render children()}
