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
	// The participant workflow runs in its own chrome (StepChrome + StepPager), so it renders
	// neither the site NavBar nor the Footer. Covers the step pages plus /next, /thank_you and
	// /return, which all sit under /workflow/. The footer's legal links move into the step
	// dropdown; see CONTEXT.md, Participant step chrome.
	const isWorkflowPage = $derived(page.url.pathname.includes('/workflow/'));

	// The conversation landing page is step zero of the participant flow and runs in the same
	// chrome the steps do, so like them it renders neither NavBar nor Footer. Flagged by its
	// own load rather than matched on the pathname, because `[[preview]]` swallows any single
	// trailing segment and a regex here would disagree with the route in those cases.
	const isParticipantChrome = $derived(page.data.participantChrome === true);

	let isAdmin = $derived(
		data.userRoles
			? data.userRoles.find((ur) => ur.resource === 'Site')?.roles.includes('Admin')
			: false
	);
</script>

<div class="flex min-h-dvh w-full flex-col {isReportPage ? 'bg-primary/10' : ''}">
	{#if !isEmbed && !isAuthPage && !isLivePage && !isWorkflowPage && !isParticipantChrome}
		<NavBar user={data.user} {isAdmin} />
	{/if}
	{#if isAuthPage || isReportPage}
		<div class="grow">
			{@render children()}
		</div>
	{:else if isLivePage || isWorkflowPage || isParticipantChrome}
		<div class="flex w-full grow flex-col">
			{@render children()}
		</div>
	{:else}
		<div class="mx-auto min-h-[80vh] w-full max-w-[1300px] grow px-4 md:px-20">
			{@render children()}
		</div>
	{/if}
	{#if !isEmbed && !isLivePage && !isWorkflowPage && !isParticipantChrome}
		<Footer />
	{/if}
</div>
