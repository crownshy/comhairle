<script lang="ts">
	import { m } from '$lib/paraglide/messages';
	import ConversationCard, {
		type ConversationCardLayout
	} from '$lib/components/ConversationCard.svelte';
	import { Pager } from '$lib/pagination';
	import { page } from '$app/state';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import ErrorMessage from '$lib/components/ErrorMessage.svelte';
	import SearchBar from '$lib/components/SearchBar.svelte';
	import QueryParams, { type ConversationsParams } from '$lib/interfaces/QueryParams.svelte';
	import { useDebounce } from 'runed';
	import ArrowDown from '@lucide/svelte/icons/arrow-down';
	import ArrowRight from '@lucide/svelte/icons/arrow-right';
	import Info from '@lucide/svelte/icons/info';
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import FilterBadges from '$lib/components/FilterBadges.svelte';
	import LayoutToggle from '$lib/components/LayoutToggle.svelte';
	import FeaturedConversations from '$lib/components/FeaturedConversations.svelte';
	import type { LocalizedConversationDto } from '@crownshy/api-client/api';
	import { resolve } from '$app/paths';
	import HomeEmptyState from './HomeEmptyState.svelte';

	let { data } = $props();

	let view = $derived(
		QueryParams.get<ConversationsParams>('view') as ConversationCardLayout | undefined
	);
	let search = $derived(QueryParams.get<ConversationsParams>('search'));
	let filter = $derived(QueryParams.get<ConversationsParams>('filter'));
	let total = $derived(data.conversations.ok?.total);
	// No conversations at all, as opposed to a search or filter that matched none.
	let isEmpty = $derived(total === 0 && !search && !filter);
	let nothingMatches = $derived(total === 0 && !isEmpty);

	function clearSearchAndFilters() {
		QueryParams.remove<ConversationsParams>('search').remove('filter').remove('page').update();
	}

	// TODO: Fix later
	const featured = $derived(
		QueryParams.get<ConversationsParams>('page') === '1' &&
			!QueryParams.get<ConversationsParams>('search')
			? data.conversations.ok?.records.filter((c) => !c.isComplete).slice(0, 3)
			: []
	);

	function cardHref(conversation: LocalizedConversationDto) {
		const conversation_id = conversation.slug || conversation.id;
		return conversation.isComplete
			? resolve('/(public)/conversations/[conversation_id]/[[preview]]/report', {
					conversation_id
				})
			: resolve('/(public)/conversations/[conversation_id]', { conversation_id });
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

<!-- No minimum height: the conversation list follows straight after the welcome text,
	so it shows in the first screen. -->
<div class="flex flex-col">
	<!-- A light gradient from each theme's accent colour, with two see-through white shapes for
		texture. Text is dark, so it reads at WCAG AA on the lightest and darkest parts. -->
	<section
		class="home-hero text-foreground relative isolate overflow-hidden rounded-b-4xl md:rounded-b-[3rem]"
	>
		<div aria-hidden="true" class="home-hero-shape home-hero-shape-a"></div>
		<div aria-hidden="true" class="home-hero-shape home-hero-shape-b"></div>
		<div
			class="mx-auto flex w-full max-w-7xl flex-col items-start gap-8 px-6 pt-16 pb-16 md:pb-28"
		>
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
									Comhairle<Info
										class="text-foreground inline size-5 self-center"
									/>
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

				{#if data.conversations.ok !== null && data.conversations.ok.records.length > 0}
					<Button
						href="#conversations"
						onclick={scrollToList}
						class="h-12 w-full gap-2 rounded-full px-6 text-base font-semibold sm:w-auto sm:self-start"
					>
						{m.explore_conversations()}
						<ArrowDown class="size-5" />
					</Button>
				{/if}
			</div>
		</div>
	</section>
	{#if isEmpty}
		<div class="mx-auto w-full max-w-7xl px-6 py-10 md:py-20">
			<HomeEmptyState />
		</div>
	{:else}
		<div class="mx-auto w-full max-w-7xl -translate-y-1/2 px-6">
			<div class="bg-card mx-auto max-w-4xl rounded-full p-3 shadow-lg">
				<SearchBar
					placeholder={m.conversations_search()}
					aria-label={m.conversations_search()}
					value={QueryParams.get<ConversationsParams>('search') as string}
					oninput={useDebounce((event) => {
						QueryParams.set<ConversationsParams>(
							'search',
							(event.target as HTMLInputElement).value
						)
							.set<ConversationsParams>('page', 0)
							.update();
					}, 300)}
					class="bg-muted h-12 rounded-full border-0 pl-11 text-lg shadow-none md:text-lg"
				/>
			</div>
		</div>
		<div class="mx-auto flex w-full max-w-7xl flex-col px-6 pt-10 md:pt-20">
			<header>
				{#if featured && featured.length > 0}
					<div class="mb-14 md:mb-20">
						<FeaturedConversations
							conversations={featured}
							hrefFor={cardHref}
							actionFor={(c) =>
								c.isComplete ? m.conversations_see_results() : m.take_part()}
						/>
					</div>
				{/if}
				<h2
					id="conversations"
					tabindex="-1"
					class="mb-6 text-2xl font-bold outline-none sm:text-3xl"
				>
					{m.conversations_all_heading()}
				</h2>
				<p>
					{m.conversations_find_open_conversations()}
				</p>
				<div class="mt-4 flex w-full items-center justify-between gap-4">
					<FilterBadges
						param="filter"
						label={m.conversations_filter_label()}
						allLabel={m.conversations_filter_all()}
						options={[
							['open', m.conversations_filter_open()],
							['ended', m.conversations_filter_ended()]
						]}
					/>
					<LayoutToggle />
				</div>
				<div class="mt-10 mb-2">
					{#if data.conversations.err !== null}
						<ErrorMessage>{m.conversations_loading_error()}</ErrorMessage>
					{:else}
						<p class="text-muted-foreground font-semibold" aria-live="polite">
							{#if data.conversations.ok.total === 0}
								{m.conversations_no_conversations_found()}
							{:else if data.conversations.ok.total === 1}
								{m.conversations_found_one()}
							{:else}
								{m.conversations_found_count({
									count: data.conversations.ok.total
								})}
							{/if}
						</p>
					{/if}
				</div>
			</header>

			{#if nothingMatches}
				<div
					class="bg-card border-border mb-10 flex flex-col items-start gap-2 rounded-2xl border p-5"
				>
					<h3 class="text-lg font-bold">{m.conversations_nothing_matches()}</h3>
					<p class="text-muted-foreground text-base">
						{m.conversations_nothing_matches_hint()}
					</p>
					<Button
						variant="outline"
						class="mt-2 rounded-full"
						onclick={clearSearchAndFilters}
					>
						{m.conversations_clear_filters()}
					</Button>
				</div>
			{/if}

			{#if data.conversations.ok !== null && data.conversations.ok.records.length > 0}
				<div
					class={view === 'list' ? 'flex flex-col gap-3' : 'dynamic-grid gap-6'}
					style="--grid-item-width: 19rem;"
				>
					{#each data.conversations.ok.records as conversation (conversation.id)}
						<ConversationCard {conversation} for="public" layout={view}>
							{#snippet Actions()}
								<span
									class={buttonVariants({
										variant: conversation.isComplete ? 'outline' : 'primaryDark'
									})}
									aria-hidden="true"
								>
									{conversation.isComplete
										? m.conversations_see_results()
										: m.take_part()}
									<ArrowRight class="size-4" />
								</span>
							{/snippet}
						</ConversationCard>
					{/each}
				</div>

				<div class="my-10 self-center md:mt-20">
					<Pager
						pageSize={data.CONVERSATION_PAGE_SIZE}
						count={data.conversations.ok.total}
						url={page.url}
					/>
				</div>
			{/if}
		</div>
	{/if}
</div>

<style>
	.home-hero {
		background: linear-gradient(
			to bottom,
			color-mix(in oklab, var(--sidebar-primary) 8%, var(--background)),
			color-mix(in oklab, var(--sidebar-primary) 50%, var(--background))
		);
	}
	/* Two big see-through planes on the right, overlapping like light through a window. */
	.home-hero-shape {
		position: absolute;
		z-index: -1;
		inset: 0;
		pointer-events: none;
	}
	.home-hero-shape-a {
		background: rgb(255 255 255 / 0.3);
		clip-path: polygon(100% 25%, 100% 100%, 30% 100%);
	}
	.home-hero-shape-b {
		background: rgb(255 255 255 / 0.2);
		clip-path: polygon(72% 0%, 100% 0%, 100% 100%, 60% 100%);
	}
</style>
