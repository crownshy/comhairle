<script lang="ts">
	import AdminNav from '$lib/components/AdminNav.svelte';
	import SidebarFloatingTriggers from '$lib/components/SidebarFloatingTriggers.svelte';
	import * as SideBar from '$lib/components/ui/sidebar';
	import { setSidebarWidth } from '$lib/components/sidebarWidthContext.svelte.js';
	import { clampWidth, DEFAULT_WIDTH } from '$lib/components/sidebarWidth.js';
	import type { LayoutProps } from './$types';
	import { page } from '$app/state';
	import { loginRedirectServer } from '$lib/urls';
	import AutosaveLeaveGuard from '$lib/components/Translation/AutosaveLeaveGuard.svelte';
	import { replaceState } from '$app/navigation';
	import PrototypeSwitcher from '$lib/components/PrototypeSwitcher.svelte';
	import SidebarDrillIn from './sidebar-prototype/SidebarDrillIn.svelte';
	import SidebarSwitcher from './sidebar-prototype/SidebarSwitcher.svelte';
	import SidebarRail from './sidebar-prototype/SidebarRail.svelte';
	import SidebarJourney from './sidebar-prototype/SidebarJourney.svelte';
	import {
		SIDEBAR_VARIANTS,
		SIDEBAR_VARIANT_COOKIE,
		isSidebarVariant,
		setSidebarVariant,
		type SidebarVariantKey
	} from './sidebar-prototype/variant.svelte';

	let { children, data }: LayoutProps = $props();
	let ownedConversations = $derived(data.ownedConversations);
	let permittedConversations = $derived(data.permittedConversations);
	let userOrganizations = $derived(data.userOrganizations);
	let userRoles = $derived(data.userRoles);

	if (!data.user) {
		loginRedirectServer(page.url.toString(), 'You need to be logged in to access this');
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

	// PROTOTYPE: sidebar variants. `?variant=` wins so links are shareable; otherwise the cookie
	// keeps the choice across navigations. Delete with sidebar-prototype/.
	let chosenVariant = $derived<SidebarVariantKey>(
		isSidebarVariant(data.sidebarPrototypeVariant) ? data.sidebarPrototypeVariant : 'current'
	);
	let urlVariant = $derived(page.url.searchParams.get('variant'));
	let sidebarVariant = $derived<SidebarVariantKey>(
		isSidebarVariant(urlVariant) ? urlVariant : chosenVariant
	);
	function chooseVariant(key: string) {
		if (!isSidebarVariant(key)) return;
		chosenVariant = key;
		document.cookie = `${SIDEBAR_VARIANT_COOKIE}=${key}; path=/; max-age=2592000`;
		const url = new URL(page.url);
		url.searchParams.set('variant', key);
		replaceState(url, page.state);
	}
	setSidebarVariant({
		get current() {
			return sidebarVariant;
		},
		set: chooseVariant
	});
</script>

<SideBar.Provider
	class="h-svh"
	style="--sidebar-width: {sidebarWidthPx}px;"
	data-sidebar-resizing={sidebarWidth.resizing ? '' : undefined}
>
	{#if sidebarVariant === 'drill-in'}
		<SidebarDrillIn user={data.user} />
	{:else if sidebarVariant === 'switcher'}
		<SidebarSwitcher
			user={data.user}
			ownedConversations={ownedConversations?.records ?? []}
			permittedConversations={permittedConversations?.records ?? []}
		/>
	{:else if sidebarVariant === 'rail'}
		<SidebarRail user={data.user} />
	{:else if sidebarVariant === 'journey'}
		<SidebarJourney user={data.user} />
	{:else}
		<AdminNav
			user={data.user}
			ownedConversations={ownedConversations?.records ?? []}
			permittedConversations={permittedConversations?.records ?? []}
			userOrganizations={userOrganizations ?? {
				organizations: [],
				canCreateOrganization: false
			}}
			{userRoles}
			path={page.url.pathname}
		/>
	{/if}
	<SideBar.Inset class="min-h-0 min-w-0">
		<SidebarFloatingTriggers />
		<main class="bg-muted flex min-h-0 w-full min-w-0 grow flex-col overflow-y-auto">
			{@render children()}
		</main>
	</SideBar.Inset>
</SideBar.Provider>

<AutosaveLeaveGuard />
<PrototypeSwitcher variants={SIDEBAR_VARIANTS} current={sidebarVariant} onChange={chooseVariant} />

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
