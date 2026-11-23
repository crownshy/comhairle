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
	import { ArrowRight, Info } from '@lucide/svelte';
	import { buttonVariants } from '$lib/components/ui/button';
	import FilterBadges from '$lib/components/FilterBadges.svelte';
	import LayoutToggle from '$lib/components/LayoutToggle.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Comhairle</title>
</svelte:head>

<!-- No minimum height: the conversation list follows straight after the welcome text,
	so it shows in the first screen. -->
<div class="flex flex-col">
	<section class="bg-primary text-primary-foreground rounded-b-4xl md:rounded-b-[3rem]">
		<div class="mx-auto max-w-7xl px-6 pt-16 pb-20 md:pb-28">
			<h1 class="scroll-m-20 pb-10 text-4xl font-bold tracking-tight sm:text-5xl">
				{#if data.isCommunity}
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
							Comhairle
							<Info class="text-primary-foreground inline size-5 self-center" />
						</Tooltip.Trigger>
						<Tooltip.Content side="top" class="text-base">
							{m.comhairle_meaning()}
						</Tooltip.Content>
					</Tooltip.Root>
				</Tooltip.Provider>

				{#if data.isCommunity}
					{m.community_server()}
				{/if}
			</h1>

			{#if data.isCommunity}
				{#each [m.community_server_description_1(), m.community_server_description_2()] as description (description)}
					<p class="text-xl sm:text-2xl">
						{description}
					</p>
				{/each}
			{:else}
				<p class="max-w-xl text-xl sm:text-2xl">
					{m.landing_page_subtitle()}
				</p>
			{/if}
		</div>
	</section>
	<div class="mx-auto w-full max-w-7xl -translate-y-1/2 px-6">
		<div class="bg-card mx-auto max-w-4xl rounded-full p-3 shadow-lg">
			<SearchBar
				placeholder={m.conversations_search()}
				aria-label={m.conversations_search()}
				value={QueryParams.get<ConversationsParams>('search')}
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
			<h2 class="mb-4 text-4xl font-bold">{m.conversations()}</h2>
			<p>
				{m.conversations_find_open_conversations()}
			</p>
			<div class="mt-1 flex w-full items-center justify-between">
				<FilterBadges
					param="filter"
					label={m.conversations_filter_label()}
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
							{m.conversations_found_count({ count: data.conversations.ok.total })}
						{/if}
					</p>
				{/if}
			</div>
		</header>

		{#if data.conversations.ok !== null && data.conversations.ok.records.length > 0}
			<div
				class={QueryParams.get<ConversationsParams>('view') === 'card'
					? 'grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3'
					: 'flex flex-col gap-3'}
			>
				{#each data.conversations.ok.records as conversation (conversation.id)}
					<ConversationCard {conversation} for="public">
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
</div>
