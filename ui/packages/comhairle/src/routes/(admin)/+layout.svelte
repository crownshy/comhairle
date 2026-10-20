<script lang="ts">
	import AdminSidebar, { type OpenConversation } from './sidebar/AdminSidebar.svelte';
	import { workspaceGroups } from './sidebar/workspaceLinks';
	import { hasSiteAdminRole, hasSuperAdminRole } from '$lib/utils/permissions';
	import type {
		ConversationWithTranslations,
		WorkflowStepsListResponse
	} from '@crownshy/api-client/api';
	import MobileAdminBar from './sidebar/MobileAdminBar.svelte';
	import TourReplayButton from '$lib/tours/TourReplayButton.svelte';
	import { conversationTitle } from '$lib/components/Translation/translationUtils';
	import * as SideBar from '$lib/components/ui/sidebar';
	import { setSidebarWidth } from '$lib/components/sidebarWidthContext.svelte.js';
	import { clampWidth, DEFAULT_WIDTH } from '$lib/components/sidebarWidth.js';
	import type { LayoutProps } from './$types';
	import { page } from '$app/state';
	import { loginRedirectServer } from '$lib/urls';
	import AutosaveLeaveGuard from '$lib/components/Translation/AutosaveLeaveGuard.svelte';

	let { children, data }: LayoutProps = $props();
	// Set by the conversation layout. Inside a conversation the sidebar shows it instead of
	// the workspace links (ADR-0052).
	let openConversation = $derived.by((): OpenConversation | undefined => {
		const conversation = page.data.conversation as ConversationWithTranslations | undefined;
		if (!conversation) return undefined;
		const steps = page.data.workflowSteps as WorkflowStepsListResponse | undefined;
		return {
			conversation,
			stepCount: steps?.length ?? 0,
			hasPolisStep: page.data.hasPolisStep === true
		};
	});
	let mobileTitle = $derived(
		openConversation ? conversationTitle(openConversation.conversation) : 'Comhairle'
	);
	let groups = $derived(
		workspaceGroups({
			isSiteAdmin: hasSiteAdminRole(data.userRoles),
			isSuperAdmin: hasSuperAdminRole(data.userRoles)
		})
	);

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

{#if data.user}
	<SideBar.Provider
		class="bg-admin-background h-svh"
		data-sidebar-tone="light"
		style="--sidebar-width: {sidebarWidthPx}px;"
		data-sidebar-resizing={sidebarWidth.resizing ? '' : undefined}
	>
		<AdminSidebar
			open={openConversation}
			workspaceGroups={groups}
			user={data.user}
			path={page.url.pathname}
		/>
		<SideBar.Inset class="min-h-0 min-w-0 bg-transparent">
			<MobileAdminBar title={mobileTitle} />
			<TourReplayButton />
			<!-- relative keeps absolute children (e.g. sr-only labels) inside the scroll box, so they cannot stretch the page and add a second scrollbar. -->
			<main class="relative flex min-h-0 w-full min-w-0 grow flex-col overflow-y-auto">
				{@render children()}
			</main>
		</SideBar.Inset>
	</SideBar.Provider>
{/if}

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
