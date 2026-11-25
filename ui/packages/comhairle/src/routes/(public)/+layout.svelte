<script lang="ts">
	import Footer from '$lib/components/Footer.svelte';
	import NavBar from '$lib/components/NavBar.svelte';
	import type { LayoutProps } from './$types';
	import { page } from '$app/state';
	import { setSupportDrawer } from '$lib/components/supportDrawerContext.svelte';
	import { cn } from '$lib/utils';
	import { canAccessAdminPortal } from '$lib/utils/permissions';
	import { MAIN_CONTENT_ID } from '$lib/utils/mainContent';
	import { m } from '$lib/paraglide/messages';

	let { children, data }: LayoutProps = $props();
	setSupportDrawer();
	const isEmbed = $derived(page.url.searchParams.get('embed') === 'true');
	// The preview password page is a sign-in page in all but path, so it gets the same
	// full-screen split with no nav bar.
	const isAuthPage = $derived(
		page.url.pathname.startsWith('/auth/') ||
			page.route.id === '/(public)/conversations/[conversation_id]/preview-unlock'
	);
	const isReportPage = $derived(page.url.pathname.endsWith('/report'));
	const isLivePage = $derived(page.url.pathname.endsWith('/live'));
	const isHomePage = $derived(page.route.id === '/(public)');
	// The conversation's own page runs edge to edge, like home, with the image across the screen.
	const isConversationPage = $derived(
		page.route.id === '/(public)/conversations/[conversation_id]/[[preview]]'
	);

	// A Room display is projected in a room, so it renders no site chrome at all and
	// fills the viewport: a NavBar and Footer would both steal space from an eight-metre
	// read and shift the layout as the page settles. See CONTEXT.md, "Room display".
	const isRoomDisplay = $derived(
		page.route.id ===
			'/(public)/conversations/[conversation_id]/room-display/[workflow_step_id]'
	);

	// The step page draws its own header row and pager in place of the NavBar and Footer.
	// See CONTEXT.md, "Step shell".
	const isStepPage = $derived(
		page.route.id ===
			'/(public)/conversations/[conversation_id]/[[preview]]/workflow/[workflow_id]/s/[workflow_step_id]'
	);

	const viewportHeightClass = $derived.by(() => {
		if (isStepPage) return 'h-dvh';
		if (isLivePage) return 'min-h-dvh';
		return 'min-h-screen';
	});

	const mainClass = $derived.by(() => {
		if (isRoomDisplay) return undefined;
		if (isAuthPage || isReportPage || isHomePage || isConversationPage) return 'grow';
		if (isLivePage) return 'w-full grow';
		return 'mx-auto min-h-[80vh] w-full max-w-[1300px] grow px-4 md:px-20';
	});

	let isAdmin = $derived(canAccessAdminPortal(data.userRoles));
</script>

<!-- The live call fills h-dvh. On mobile 100vh is taller than that while the browser
	toolbar shows, so min-h-screen would leave a strip of blank page to scroll into. -->
<!-- Home and conversation pages sit on a light tint of the theme colour, so white panels stand out.
	It's set here, not on the page, so the gap above the footer is tinted too. -->
<div
	class={cn(
		'bg-background flex w-full flex-col',
		(isHomePage || isConversationPage) &&
			'bg-[color-mix(in_oklab,var(--primary)_6%,var(--background))]',
		viewportHeightClass
	)}
>
	<a
		href="#{MAIN_CONTENT_ID}"
		class="bg-background text-foreground ring-ring sr-only z-50 rounded-md px-4 py-2 text-base font-medium shadow-md ring-2 focus:not-sr-only focus:fixed focus:top-4 focus:left-4"
	>
		{m.skip_to_content()}
	</a>
	{#if !isEmbed && !isAuthPage && !isLivePage && !isRoomDisplay && !isStepPage}
		<NavBar user={data.user} {isAdmin} overHero={isHomePage} />
	{/if}
	{#if isStepPage}
		<!-- The step page puts the main landmark on its own scroll, so its header and pager stay outside it. -->
		<div class="flex min-h-0 w-full grow flex-col">
			{@render children()}
		</div>
	{:else}
		<main id={MAIN_CONTENT_ID} tabindex="-1" class={cn('outline-none', mainClass)}>
			{@render children()}
		</main>
	{/if}
	<!-- The report pages by its own pager, so a footer would only sit under it. -->
	{#if !isEmbed && !isLivePage && !isRoomDisplay && !isStepPage && !isReportPage}
		<Footer />
	{/if}
</div>
