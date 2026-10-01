<script lang="ts">
	import Footer from '$lib/components/Footer.svelte';
	import NavBar from '$lib/components/NavBar.svelte';
	import type { LayoutProps } from './$types';
	import { page } from '$app/state';
	import { setSupportDrawer } from '$lib/components/supportDrawerContext.svelte';
	import { cn } from '$lib/utils';
	import { canAccessAdminPortal } from '$lib/utils/permissions';

	let { children, data }: LayoutProps = $props();
	setSupportDrawer();
	const isEmbed = $derived(page.url.searchParams.get('embed') === 'true');
	const isAuthPage = $derived(page.url.pathname.startsWith('/auth/'));
	const isReportPage = $derived(page.url.pathname.endsWith('/report'));
	const isLivePage = $derived(page.url.pathname.endsWith('/live'));

	// A Room display is projected in a room, so it renders no site chrome at all and
	// fills the viewport: a NavBar and Footer would both steal space from an eight-metre
	// read and shift the layout as the page settles. See CONTEXT.md, "Room display".
	const isRoomDisplay = $derived(
		page.route.id ===
			'/(public)/conversations/[conversation_id]/room-display/[workflow_step_id]'
	);

	// The step shell scrolls inside a fixed viewport, so the step page gets no Footer.
	// See CONTEXT.md, "Step shell".
	const isStepPage = $derived(
		page.route.id ===
			'/(public)/conversations/[conversation_id]/[[preview]]/workflow/[workflow_id]/s/[workflow_step_id]'
	);

	let isAdmin = $derived(canAccessAdminPortal(data.userRoles));
</script>

<!-- The live call fills h-dvh. On mobile 100vh is taller than that while the browser
	toolbar shows, so min-h-screen would leave a strip of blank page to scroll into. -->
<div
	class={cn(
		'flex w-full flex-col',
		isStepPage ? 'h-dvh' : isLivePage ? 'min-h-dvh' : 'min-h-screen',
		isReportPage && 'bg-primary/10'
	)}
>
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
	{:else if isStepPage}
		<div class="flex min-h-0 w-full grow flex-col">
			{@render children()}
		</div>
	{:else}
		<div class="mx-auto min-h-[80vh] w-full max-w-[1300px] grow px-4 md:px-20">
			{@render children()}
		</div>
	{/if}
	{#if !isEmbed && !isLivePage && !isRoomDisplay && !isStepPage}
		<Footer />
	{/if}
</div>
