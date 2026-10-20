<script lang="ts">
	import { beforeNavigate } from '$app/navigation';
	import { navigating, page } from '$app/state';
	import ConversationCard from '$lib/components/ConversationCard.svelte';
	import NewConversationButton from '$lib/components/NewConversationButton.svelte';
	import { Pager } from '$lib/pagination';
	import { permissions } from '$lib/permissions.svelte';
	import { CONVERSATION_STATUS_FILTER_LABELS, isPageTurn } from './conversationFilters';
	import type { PageProps } from './$types';
	import ConversationSearch from './ConversationSearch.svelte';
	import StatusFilterChips from './StatusFilterChips.svelte';
	import SectionHeader from '$lib/components/SectionHeader.svelte';
	import TabContent from '$lib/components/TabContent.svelte';
	import { LayoutGrid, List } from 'lucide-svelte';
	let view = $state<'cards' | 'list'>('cards');
	let { data }: PageProps = $props();
	let filteredNoun = $derived(
		data.filter === 'all'
			? 'conversations'
			: `${CONVERSATION_STATUS_FILTER_LABELS[data.filter].toLowerCase()} conversations`
	);
	let total = $derived(data.conversations.total);
	let searchTerm = $derived(data.search.trim());
	let countLabel = $derived.by(() => {
		const noun = total === 1 ? filteredNoun.replace(/s$/, '') : filteredNoun;
		if (searchTerm)
			return `${total} ${noun} ${total === 1 ? 'matches' : 'match'} "${searchTerm}"`;
		if (data.filter === 'all') return `You have ${total} ${noun}`;
		return `${total} ${noun}`;
	});
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

{#snippet viewSwitch()}
	<div class="bg-muted flex shrink-0 gap-1 rounded-full p-1" role="group" aria-label="Layout">
		<button
			type="button"
			aria-label="Card view"
			aria-pressed={view === 'cards'}
			onclick={() => (view = 'cards')}
			class="rounded-full p-2 transition-colors {view === 'cards'
				? 'bg-card text-foreground shadow-sm'
				: 'text-muted-foreground hover:text-foreground'}"
		>
			<LayoutGrid class="size-4" />
		</button>
		<button
			type="button"
			aria-label="List view"
			aria-pressed={view === 'list'}
			onclick={() => (view = 'list')}
			class="rounded-full p-2 transition-colors {view === 'list'
				? 'bg-card text-foreground shadow-sm'
				: 'text-muted-foreground hover:text-foreground'}"
		>
			<List class="size-4" />
		</button>
	</div>
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
				view === 'cards'
					? 'grid w-full grid-cols-1 gap-5 md:grid-cols-2 xl:grid-cols-3'
					: 'flex w-full flex-col gap-3',
				'transition-opacity',
				turningPage && 'pointer-events-none opacity-50'
			]}
			aria-busy={turningPage}
		>
			{#each data.conversations.records as conversation (conversation.id)}
				<ConversationCard
					{conversation}
					isShared={conversation.isShared}
					layout={view === 'cards' ? 'card' : 'row'}
				/>
			{:else}
				<p class="text-muted-foreground text-base">
					{#if data.search.trim()}
						No {filteredNoun} match "{data.search.trim()}"
					{:else if data.filter === 'all'}
						You don't have any conversations yet.
					{:else}
						No {filteredNoun}.
					{/if}
				</p>
			{/each}
		</div>

		{#if data.conversations.total > data.pageSize}
			<Pager pageSize={data.pageSize} count={data.conversations.total} url={page.url} />
		{/if}
	</div>
</TabContent>
