<script lang="ts">
	import { beforeNavigate, goto } from '$app/navigation';
	import { navigating, page } from '$app/state';
	import { LayoutGrid, List } from 'lucide-svelte';
	import ConversationCard from '$lib/components/ConversationCard.svelte';
	import NewConversationButton from '$lib/components/NewConversationButton.svelte';
	import SectionHeader from '$lib/components/SectionHeader.svelte';
	import TabContent from '$lib/components/TabContent.svelte';
	import * as ToggleGroup from '$lib/components/ui/toggle-group';
	import { Pager } from '$lib/pagination';
	import { permissions } from '$lib/permissions.svelte';
	import {
		conversationCountLabel,
		conversationNoun,
		isPageTurn,
		listViewFromUrl,
		listViewHref
	} from './conversationFilters';
	import type { PageProps } from './$types';
	import ConversationSearch from './ConversationSearch.svelte';
	import StatusFilterChips from './StatusFilterChips.svelte';

	let { data }: PageProps = $props();

	let view = $derived(listViewFromUrl(page.url));
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

	// In the URL so the choice survives a reload and the pager's links. The load doesn't read
	// `view`, so switching doesn't refetch.
	function showView(next: string) {
		if (next !== 'card' && next !== 'list') return;
		// eslint-disable-next-line svelte/no-navigation-without-resolve -- only swaps the query on this page's URL
		goto(listViewHref(page.url, next), { replaceState: true, noScroll: true, keepFocus: true });
	}

	const VIEW_ITEM_CLASS =
		'text-muted-foreground hover:text-foreground data-[state=on]:bg-card data-[state=on]:text-foreground rounded-full p-2 first:rounded-full last:rounded-full hover:bg-transparent data-[state=on]:shadow-sm';
</script>

<svelte:head>
	<title>Your conversations - Comhairle</title>
</svelte:head>

{#snippet viewSwitch()}
	<ToggleGroup.Root
		type="single"
		value={view}
		onValueChange={showView}
		aria-label="Layout"
		class="bg-muted shrink-0 gap-1 rounded-full p-1"
	>
		<ToggleGroup.Item value="card" aria-label="Card view" class={VIEW_ITEM_CLASS}>
			<LayoutGrid class="size-4" />
		</ToggleGroup.Item>
		<ToggleGroup.Item value="list" aria-label="List view" class={VIEW_ITEM_CLASS}>
			<List class="size-4" />
		</ToggleGroup.Item>
	</ToggleGroup.Root>
{/snippet}

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
			<NewConversationButton class="w-full sm:w-auto" label="Create new conversation" />
		{/if}
	{/snippet}
</SectionHeader>

<TabContent>
	<div class="@container flex w-full flex-col gap-11">
		<div class="flex flex-col gap-4 @4xl:flex-row @4xl:items-center @4xl:justify-between">
			<StatusFilterChips active={data.filter} />
			<div class="flex items-center gap-3">
				<ConversationSearch />
				{@render viewSwitch()}
			</div>
		</div>
		{#if total > 0}
			<p class="text-muted-foreground text-base" aria-live="polite">{countLabel}</p>
		{/if}
		<div
			class={[
				view === 'card'
					? 'grid w-full grid-cols-1 gap-5 md:grid-cols-2 xl:grid-cols-3'
					: 'flex w-full flex-col gap-3',
				'transition-opacity',
				turningPage && 'pointer-events-none opacity-50'
			]}
			aria-busy={turningPage}
		>
			{#each data.conversations.records as conversation (conversation.id)}
				<ConversationCard {conversation} isShared={conversation.isShared} layout={view} />
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
