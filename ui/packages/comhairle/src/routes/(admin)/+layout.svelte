<script lang="ts">
	import AdminNav from './sidebar/AdminNav.svelte';
	import type { StageConversation } from './sidebar/conversationStages';
	import SidebarFloatingTriggers from '$lib/components/SidebarFloatingTriggers.svelte';
	import * as SideBar from '$lib/components/ui/sidebar';
	import { setSidebarWidth } from '$lib/components/sidebarWidthContext.svelte.js';
	import { clampWidth, DEFAULT_WIDTH } from '$lib/components/sidebarWidth.js';
	import type { LayoutProps } from './$types';
	import { page } from '$app/state';
	import { loginRedirectServer } from '$lib/urls';
	import AutosaveLeaveGuard from '$lib/components/Translation/AutosaveLeaveGuard.svelte';
	import { getTextInLocale } from '$lib/components/Translation/translationUtils';
	import type { ConversationWithTranslations } from '@crownshy/api-client/api';

	let { children, data }: LayoutProps = $props();
	let latestConversations = $derived(data.latestConversations);
	let organizations = $derived(data.userOrganizations?.organizations ?? []);
	// Set by the conversation layout. Its presence switches the sidebar to that conversation.
	let conversation = $derived(page.data.conversation as StageConversation | undefined);
	// Same rule as the conversation header: primary-locale translation, falling back to the raw title.
	let conversationTitle = $derived.by(() => {
		const c = page.data.conversation as ConversationWithTranslations | undefined;
		if (!c) return '';
		return getTextInLocale(c.translations?.title, c.primaryLocale ?? 'en', c.title) || c.title;
	});
	type StepTool = { toolConfig?: { type?: string }; previewToolConfig?: { type?: string } };
	let workflowSteps = $derived((page.data.workflowSteps as StepTool[] | undefined) ?? []);
	let stepCount = $derived(workflowSteps.length);
	// Moderation is for Polis steps, whether they are live or still in preview.
	let hasPolisStep = $derived(
		workflowSteps.some(
			(step) => step.toolConfig?.type === 'polis' || step.previewToolConfig?.type === 'polis'
		)
	);
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
</script>

<SideBar.Provider
	class="h-svh"
	style="--sidebar-width: {sidebarWidthPx}px;"
	data-sidebar-resizing={sidebarWidth.resizing ? '' : undefined}
>
	<AdminNav
		user={data.user}
		latestConversations={latestConversations ?? []}
		{organizations}
		{conversation}
		{conversationTitle}
		{stepCount}
		{hasPolisStep}
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
