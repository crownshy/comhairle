<script lang="ts">
	import AdminNav from '$lib/components/AdminNav.svelte';
	import SidebarFloatingTriggers from '$lib/components/SidebarFloatingTriggers.svelte';
	import * as SideBar from '$lib/components/ui/sidebar';
	import { sidebarWidth } from '$lib/components/sidebarWidth.svelte.js';
	import type { LayoutProps } from './$types';
	import { page } from '$app/state';
	import { loginRedirect } from '$lib/urls';

	let { children, data }: LayoutProps = $props();
	let conversations = $derived(data.conversations);

	if (!data.user) {
		loginRedirect(page.url.toString(), 'You need to be logged in to access this');
	}

	$effect(() => {
		sidebarWidth.hydrate();
	});
</script>

<SideBar.Provider
	style="--sidebar-width: {sidebarWidth.width}px;"
	data-sidebar-resizing={sidebarWidth.resizing || sidebarWidth.initializing ? '' : undefined}
>
	<AdminNav user={data.user} conversations={conversations.records} path={page.url.pathname} />
	<SideBar.Inset>
		<SidebarFloatingTriggers />
		<main class="bg-muted flex grow flex-col overflow-y-auto">
			{@render children()}
		</main>
	</SideBar.Inset>
</SideBar.Provider>

<style>
	:global([data-slot='sidebar-wrapper'] [data-slot='sidebar-gap']),
	:global([data-slot='sidebar-wrapper'] [data-slot='sidebar-container']) {
		transition-duration: 320ms;
		transition-timing-function: cubic-bezier(0.32, 0.72, 0, 1);
	}
	:global([data-sidebar-resizing] [data-slot='sidebar-gap']),
	:global([data-sidebar-resizing] [data-slot='sidebar-container']) {
		transition: none !important;
	}
</style>
