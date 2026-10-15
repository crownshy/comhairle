<script lang="ts">
	import { afterNavigate } from '$app/navigation';
	import { page } from '$app/state';
	import { Home } from 'lucide-svelte';
	import ConversationCard from '$lib/components/ConversationCard.svelte';
	import NewConversationButton from '$lib/components/NewConversationButton.svelte';
	import { Pager } from '$lib/pagination';
	import { permissions } from '$lib/permissions.svelte';
	import { CONVERSATION_STATUS_FILTER_LABELS, isPageTurn } from './conversationFilters';
	import type { PageProps } from './$types';
	import ConversationSearch from './ConversationSearch.svelte';
	import StatusFilterChips from './StatusFilterChips.svelte';

	let { data }: PageProps = $props();
	let filteredNoun = $derived(
		data.filter === 'all'
			? 'conversations'
			: `${CONVERSATION_STATUS_FILTER_LABELS[data.filter].toLowerCase()} conversations`
	);

	let pageTop: HTMLElement | undefined = $state();

	// The admin layout scrolls inside <main>, which SvelteKit's own scroll reset doesn't reach.
	afterNavigate(({ from, to }) => {
		if (from && to && isPageTurn(from.url, to.url)) pageTop?.scrollIntoView({ block: 'start' });
	});
</script>

<svelte:head>
	<title>Admin Dashboard - Comhairle</title>
</svelte:head>

<div
	bind:this={pageTop}
	class="bg-muted pt-page-top px-gutter flex w-full pb-8 sm:pr-8 sm:pb-12 lg:pr-16"
>
	<div class="@container flex w-full max-w-300 flex-col gap-11">
		<!-- The right padding matches the card padding so the button lines up with the card images. -->
		<div
			class="mb-10 flex flex-col items-start gap-4 @4xl:flex-row @4xl:items-center @4xl:justify-between @4xl:pr-6"
		>
			<div class="flex items-center gap-2">
				<Home class="size-7 sm:size-9" />
				<h1 class="text-2xl sm:text-4xl">Your conversations</h1>
			</div>
			{#if permissions.can('system', 'conversation_create')}
				<NewConversationButton class="w-full sm:w-auto" label="Create New Conversation" />
			{/if}
		</div>
		<div class="flex w-full flex-col gap-11 overflow-y-auto">
			<div
				class="flex flex-col gap-4 @4xl:flex-row @4xl:items-center @4xl:justify-between @4xl:pr-6"
			>
				<StatusFilterChips active={data.filter} />
				<ConversationSearch />
			</div>

			<div class="grid w-full grid-cols-1 gap-x-2 gap-y-16">
				{#each data.conversations.records as conversation (conversation.id)}
					<ConversationCard {conversation} isShared={conversation.isShared} />
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
	</div>
</div>
