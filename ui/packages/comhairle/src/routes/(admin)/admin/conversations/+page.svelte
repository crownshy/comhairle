<script lang="ts">
	import { page } from '$app/state';
	import ArrowRight from '@lucide/svelte/icons/arrow-right';
	import ConversationCard, {
		type ConversationCardLayout
	} from '$lib/components/ConversationCard.svelte';
	import { conversationLandingUrl } from '$lib/utils/conversationLanding';
	import NewConversationButton from '$lib/components/NewConversationButton.svelte';
	import SectionHeader from '$lib/components/SectionHeader.svelte';
	import TabContent from '$lib/components/TabContent.svelte';
	import { Pager } from '$lib/pagination';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import { permissions } from '$lib/permissions.svelte';
	import { isModeratorOnly, moderatorOnlyConversationIds } from '$lib/utils/permissions';
	import LayoutToggle from '$lib/components/LayoutToggle.svelte';
	import { useTour } from '$lib/tours/useTour.svelte';
	import { conversationsListTour } from './tour';
	import QueryParams, { type ConversationsParams } from '$lib/interfaces/QueryParams.svelte';
	import SearchBar from '$lib/components/SearchBar.svelte';
	import { useDebounce } from 'runed';
	import { m } from '$lib/paraglide/messages';
	import { scrollBehavior } from '$lib/utils/reducedMotion';
	import ErrorMessage from '$lib/components/ErrorMessage.svelte';
	import FilterBadges from '$lib/components/FilterBadges.svelte';
	import AdminConversation from './AdminConversation';
	import { capitalise } from '$lib/utils/casingUtils';

	useTour(() => conversationsListTour);

	let { data } = $props();

	// Only for someone who does nothing but moderate. Anyone with another role may also have
	// edit rights through it, so their card opens as usual and the conversation layout decides.
	let moderatorOnlyIds = $derived(
		new Set(isModeratorOnly(data.userRoles) ? moderatorOnlyConversationIds(data.userRoles) : [])
	);
	let view = $derived(
		QueryParams.get<ConversationsParams>('view') as ConversationCardLayout | undefined
	);
	let pageTop: HTMLElement | undefined = $state();

	function scrollToTop() {
		pageTop?.scrollIntoView({ block: 'start', behavior: scrollBehavior() });
	}
</script>

<svelte:head>
	<title>Your conversations - Comhairle</title>
</svelte:head>

<!-- Page-turn scroll target (see beforeNavigate above). -->
<div bind:this={pageTop} aria-hidden="true"></div>

<SectionHeader
	content={{
		title: 'Your conversations',
		description:
			'Conversations you created or that were shared with you. Open one to set it up, run it or see the results.'
	}}
>
	{#snippet actions()}
		{#if permissions.can('system', 'conversation_create')}
			<div data-tour="new-conversation" class="w-full sm:w-auto">
				<NewConversationButton class="w-full sm:w-auto" label="Create new conversation" />
			</div>
		{/if}
	{/snippet}
</SectionHeader>

<TabContent>
	{#if data.conversations.err !== null}
		{console.error(data.conversations.err)}
		<ErrorMessage>Could not load conversations, please try again</ErrorMessage>
	{:else}
		<div class="@container flex w-full flex-col">
			<div
				class="mb-11 flex flex-col gap-4 @4xl:flex-row @4xl:items-center @4xl:justify-between"
			>
				<div data-tour="status-filters">
					<FilterBadges
						param="filter"
						options={AdminConversation.filters.map((filter) => [
							filter,
							capitalise(filter)
						])}
						label="Filter conversations"
						allLabel="All"
					/>
				</div>
				<div class="flex items-center gap-3">
					<SearchBar
						data-tour="conversation-search"
						value={QueryParams.get<ConversationsParams>('search') as string | undefined}
						oninput={useDebounce((e) => {
							QueryParams.set<ConversationsParams>(
								'search',
								(e.target as HTMLInputElement).value
							)
								.set('page', '1')
								.update();
						})}
						placeholder={m.conversations_search()}
						aria-label="Search conversations"
						class="bg-card h-10 w-full @4xl:w-80"
					/>
					<LayoutToggle />
				</div>
			</div>
			{#if data.conversations.ok.total > 0}
				<p class="text-muted-foreground mb-3 text-base" aria-live="polite">
					{data.conversations.ok.total} conversation(s) found
				</p>
			{/if}
			<div
				data-tour="conversation-list"
				class={[
					view === 'list' ? 'flex w-full flex-col gap-3' : 'dynamic-grid w-full gap-6',
					'transition-opacity'
				]}
				style="--grid-item-width: 19rem;"
			>
				{#each data.conversations.ok.records as conversation (conversation.id)}
					<ConversationCard {conversation} for="admin" layout={view}>
						{#snippet Meta()}
							{#if conversation.isShared}
								<Badge variant="outline">Shared</Badge>
							{/if}
						{/snippet}
						{#snippet Actions()}
							<Button
								variant="primaryDark"
								class="relative z-10 rounded-full"
								href={conversationLandingUrl(
									conversation,
									moderatorOnlyIds.has(conversation.id) ? 'moderation' : 'full'
								)}
							>
								Open conversation <ArrowRight class="size-4" />
							</Button>
						{/snippet}
					</ConversationCard>
				{:else}
					<p class="text-muted-foreground text-base">No conversations found</p>
				{/each}
			</div>

			{#if data.conversations.ok.total > data.PAGE_SIZE}
				<Pager
					pageSize={data.PAGE_SIZE}
					count={data.conversations.ok.total}
					url={page.url}
					onNext={() => void scrollToTop()}
					onPrev={() => void scrollToTop()}
				/>
			{/if}
		</div>
	{/if}
</TabContent>
