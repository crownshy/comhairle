<script lang="ts">
	import AdminNav from '$lib/components/AdminNav.svelte';
	import SidebarFloatingTriggers from '$lib/components/SidebarFloatingTriggers.svelte';
	import * as SideBar from '$lib/components/ui/sidebar';
	import { setSidebarWidth } from '$lib/components/sidebarWidthContext.svelte.js';
	import { clampWidth, DEFAULT_WIDTH } from '$lib/components/sidebarWidth.js';
	import type { LayoutProps } from './$types';
	import { page } from '$app/state';
	import { loginRedirect } from '$lib/urls';

	let { children, data }: LayoutProps = $props();
	let ownedConversations = $derived(data.ownedConversations);
	let permittedConversations = $derived(data.permittedConversations);
	let userOrganizations = $derived(data.userOrganizations);

	if (!data.user) {
		loginRedirect(page.url.toString(), 'You need to be logged in to access this');
	}

	// Writable derived seeded from the server-read cookie: SSR and the live value are one
	// reactive expression, so first paint is correct (no jump) and a drag overrides it
	// until the next load re-seeds. See ADR-0004.
	let sidebarWidthPx = $derived(clampWidth(data.sidebarWidth ?? DEFAULT_WIDTH));
	const sidebarWidth = setSidebarWidth({
		width: () => sidebarWidthPx,
		setWidth: (px) => {
			sidebarWidthPx = clampWidth(px);
		}
	});
</script>

<SideBar.Provider
	class="h-svh"
	style="--sidebar-width: {sidebarWidthPx}px;"
	data-sidebar-resizing={sidebarWidth.resizing ? '' : undefined}
>
	<AdminNav
		user={data.user}
		ownedConversations={ownedConversations?.records ?? []}
		permittedConversations={permittedConversations?.records ?? []}
		userOrganizations={userOrganizations ?? {
			organizations: [],
			canCreateOrganization: false
		}}
		path={page.url.pathname}
	/>
	<SideBar.Inset class="min-h-0 min-w-0">
		<SidebarFloatingTriggers />
		<main class="bg-muted flex min-h-0 w-full min-w-0 grow flex-col overflow-y-auto">
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
