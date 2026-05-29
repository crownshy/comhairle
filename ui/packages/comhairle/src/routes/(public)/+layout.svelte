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
	const isWorkflowStep = $derived(/\/workflow\/[^/]+\/s\/[^/]+/.test(page.url.pathname));

	let isAdmin = $derived(
		data.userRoles
			? data.userRoles.find((ur) => ur.resource === 'Site')?.roles.includes('Admin')
			: false
	);
</script>

<svelte:head>
	{#if isWorkflowStep}
		<style>
			@media (min-width: 1024px) {
				html,
				body {
					overflow: hidden;
				}
			}
		</style>
	{/if}
</svelte:head>

<div
	class="flex min-h-screen w-full flex-col {isReportPage ? 'bg-primary/10' : ''} {isWorkflowStep
		? 'lg:h-screen lg:min-h-0 lg:overflow-hidden'
		: ''}"
>
	{#if !isEmbed && !isAuthPage && !isLivePage}
		<NavBar user={data.user} {isAdmin} />
	{/if}
	{#if isAuthPage || isReportPage}
		<div class="grow">
			{@render children()}
		</div>
	{:else if isLivePage}
		<div class="w-full grow">
			{@render children()}
		</div>
	{:else if isWorkflowStep}
		<div class="w-full grow lg:flex lg:min-h-0 lg:flex-col">
			<div
				class="mx-auto min-h-[80vh] w-full max-w-[1300px] px-4 md:px-20 lg:flex lg:min-h-0 lg:max-w-none lg:flex-1 lg:flex-col lg:px-0"
			>
				{@render children()}
			</div>
		</div>
	{:else}
		<div class="mx-auto min-h-[80vh] w-full max-w-[1300px] grow px-4 md:px-20">
			{@render children()}
		</div>
	{/if}
	{#if !isEmbed && !isLivePage && !isWorkflowStep}
		<Footer />
	{/if}
</div>
