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

	/// Ragflow helper chat

	// $effect only runs in the browser, unlike top-level script code which also runs
	// during SSR, where `window` doesn't exist.
	$effect(() => {
		function onMessage(e: MessageEvent) {
			if (e.origin !== 'https://rag.comhairle.scot') return;
			if (e.data.type === 'CREATE_CHAT_WINDOW') {
				if (document.getElementById('chat-win')) return;
				const i = document.createElement('iframe');
				i.id = 'chat-win';
				i.src = e.data.src;
				i.style.cssText =
					'position:fixed;bottom:104px;right:24px;width:380px;height:500px;border:none;background:transparent;z-index:9998;display:none';
				i.frameBorder = '0';
				i.allow = 'microphone;camera';
				document.body.appendChild(i);
			} else if (e.data.type === 'TOGGLE_CHAT') {
				const w = document.getElementById('chat-win');
				if (w) w.style.display = e.data.isOpen ? 'block' : 'none';
			} else if (e.data.type === 'SCROLL_PASSTHROUGH') window.scrollBy(0, e.data.deltaY);
		}
		window.addEventListener('message', onMessage);
		return () => window.removeEventListener('message', onMessage);
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
		{userRoles}
		path={page.url.pathname}
	/>
	<SideBar.Inset class="min-h-0 min-w-0">
		<SidebarFloatingTriggers />
		<main class="bg-muted flex min-h-0 w-full min-w-0 grow flex-col overflow-y-auto">
			{@render children()}
		</main>
	</SideBar.Inset>
</SideBar.Provider>

<AutosaveLeaveGuard />

<iframe
	src="https://rag.comhairle.scot/next-chats/widget?shared_id=b62170c0cba711f18cab2699c84c9b80&from=chat&auth=3YDeffNn2db8emYnsh2I2-5MV7rWKKXh&visible_avatar=1&locale=en&streaming=true&mode=master&streaming=true"
	style="position:fixed;bottom:0;right:0;width:100px;height:100px;border:none;background:transparent;z-index:9999"
	frameborder="0"
	allow="microphone;camera"
></iframe>

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
