<script lang="ts">
	import { resolve } from '$app/paths';
	import { m } from '$lib/paraglide/messages';
	import ConversationCard from '$lib/components/ConversationCard.svelte';
	import LayoutToggle from '$lib/components/LayoutToggle.svelte';
	import { listViewFromUrl } from '$lib/utils/listView';

	import Search from './conversations/Search.svelte';
	import { Pager } from '$lib/pagination';
	import { page } from '$app/state';
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import { PAGE_SIZE, getSearch, statusFilterHref } from './conversations/utils';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import { ArrowDown, ArrowRight, Info } from 'lucide-svelte';
	import FilterChips from '$lib/components/FilterChips.svelte';
	import FeaturedConversations from '$lib/components/FeaturedConversations.svelte';
	import { getPage } from '$lib/pagination';

	let { data } = $props();
	const isCommunity = $derived(data.isCommunity);
	const pageUrl = $derived(page.url);
	const view = $derived(listViewFromUrl(page.url));

	// TODO: the API has no "featured" flag yet, so for now the first three open conversations
	// stand in. Swap this for the real featured list once the backend supports it.
	// Only on the first page and when not searching, so it doesn't sit above search results.
	const featured = $derived(
		getPage(page.url) === 1 && !getSearch(page.url)
			? data.records.filter((c) => !c.isComplete).slice(0, 3)
			: []
	);

	function cardHref(conversation: (typeof data.records)[number]) {
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
		document.getElementById('conversations-heading')?.focus({ preventScroll: true });
	}
</script>

<svelte:head>
	<title>Comhairle</title>
</svelte:head>

<!-- No minimum height: the conversation list follows straight after the welcome text,
	so it shows in the first screen. -->
<div class="flex flex-col">
	<section class="bg-primary text-primary-foreground rounded-b-4xl md:rounded-b-[3rem]">
		<div
			class="mx-auto flex w-full max-w-[1280px] flex-col items-start gap-6 px-6 pt-16 pb-20 md:pb-28"
		>
			<div class="flex max-w-3xl flex-col gap-3">
				<h1
					class="scroll-m-20 text-[1.75rem] leading-tight font-bold text-balance sm:text-4xl lg:text-[2.625rem]"
				>
					{#if isCommunity}
						{m.welcome_to_the()}
					{:else}
						{m.welcome_to()}
					{/if}

					<Tooltip.Provider delayDuration={0}>
						<Tooltip.Root>
							<Tooltip.Trigger
								class="inline-flex cursor-default items-baseline gap-2"
								onclick={(e) => e.preventDefault()}
							>
								Comhairle<Info
									class="text-primary-foreground inline size-5 self-center"
								/>
							</Tooltip.Trigger>
							<Tooltip.Content side="top" class="text-base">
								{m.comhairle_meaning()}
							</Tooltip.Content>
						</Tooltip.Root>
					</Tooltip.Provider>

					{#if isCommunity}
						{m.community_server()}
					{/if}
				</h1>

				{#if isCommunity}
					{#each [m.community_server_description_1(), m.community_server_description_2()] as description (description)}
						<p class="text-lg text-pretty opacity-90 sm:text-xl">
							{description}
						</p>
					{/each}
				{:else}
					<p class="text-lg text-pretty opacity-90 sm:text-xl">
						{m.landing_page_subtitle()}
					</p>
				{/if}
			</div>

			{#if data.records.length > 0}
				<Button
					href="#conversations"
					onclick={scrollToList}
					variant="outline"
					class="text-primary h-12 w-full gap-2 rounded-full border-transparent bg-white px-6 text-base font-semibold hover:bg-white/90 sm:w-auto"
				>
					{m.explore_conversations()}
					<ArrowDown class="size-5" />
				</Button>
			{/if}
		</div>
	</section>
	<div class="relative mx-auto -mt-10 w-full max-w-[1280px] px-6">
		<div
			class="bg-card mx-auto max-w-4xl rounded-full p-3 shadow-[0_8px_24px_rgba(0,0,0,0.12)] md:p-4"
		>
			<Search />
		</div>
	</div>
	{#if featured.length > 0}
		<div class="mx-auto w-full max-w-[1280px] px-6 pt-10 md:pt-20">
			<FeaturedConversations
				conversations={featured}
				hrefFor={cardHref}
				actionFor={(c) => (c.isComplete ? m.conversations_see_results() : m.take_part())}
			/>
		</div>
	{/if}
	<div class="mx-auto flex h-full w-full max-w-[1280px] flex-col px-6 pt-10 md:pt-20">
		<header id="conversations" class="scroll-mt-6 pb-5 md:pb-20">
			<p class="sr-only" aria-live="polite">
				{#if data.total === 0}
					{m.conversations_no_conversations_found()}
				{:else if data.total === 1}
					{m.conversations_found_one()}
				{:else}
					{m.conversations_found_count({ count: data.total })}
				{/if}
			</p>
			<h2
				id="conversations-heading"
				tabindex="-1"
				class="mb-2 text-2xl font-bold focus:outline-none sm:text-3xl"
			>
				{m.conversations_all_heading()}
			</h2>
			<p class="mb-6">
				{m.conversations_find_open_conversations()}
			</p>
			<!-- Filters on the left, the grid/list toggle on the right; they wrap on narrow screens. -->
			<div class="flex flex-wrap items-center justify-between gap-4">
				<div class="@container min-w-0 flex-1">
					<FilterChips
						options={[
							{ value: 'open', label: m.conversations_filter_open() },
							{ value: 'ended', label: m.conversations_filter_ended() },
							{ value: 'all', label: m.conversations_filter_all() }
						]}
						active={data.status}
						hrefFor={(value) => statusFilterHref(page.url, value)}
						label={m.conversations_filter_label()}
					/>
				</div>
				<LayoutToggle
					value={view}
					groupLabel={m.conversations_layout()}
					cardLabel={m.conversations_card_view()}
					listLabel={m.conversations_list_view()}
				/>
			</div>
			{#if data.records.length === 0}
				<p class="text-muted-foreground pt-10">
					{m.conversations_no_conversations_found()}
				</p>
			{/if}
		</header>

		{#if data.records.length > 0}
			<div
				class={view === 'card'
					? 'grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3'
					: 'flex flex-col gap-3'}
			>
				{#each data.records as conversation (conversation.id)}
					<ConversationCard
						statusLabel={conversation.isComplete
							? m.conversation_status_ended()
							: m.conversation_status_open()}
						layout={view}
						{conversation}
						href={cardHref(conversation)}
						headingLevel={3}
					>
						{#snippet actions()}
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
						{/snippet}</ConversationCard
					>
				{/each}
			</div>

			<div class="my-10 flex w-full justify-center md:mt-20">
				<Pager pageSize={PAGE_SIZE} count={data.total} url={pageUrl} />
			</div>
		{/if}
	</div>
</div>
