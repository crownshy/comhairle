<script lang="ts">
	import { beforeNavigate } from '$app/navigation';
	import { navigating, page } from '$app/state';
	import { ArrowRight } from '@lucide/svelte';
	import ConversationCard from '$lib/components/ConversationCard.svelte';
	import { conversationLandingUrl } from '$lib/utils/conversationLanding';
	import NewConversationButton from '$lib/components/NewConversationButton.svelte';
	import SectionHeader from '$lib/components/SectionHeader.svelte';
	import TabContent from '$lib/components/TabContent.svelte';
	import { Pager } from '$lib/pagination';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import { permissions } from '$lib/permissions.svelte';
	import LayoutToggle from '$lib/components/LayoutToggle.svelte';
	import { conversationCountLabel, conversationNoun, isPageTurn } from './conversationFilters';
	import type { PageProps } from './$types';
	import ConversationSearch from './ConversationSearch.svelte';
	import StatusFilterChips from './StatusFilterChips.svelte';
	import { useTour } from '$lib/tours/useTour.svelte';
	import { conversationsListTour } from './tour';
	import QueryParams, { type ConversationsParams } from '$lib/interfaces/QueryParams.svelte';

	useTour(() => conversationsListTour);

	let { data }: PageProps = $props();

	let total = $derived(data.conversations.total);
	let searchTerm = $derived(data.search.trim());
	let countLabel = $derived(conversationCountLabel(total, data.filter, searchTerm));
	let pageTop: HTMLElement | undefined = $state();

	let turningPage = $derived(!!navigating.to && isPageTurn(page.url, navigating.to.url));

	// The admin layout scrolls inside <main>, which SvelteKit's own scroll reset doesn't reach.
	// Scrolling on click rather than on arrival shows the page turn has started.
	beforeNavigate(({ from, to }) => {
		if (from && to && isPageTurn(from.url, to.url))
			pageTop?.scrollIntoView({ block: 'start', behavior: 'smooth' });
	});
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
	<div class="@container flex w-full flex-col gap-11">
		<div class="flex flex-col gap-4 @4xl:flex-row @4xl:items-center @4xl:justify-between">
			<div data-tour="status-filters">
				<StatusFilterChips active={data.filter} />
			</div>
			<div class="flex items-center gap-3">
				<div data-tour="conversation-search">
					<ConversationSearch />
				</div>
				<LayoutToggle />
			</div>
		</div>
		{#if total > 0}
			<p class="text-muted-foreground text-base" aria-live="polite">{countLabel}</p>
		{/if}
		<div
			data-tour="conversation-list"
			class={[
				QueryParams.get<ConversationsParams>('view') === 'list'
					? 'flex w-full flex-col gap-3'
					: 'grid w-full grid-cols-1 gap-5 md:grid-cols-2 xl:grid-cols-3',
				'transition-opacity',
				turningPage && 'pointer-events-none opacity-50'
			]}
			aria-busy={turningPage}
		>
			{#each data.conversations.records as conversation (conversation.id)}
				<ConversationCard {conversation} for="admin">
					{#snippet Meta()}
						{#if conversation.isShared}<Badge variant="outline">Shared</Badge>{/if}
					{/snippet}
					{#snippet Actions()}
						<Button
							variant="primaryDark"
							class="relative z-10 rounded-full"
							href={conversationLandingUrl(conversation)}
						>
							Open conversation <ArrowRight class="size-4" />
						</Button>
					{/snippet}
				</ConversationCard>
			{:else}
				<p class="text-muted-foreground text-base">
					{#if searchTerm}
						No {conversationNoun(data.filter, 0)} match "{searchTerm}"
					{:else if data.filter === 'all'}
						You don't have any conversations yet.
					{:else}
						No {conversationNoun(data.filter, 0)}.
					{/if}
				</p>
			{/each}
		</div>

		{#if total > data.pageSize}
			<Pager pageSize={data.pageSize} count={total} url={page.url} />
		{/if}
	</div>
</TabContent>
