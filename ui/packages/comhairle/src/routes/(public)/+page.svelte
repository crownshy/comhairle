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

	let { data } = $props();
	const isCommunity = $derived(data.isCommunity);
	const pageUrl = $derived(page.url);
	const view = $derived(listViewFromUrl(page.url));

	// The line under the filters: how many conversations the chosen filter (and any search) found.
	const countLabel = $derived.by(() => {
		const count = data.total;
		const search = getSearch(page.url).trim();
		if (search) {
			if (data.status === 'open') {
				if (count === 0) return m.conversations_match_open_none({ search });
				if (count === 1) return m.conversations_match_open_one({ search });
				return m.conversations_match_open({ count, search });
			}
			if (data.status === 'ended') {
				if (count === 0) return m.conversations_match_ended_none({ search });
				if (count === 1) return m.conversations_match_ended_one({ search });
				return m.conversations_match_ended({ count, search });
			}
			if (count === 0) return m.conversations_match_all_none({ search });
			if (count === 1) return m.conversations_match_all_one({ search });
			return m.conversations_match_all({ count, search });
		}
		if (data.status === 'open') {
			if (count === 0) return m.conversations_count_open_none();
			if (count === 1) return m.conversations_count_open_one();
			return m.conversations_count_open({ count });
		}
		if (data.status === 'ended') {
			if (count === 0) return m.conversations_count_ended_none();
			if (count === 1) return m.conversations_count_ended_one();
			return m.conversations_count_ended({ count });
		}
		if (count === 0) return m.conversations_no_conversations_found();
		if (count === 1) return m.conversations_found_one();
		return m.conversations_found_count({ count });
	});

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
			class="mx-auto flex w-full max-w-[1280px] flex-col items-start gap-6 px-6 pt-16 pb-16 md:pb-28"
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
	{#if data.featured.length > 0}
		<div class="mx-auto w-full max-w-[1280px] px-6 pt-10 md:pt-20">
			<FeaturedConversations
				conversations={data.featured}
				hrefFor={cardHref}
				actionFor={(c) => (c.isComplete ? m.conversations_see_results() : m.take_part())}
			/>
		</div>
	{/if}
	<div class="mx-auto flex h-full w-full max-w-[1280px] flex-col px-6 pt-10 md:pt-20">
		<header id="conversations" class="scroll-mt-6 pb-6 md:pb-8">
			<h2
				id="conversations-heading"
				tabindex="-1"
				class="mb-6 text-2xl font-bold focus:outline-none sm:text-3xl"
			>
				{m.conversations_all_heading()}
			</h2>
			<!-- Filters, then search, with the grid/list toggle on the right; they wrap on narrow screens. -->
			<!-- The whole row is the size container, so the chips know how much room there is
				(below @lg they collapse into a dropdown) and keep their natural width beside search. -->
			<div class="@container">
				<div class="flex flex-wrap items-center gap-4">
					<div class="w-full shrink-0 @lg:w-auto">
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
					<div class="w-full sm:w-auto sm:max-w-md sm:min-w-64 sm:flex-1">
						<Search />
					</div>
					<div class="ml-auto">
						<LayoutToggle
							value={view}
							groupLabel={m.conversations_layout()}
							cardLabel={m.conversations_card_view()}
							listLabel={m.conversations_list_view()}
						/>
					</div>
				</div>
			</div>
			<!-- Visible, and announced to screen readers whenever a chip or search changes it. -->
			<p class="text-muted-foreground mt-4 text-base" aria-live="polite">
				{countLabel}
			</p>
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
