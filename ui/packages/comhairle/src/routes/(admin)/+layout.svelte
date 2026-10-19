<script lang="ts">
	import AdminSidebar, { type OpenConversation } from './sidebar/AdminSidebar.svelte';
	import { workspaceGroups } from './sidebar/workspaceLinks';
	import { hasSiteAdminRole, hasSuperAdminRole } from '$lib/utils/permissions';
	import type {
		ConversationWithTranslations,
		WorkflowStepsListResponse
	} from '@crownshy/api-client/api';
	import SidebarFloatingTriggers from '$lib/components/SidebarFloatingTriggers.svelte';
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
</script>

{#if data.user}
	<SideBar.Provider
		class="h-svh"
		style="--sidebar-width: {sidebarWidthPx}px;"
		data-sidebar-resizing={sidebarWidth.resizing ? '' : undefined}
	>
		<AdminSidebar
			open={openConversation}
			workspaceGroups={groups}
			user={data.user}
			path={page.url.pathname}
		/>
		<SideBar.Inset class="min-h-0 min-w-0">
			<SidebarFloatingTriggers />
			<!-- relative keeps absolute children (e.g. sr-only labels) inside the scroll box, so they cannot stretch the page and add a second scrollbar. -->
			<main
				class="bg-muted relative flex min-h-0 w-full min-w-0 grow flex-col overflow-y-auto"
			>
				{@render children()}
			</main>
		</SideBar.Inset>
	</SideBar.Provider>
{/if}

<AutosaveLeaveGuard />

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
