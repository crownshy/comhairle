<script lang="ts">
	import { m } from '$lib/paraglide/messages';
	import ConversationCard from '$lib/components/ConversationCard.svelte';
	import { Pager } from '$lib/pagination';
	import { page } from '$app/state';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import ErrorMessage from '$lib/components/ErrorMessage.svelte';
	import SearchBar from '$lib/components/SearchBar.svelte';
	import QueryParams, { type ConversationsParams } from '$lib/interfaces/QueryParams.svelte';
	import { useDebounce } from 'runed';
	import { Button } from '$lib/components/ui/button';
	import ArrowDown from '@lucide/svelte/icons/arrow-down';
	import Info from '@lucide/svelte/icons/info';
	import FilterBadges from '$lib/components/FilterBadges.svelte';
	import Conversation from '$lib/interfaces/Conversation';
	import type { LocalizedConversationDto } from '@crownshy/api-client/api';
	import HomeEmptyState from './HomeEmptyState.svelte';
	import PageHero from '$lib/components/PageHero.svelte';

	let { data } = $props();

	let search = $derived(QueryParams.get<ConversationsParams>('search'));
	let filter = $derived(QueryParams.get<ConversationsParams>('filter'));
	let total = $derived(data.conversations.ok?.total);
	// No conversations at all, as opposed to a search or filter that matched none.
	let isEmpty = $derived(total === 0 && !search && !filter);
	let nothingMatches = $derived(total === 0 && !isEmpty);

	function statusLabel(conversation: LocalizedConversationDto) {
		const status = Conversation.getStatus(conversation);
		if (status === 'completed') return m.conversations_filter_ended();
		if (status === 'live') return m.conversations_filter_open();
		return undefined;
	}

	function clearSearch() {
		QueryParams.remove<ConversationsParams>('search').remove('page').update();
	}

	function clearSearchAndFilters() {
		QueryParams.remove<ConversationsParams>('search').remove('filter').remove('page').update();
	}

	// Glide down to the list rather than jump; people who ask for reduced motion jump straight
	// there. Focus moves to the list heading so keyboard and screen reader users land there too.
	function scrollToList(event: MouseEvent) {
		const list = document.getElementById('conversations');
		if (!list) return;
		event.preventDefault();
		const reduceMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
		list.scrollIntoView({ behavior: reduceMotion ? 'auto' : 'smooth', block: 'start' });
		list.focus({ preventScroll: true });
	}
</script>

<svelte:head>
	<title>Comhairle</title>
</svelte:head>

<div class="flex flex-col gap-14 pb-24 md:gap-20">
	<PageHero>
		<div class="flex max-w-3xl flex-col gap-4">
			{#if data.isCommunity}
				<h1
					class="scroll-m-20 text-[1.75rem] leading-tight font-bold text-balance sm:text-4xl lg:text-[2.625rem]"
				>
					{m.welcome_to_the()}

					<Tooltip.Provider delayDuration={0}>
						<Tooltip.Root>
							<Tooltip.Trigger
								class="inline-flex cursor-default items-baseline gap-2"
								onclick={(e) => e.preventDefault()}
							>
								Comhairle<Info class="text-foreground inline size-5 self-center" />
							</Tooltip.Trigger>
							<Tooltip.Content side="top" class="text-base">
								{m.comhairle_meaning()}
							</Tooltip.Content>
						</Tooltip.Root>
					</Tooltip.Provider>

					{m.community_server()}
				</h1>

				{#each [m.community_server_description_1(), m.community_server_description_2()] as description (description)}
					<p class="text-lg text-pretty sm:text-xl">
						{description}
					</p>
				{/each}
			{:else}
				<h1
					class="scroll-m-20 text-[1.75rem] leading-tight font-bold text-balance sm:text-4xl lg:text-[2.625rem]"
				>
					{m.home_hero_title()}
				</h1>
				<p class="text-lg text-pretty sm:text-xl">
					{m.home_hero_subtitle()}
				</p>
			{/if}
		</div>
		{#if !isEmpty}
			<Button
				href="#conversations"
				onclick={scrollToList}
				class="h-12 w-full gap-2 rounded-full px-6 text-base font-semibold sm:w-auto"
			>
				{m.explore_conversations()}
				<ArrowDown class="size-5" />
			</Button>
		{/if}
	</PageHero>

	<div class="mx-auto w-full max-w-[1300px] px-4 md:px-8">
		<div class="mx-auto w-full max-w-6xl">
			{#if data.conversations.err !== null}
				<ErrorMessage>{m.conversations_loading_error()}</ErrorMessage>
			{:else if isEmpty}
				<HomeEmptyState />
			{:else}
				<section aria-labelledby="conversations" class="flex flex-col gap-6 md:gap-8">
					<div class="flex flex-wrap items-baseline justify-between gap-x-4 gap-y-1">
						<h2
							id="conversations"
							tabindex="-1"
							class="scroll-mt-28 text-2xl font-bold outline-none sm:text-3xl"
						>
							{search
								? m.conversations_results_for({ search })
								: m.conversations_all_heading()}
						</h2>
						{#if search}
							<button
								type="button"
								onclick={clearSearch}
								class="text-primary text-base font-semibold underline"
							>
								{m.conversations_clear_search()}
							</button>
						{/if}
					</div>

					<div class="flex flex-col gap-4">
						<div class="max-w-2xl">
							<SearchBar
								placeholder={m.home_search_placeholder()}
								aria-label={m.conversations_search()}
								iconClass="ml-5 size-5"
								value={search as string}
								oninput={useDebounce((event) => {
									QueryParams.set<ConversationsParams>(
										'search',
										(event.target as HTMLInputElement).value
									)
										.set<ConversationsParams>('page', 0)
										.update();
								}, 300)}
								class="bg-card border-border h-14 rounded-full pl-13 text-base shadow-none md:text-base"
							/>
						</div>
						<FilterBadges
							param="filter"
							label={m.conversations_filter_label()}
							allLabel={m.conversations_filter_all()}
							options={[
								['open', m.conversations_filter_open()],
								['ended', m.conversations_filter_ended()]
							]}
						/>
					</div>

					<p class="sr-only" aria-live="polite">
						{#if total === 1}
							{m.conversations_found_one()}
						{:else}
							{m.conversations_found_count({ count: total ?? 0 })}
						{/if}
					</p>

					{#if nothingMatches}
						<div
							class="bg-card border-border flex flex-col items-start gap-2 rounded-2xl border p-5"
						>
							<h3 class="text-lg font-bold">{m.conversations_nothing_matches()}</h3>
							<p class="text-muted-foreground text-base">
								{m.conversations_nothing_matches_hint()}
							</p>
							<Button
								variant="outline"
								class="mt-2 rounded-full text-base"
								onclick={clearSearchAndFilters}
							>
								{m.conversations_clear_filters()}
							</Button>
						</div>
					{:else if data.conversations.ok}
						<div class="grid gap-8 sm:grid-cols-2 lg:grid-cols-3 lg:gap-y-10">
							{#each data.conversations.ok.records as conversation (conversation.id)}
								<ConversationCard
									{conversation}
									for="public"
									statusLabel={statusLabel(conversation)}
								/>
							{/each}
						</div>

						{#if data.conversations.ok.total > data.CONVERSATION_PAGE_SIZE}
							<div class="self-center">
								<Pager
									pageSize={data.CONVERSATION_PAGE_SIZE}
									count={data.conversations.ok.total}
									url={page.url}
								/>
							</div>
						{/if}
					{/if}
				</section>
			{/if}
		</div>
	</div>
</div>
