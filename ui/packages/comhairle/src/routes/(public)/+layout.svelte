<script lang="ts">
	import Footer from '$lib/components/Footer.svelte';
	import NavBar from '$lib/components/NavBar.svelte';
	import type { LayoutProps } from './$types';
	import { page } from '$app/state';

	let { children, data }: LayoutProps = $props();
	const isEmbed = $derived(page.url.searchParams.get('embed') === 'true');
	const isAuthPage = $derived(page.url.pathname.startsWith('/auth/'));
	const isReportPage = $derived(page.url.pathname.endsWith('/report'));
	const isLivePage = $derived(page.url.pathname.endsWith('/live'));
	// A Room display is projected in a room, so it renders no site chrome at all and
	// fills the viewport: a NavBar and Footer would both steal space from an eight-metre
	// read and shift the layout as the page settles. See CONTEXT.md, "Room display".
	const isRoomDisplay = $derived(page.url.pathname.includes('room-display'));

	let isAdmin = $derived(
		data.userRoles
			? data.userRoles.find((ur) => ur.resource === 'Site')?.roles.includes('Admin')
			: false
	);
</script>

<div class="flex min-h-screen w-full flex-col {isReportPage ? 'bg-primary/10' : ''}">
	{#if !isEmbed && !isAuthPage && !isLivePage && !isRoomDisplay}
		<NavBar user={data.user} {isAdmin} />
	{/if}
	{#if isRoomDisplay}
		{@render children()}
	{:else if isAuthPage || isReportPage}
		<div class="grow">
			{@render children()}
		</div>
	{:else if isLivePage}
		<div class="w-full grow">
			{@render children()}
		</div>
	{:else}
		<div class="mx-auto min-h-[80vh] w-full max-w-[1300px] grow px-4 md:px-20">
			{@render children()}
		</div>
	{/if}
	{#if !isEmbed && !isLivePage && !isRoomDisplay}
		<Footer />
	{/if}
</div>
