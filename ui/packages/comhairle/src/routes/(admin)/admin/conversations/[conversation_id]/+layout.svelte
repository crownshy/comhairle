<script lang="ts">
	import { page, navigating } from '$app/state';
	import TabContentSkeleton from './TabContentSkeleton.svelte';
	import { Button } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { ArrowUpRight, MoreHorizontal, Eye, ExternalLink, Check, CircleX } from 'lucide-svelte';
	import LaunchConversationModal from '$lib/components/LaunchConversationModal.svelte';
	import EndConversationModal from '$lib/components/EndConversationModal.svelte';
	import ConversationTabs from '$lib/components/ConversationTabs.svelte';
	import TabStripSkeleton from '$lib/components/TabStripSkeleton.svelte';
	import WorkflowStepStrip from '$lib/components/WorkflowStepStrip.svelte';
	import ConfigureTabStrip from './configure/ConfigureTabStrip.svelte';
	import { CONFIGURE_TABS } from './configure/tabs';
	import SubTabStrip from '$lib/components/SubTabStrip.svelte';
	import { INVITE_SUBTABS } from './invites/tabs';
	import EventStrip from '$lib/components/EventStrip.svelte';
	import { EVENT_SUBTABS } from './events/[event_id]/tabs';
	import { addStepDialog } from '$lib/stores/addStepDialog.svelte';
	import { conversationPrimaryStripSkeleton } from '$lib/utils/conversationTabStrip';
	import { getTextInLocale } from '$lib/components/Translation/translationUtils';

	let { data, children } = $props();

	let conversation = $derived(data.conversation);
	let displayTitle = $derived(
		getTextInLocale(
			conversation.translations?.title,
			conversation.primaryLocale ?? 'en',
			conversation.title
		) || conversation.title
	);
	let endModalOpen = $state(false);
	let launchModalOpen = $state(false);

	// Only the design board itself is full-bleed (it renders its own white card + palette
	// + scroll region). Every other tab — including the design step config pages under
	// /design/step/* — keeps the padded, max-width reading column.
	let isDesignBoard = $derived(
		page.url.pathname.replace(/\/+$/, '') === `/admin/conversations/${conversation.id}/design`
	);

	// A single workflow step (/design/step/*) owns its own content region: its layout renders
	// a full-bleed sub-tab strip (Row 4) flush under Row 3, then its own padded reading column.
	// So we render its children bare here rather than wrapping them in the padded column below.
	let isStepPage = $derived(
		page.url.pathname
			.replace(/\/+$/, '')
			.startsWith(`/admin/conversations/${conversation.id}/design/step/`)
	);

	// Configure's sub-tabs are a static list (its `?tab=` sections share one form + load), so we
	// server-render the strip here from CONFIGURE_TABS, the same way the workflow step strip is
	// rendered from data, rather than the page injecting it via a client `$effect`.
	let isConfigureSection = $derived(
		page.url.pathname.replace(/\/+$/, '') ===
			`/admin/conversations/${conversation.id}/configure`
	);

	// Recruit (invites) is the same shape as Configure: a static `?subtab=` strip over one page,
	// so we server-render it here from INVITE_SUBTABS instead of a client `$effect`.
	let isInvitesSection = $derived(
		page.url.pathname.replace(/\/+$/, '') === `/admin/conversations/${conversation.id}/invites`
	);

	// The Events section shows the events strip (Row 3) on every /events* page, rendered from
	// `data.events` like the workflow step strip. Event *detail* pages additionally show a static
	// `?subtab=` strip (Row 4). Both are server-rendered here rather than injected via `$effect`.
	let eventsBase = $derived(`/admin/conversations/${conversation.id}/events`);
	let isEventsSection = $derived.by(() => {
		const path = page.url.pathname.replace(/\/+$/, '');
		return path === eventsBase || path.startsWith(`${eventsBase}/`);
	});
	// A single event: /events/<id> (not the list, not /events/new). Its detail sub-tabs are the
	// only thing on Row 4 here now.
	let isEventDetailPage = $derived.by(() => {
		const path = page.url.pathname.replace(/\/+$/, '');
		return path.startsWith(`${eventsBase}/`) && path !== `${eventsBase}/new`;
	});

	// The whole Workflow section (the board and its /design/step/* pages) shows the workflow
	// step strip. We render it here from `data.workflowSteps` (loaded by this layout) so it's
	// server-rendered, rather than injected by the design layout's client `$effect`.
	let isDesignSection = $derived.by(() => {
		const base = `/admin/conversations/${conversation.id}/design`;
		const path = page.url.pathname.replace(/\/+$/, '');
		return path === base || path.startsWith(`${base}/`);
	});

	// A workflow step's sub-tabs (Configure/Setup/Moderation/Insights) are real routes, so
	// navigating between them changes the pathname. That's *not* a section switch: Row 3, Row 4
	// and the content shell all stay mounted while only the inner page swaps. Collapsing the
	// sub-tab segment off a step path lets `switchingSection` tell the two apart.
	function sectionKey(pathname: string): string {
		return pathname
			.replace(/\/+$/, '')
			.replace(/(\/design\/step\/[^/]+)\/(configure|setup|moderation|insights)$/, '$1');
	}

	// A pending navigation to a *different* section or step (its `sectionKey` changes, unlike
	// Configure's `?tab=` sub-tabs which only swap the query, or a step's sub-tab routes which
	// share a key). `page` still reflects the old route until the load resolves, so this is how
	// we know a throttled tab switch is in flight.
	let switchingSection = $derived(
		!!navigating.to && sectionKey(navigating.to.url.pathname) !== sectionKey(page.url.pathname)
	);

	// While switching, reserve the *destination's* strip and content skeletons so the whole
	// content region flips to a loading state the instant the tab is clicked.
	let effectivePathname = $derived(navigating.to?.url.pathname ?? page.url.pathname);

	// Reserve the injected primary strip's row with a matching skeleton (null = no strip on this
	// route) so a hard refresh doesn't shift the layout. See conversationPrimaryStripSkeleton for why.
	let primaryStripSkeleton = $derived(
		conversationPrimaryStripSkeleton(effectivePathname, conversation.id)
	);
</script>

<!-- Row 1: conversation title + launch controls -->
<div
	class="border-border bg-background md:pl-gutter flex w-full shrink-0 items-center justify-between border-b py-2 pr-3 pl-14 md:pr-6"
>
	<h1
		class="text-primary max-w-[22ch] truncate text-lg leading-7 font-semibold sm:max-w-[40ch]"
		title={displayTitle}
	>
		{displayTitle}
	</h1>

	<!-- Mobile actions: single more-menu -->
	<div class="flex shrink-0 items-center lg:hidden">
		<DropdownMenu.Root>
			<DropdownMenu.Trigger
				class="bg-primary/20 hover:bg-primary/30 inline-flex size-9 items-center justify-center rounded-full"
				aria-label="Actions"
			>
				<MoreHorizontal class="size-4" />
			</DropdownMenu.Trigger>
			<DropdownMenu.Content align="end" class="w-56">
				<DropdownMenu.Item>
					<a
						href={`/conversations/${conversation.id}/preview`}
						target="_blank"
						class="flex w-full items-center gap-2"
					>
						<Eye class="size-4" />
						Preview
					</a>
				</DropdownMenu.Item>
				{#if conversation.isLive}
					<DropdownMenu.Item>
						<a
							href={`/conversations/${conversation.id}`}
							class="flex w-full items-center gap-2"
						>
							<ExternalLink class="size-4" />
							Live Conversation Link
						</a>
					</DropdownMenu.Item>
					<DropdownMenu.Separator />
					{#if !conversation.isComplete}
						<DropdownMenu.Item
							class="text-destructive focus:text-destructive focus:bg-destructive/10 hover:text-destructive! hover:bg-destructive/20!"
							onclick={() => (endModalOpen = true)}
						>
							<CircleX class="text-destructive size-4" />
							End Conversation
						</DropdownMenu.Item>
					{:else}
						<DropdownMenu.Item onclick={() => (endModalOpen = true)}>
							<Check class="size-4" />
							Re-open Conversation
						</DropdownMenu.Item>
					{/if}
				{:else}
					<DropdownMenu.Separator />
					<DropdownMenu.Item onclick={() => (launchModalOpen = true)}>
						<ArrowUpRight class="size-4" />
						Launch Conversation
					</DropdownMenu.Item>
				{/if}
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</div>

	<!-- Desktop actions -->
	<div class="hidden shrink-0 items-center gap-4 md:flex">
		<Button
			href={`/conversations/${conversation.id}/preview`}
			target="_blank"
			variant="secondary"
			class="bg-primary/20 text-foreground hover:bg-primary/30 inline-flex h-10 rounded-full px-4 text-sm"
		>
			Preview
			<ArrowUpRight class="size-4" />
		</Button>

		{#if conversation.isLive}
			{#if !conversation.isComplete}
				<span
					class="bg-primary text-primary-foreground inline-flex h-10 items-center gap-2 rounded-full py-1 pr-1 pl-5 text-sm font-medium"
				>
					Launched
					<span
						class="bg-primary-foreground text-primary inline-flex size-8 items-center justify-center rounded-full"
					>
						<Check class="size-4" strokeWidth={3} />
					</span>
				</span>
			{/if}

			<DropdownMenu.Root>
				<DropdownMenu.Trigger
					class="bg-primary/20 hover:bg-primary/30 inline-flex size-10 items-center justify-center rounded-full"
					aria-label="More actions"
				>
					<MoreHorizontal class="size-4" />
				</DropdownMenu.Trigger>
				<DropdownMenu.Content align="end" class="w-56">
					<DropdownMenu.Item>
						<a
							href={`/conversations/${conversation.id}`}
							class="flex w-full items-center gap-2"
						>
							<ExternalLink class="size-4" />
							Live Conversation Link
						</a>
					</DropdownMenu.Item>
					<DropdownMenu.Separator />
					{#if !conversation.isComplete}
						<DropdownMenu.Item
							class="text-destructive focus:text-destructive focus:bg-destructive/10 hover:text-destructive! hover:bg-destructive/20!"
							onclick={() => (endModalOpen = true)}
						>
							<CircleX class="text-destructive size-4" />
							End Conversation
						</DropdownMenu.Item>
					{:else}
						<DropdownMenu.Item onclick={() => (endModalOpen = true)}>
							<Check class="size-4" />
							Re-open Conversation
						</DropdownMenu.Item>
					{/if}
				</DropdownMenu.Content>
			</DropdownMenu.Root>
		{:else}
			<Button variant="default" class="h-[40px]" onclick={() => (launchModalOpen = true)}>
				Launch Conversation
			</Button>
		{/if}
	</div>
</div>

<!-- Modals (triggered programmatically from mobile dropdown or desktop UI) -->
<EndConversationModal {conversation} bind:open={endModalOpen} hideTrigger />
<LaunchConversationModal
	conversation_id={conversation.id}
	bind:open={launchModalOpen}
	hideTrigger
/>

<!-- Rows 2–4: section tabs + injected sub-strips. shrink-0 so these fixed headers keep
	 their height inside the viewport-capped `main` (otherwise flexbox compresses them). -->
<div class="flex shrink-0 flex-col">
	<!-- Row 2: section tabs -->
	<ConversationTabs conversationId={conversation.id} conversationIsLive={conversation.isLive} />

	<!-- Row 3+ : section sub-strips, all server-rendered here from loaded data / static lists
		 (keyed off the pathname), never injected from a child `$effect`. While switching sections
		 we ignore them and show the destination's reserved skeleton instead. -->
	{#if switchingSection}
		{#if primaryStripSkeleton}
			<TabStripSkeleton
				leadingIcon={primaryStripSkeleton.leadingIcon}
				widths={primaryStripSkeleton.widths}
			/>
		{/if}
	{:else if isDesignSection}
		<WorkflowStepStrip
			conversationId={conversation.id}
			steps={data.workflowSteps}
			onAddStep={() => (addStepDialog.open = true)}
		/>
	{:else if isConfigureSection}
		<ConfigureTabStrip tabs={CONFIGURE_TABS} />
	{:else if isInvitesSection}
		<SubTabStrip tone="primary" items={INVITE_SUBTABS} defaultValue="email" />
	{:else if isEventsSection}
		<EventStrip conversationId={conversation.id} events={data.events} />
	{:else if primaryStripSkeleton}
		<TabStripSkeleton
			leadingIcon={primaryStripSkeleton.leadingIcon}
			widths={primaryStripSkeleton.widths}
		/>
	{/if}
	<!-- Row 4: only the event-detail sub-tabs live here now. -->
	{#if isEventDetailPage && !switchingSection}
		<SubTabStrip items={EVENT_SUBTABS} defaultValue="details" />
	{/if}

	{#if conversation.isComplete}
		<div class="border-destructive/20 bg-destructive/10 pl-gutter border-b py-2 pr-5">
			<p class="text-destructive text-sm">This conversation has closed</p>
		</div>
	{/if}
</div>

{#if isDesignBoard}
	<div class="bg-card flex min-h-0 grow flex-col overflow-hidden">
		{#if switchingSection}
			<div class="pt-page-top px-gutter">
				<div class="w-full max-w-[1200px]">
					<TabContentSkeleton />
				</div>
			</div>
		{:else}
			{@render children()}
		{/if}
	</div>
{:else if isStepPage}
	<!-- The step layout renders Row 4 + its own padded column; while switching in, stand in with
		 the same padded skeleton so the region doesn't collapse before the step load resolves. -->
	{#if switchingSection}
		<div class="bg-muted pt-page-top px-gutter grow pb-8 sm:pr-8 sm:pb-12 lg:pr-16">
			<div class="h-full w-full max-w-[1200px]">
				<TabContentSkeleton />
			</div>
		</div>
	{:else}
		{@render children()}
	{/if}
{:else}
	<!-- Mobile: symmetric `px-gutter` so content is evenly inset. Larger screens keep the
		 left gutter for tab alignment and widen the right margin. Top is token-driven. -->
	<div class="bg-muted pt-page-top px-gutter grow pb-8 sm:pr-8 sm:pb-12 lg:pr-16">
		<div class="h-full w-full max-w-[1200px]">
			{#if switchingSection}
				<TabContentSkeleton />
			{:else}
				{@render children()}
			{/if}
		</div>
	</div>
{/if}
