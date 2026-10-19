<script lang="ts">
	import { beforeNavigate } from '$app/navigation';
	import { navigating, page } from '$app/state';
	import { Home } from 'lucide-svelte';
	import ConversationCard from '$lib/components/ConversationCard.svelte';
	import NewConversationButton from '$lib/components/NewConversationButton.svelte';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb';
	import { Pager } from '$lib/pagination';
	import { m } from '$lib/paraglide/messages';
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

	let turningPage = $derived(!!navigating.to && isPageTurn(page.url, navigating.to.url));

	// The admin layout scrolls inside <main>, which SvelteKit's own scroll reset doesn't reach.
	// Scrolling on click rather than on arrival shows the page turn has started.
	beforeNavigate(({ from, to }) => {
		if (from && to && isPageTurn(from.url, to.url))
			pageTop?.scrollIntoView({ block: 'start', behavior: 'smooth' });
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
		<Breadcrumb.Root>
			<Breadcrumb.List>
				<Breadcrumb.Item>
					<Breadcrumb.Link href="/admin">{m.workspace()}</Breadcrumb.Link>
				</Breadcrumb.Item>
				<Breadcrumb.Separator />
				<Breadcrumb.Item>Conversations</Breadcrumb.Item>
			</Breadcrumb.List>
		</Breadcrumb.Root>

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

			<div
				class={[
					'grid w-full grid-cols-1 gap-x-2 gap-y-16 transition-opacity',
					turningPage && 'pointer-events-none opacity-50'
				]}
				aria-busy={turningPage}
			>
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
