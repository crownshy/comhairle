<script lang="ts">
	import ConversationNav from './sidebar/ConversationNav.svelte';
	import WorkspaceTopBar from './workspace/WorkspaceTopBar.svelte';
	import type { ConversationWithTranslations } from '@crownshy/api-client/api';
	import SidebarFloatingTriggers from '$lib/components/SidebarFloatingTriggers.svelte';
	import * as SideBar from '$lib/components/ui/sidebar';
	import { setSidebarWidth } from '$lib/components/sidebarWidthContext.svelte.js';
	import { clampWidth, DEFAULT_WIDTH } from '$lib/components/sidebarWidth.js';
	import type { LayoutProps } from './$types';
	import { page } from '$app/state';
	import { loginRedirectServer } from '$lib/urls';
	import AutosaveLeaveGuard from '$lib/components/Translation/AutosaveLeaveGuard.svelte';

	let { children, data }: LayoutProps = $props();
	// Set by the conversation layout. Only conversation pages have a sidebar (ADR-0052).
	let conversation = $derived(page.data.conversation as ConversationWithTranslations | undefined);
	let stepCount = $derived((page.data.workflowSteps as unknown[] | undefined)?.length ?? 0);

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

{#if conversation}
	<SideBar.Provider
		class="h-svh"
		style="--sidebar-width: {sidebarWidthPx}px;"
		data-sidebar-resizing={sidebarWidth.resizing ? '' : undefined}
	>
		<ConversationNav {conversation} {stepCount} user={data.user} path={page.url.pathname} />
		<SideBar.Inset class="min-h-0 min-w-0">
			<SidebarFloatingTriggers />
			<main class="bg-muted flex min-h-0 w-full min-w-0 grow flex-col overflow-y-auto">
				{@render children()}
			</main>
		</SideBar.Inset>
	</SideBar.Provider>
{:else}
	<div class="flex h-svh flex-col">
		<WorkspaceTopBar user={data.user} path={page.url.pathname} />
		<main class="bg-muted flex min-h-0 w-full min-w-0 grow flex-col overflow-y-auto">
			<div class="mx-auto flex w-full max-w-336 grow flex-col">
				{@render children()}
			</div>
		</main>
	</div>
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
