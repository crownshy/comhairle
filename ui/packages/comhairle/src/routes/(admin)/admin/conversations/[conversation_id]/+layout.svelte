<script lang="ts">
	import { page, navigating } from '$app/state';
	import TabContent from './TabContent.svelte';
	import TabContentSkeleton from './TabContentSkeleton.svelte';
	import TabStripSkeleton from '$lib/components/TabStripSkeleton.svelte';
	import SubTabStrip from '$lib/components/SubTabStrip.svelte';
	import { INVITE_SUBTABS } from './invites/tabs';
	import EventStrip from '$lib/components/EventStrip.svelte';
	import { EVENT_SUBTABS } from './events/[event_id]/tabs';
	import { conversationPrimaryStripSkeleton } from '$lib/utils/conversationTabStrip';
	import { delayedFlag } from '$lib/utils/delayedFlag.svelte';
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

	// Configure renders its own sub-tab strip above a TabContent, so it skips the padded wrapper.
	let isConfigureSection = $derived.by(() => {
		const base = `/admin/conversations/${conversation.id}/configure`;
		const path = page.url.pathname.replace(/\/+$/, '');
		return path === base || path.startsWith(`${base}/`);
	});

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

	// A fast switch (Configure/Setup, a nearby step) resolves in well under 150ms; painting the
	// skeleton for those few frames only reads as a flash. Delay the swap so quick loads keep the
	// current strip + content visible until the destination is ready, and only genuinely slow loads
	// (where the feedback is wanted) ever show the skeleton. Hiding is immediate, so nothing lingers.
	let showSwitchingSkeleton = delayedFlag(() => switchingSection, 150);

	// While switching, reserve the *destination's* strip and content skeletons so the whole
	// content region flips to a loading state once the switch has run long enough to warrant it.
	let effectivePathname = $derived(navigating.to?.url.pathname ?? page.url.pathname);

	// Reserve the injected primary strip's row with a matching skeleton (null = no strip on this
	// route) so a hard refresh doesn't shift the layout. See conversationPrimaryStripSkeleton for why.
	let primaryStripSkeleton = $derived(
		conversationPrimaryStripSkeleton(effectivePathname, conversation.id)
	);
</script>

<!-- Mobile only: the sidebar is a drawer there, so keep the title visible and leave room for
	 the floating menu button. On larger screens the sidebar shows the name and status. -->
<div
	class="border-border bg-background flex w-full shrink-0 items-center border-b py-2 pr-3 pl-14 md:hidden"
>
	<h1 class="text-primary truncate text-lg leading-7 font-semibold" title={displayTitle}>
		{displayTitle}
	</h1>
</div>

<!-- Section sub-strips. The sidebar picks the section (ADR-0052). shrink-0 keeps their
	 height inside the viewport-capped `main`. -->
<div class="flex shrink-0 flex-col">
	<!-- Row 3+ : section sub-strips, all server-rendered here from loaded data / static lists
		 (keyed off the pathname), never injected from a child `$effect`. Once a switch has run long
		 enough to warrant the skeleton we ignore them and show the destination's reserved one. -->
	{#if showSwitchingSkeleton.current}
		{#if primaryStripSkeleton}
			<TabStripSkeleton
				leadingIcon={primaryStripSkeleton.leadingIcon}
				widths={primaryStripSkeleton.widths}
			/>
		{/if}
	{:else if isInvitesSection}
		<SubTabStrip tone="primary" items={INVITE_SUBTABS} defaultValue="email" />
	{:else if isEventsSection}
		<EventStrip conversationId={conversation.id} events={data.events} />
	{/if}
	<!-- Row 4: only the event-detail sub-tabs live here now. -->
	{#if isEventDetailPage && !showSwitchingSkeleton.current}
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
		{#if showSwitchingSkeleton.current}
			<div class="pt-page-top px-gutter">
				<div class="w-full max-w-300">
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
	{#if showSwitchingSkeleton.current}
		<div class="bg-admin-background pt-page-top px-gutter grow pb-8 sm:pr-8 sm:pb-12 lg:pr-16">
			<div class="h-full w-full max-w-300">
				<TabContentSkeleton />
			</div>
		</div>
	{:else}
		{@render children()}
	{/if}
{:else}
	<!-- Mobile: symmetric `px-gutter` so content is evenly inset. Larger screens keep the
		 left gutter for tab alignment and widen the right margin. Top is token-driven. -->
	<div class="bg-admin-background grow">
		{#if showSwitchingSkeleton.current}
			<TabContent>
				<TabContentSkeleton />
			</TabContent>
		{:else if isConfigureSection || isDesignSection}
			{@render children()}
		{:else}
			<TabContent>
				{@render children()}
			</TabContent>
		{/if}
	</div>
{/if}
