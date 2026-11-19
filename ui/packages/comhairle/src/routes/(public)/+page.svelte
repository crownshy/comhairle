<script lang="ts">
	import { m } from '$lib/paraglide/messages';
	import ConversationCard from '$lib/components/ConversationCard.svelte';
	import { Pager } from '$lib/pagination';
	import { page } from '$app/state';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import Info from '@lucide/svelte/icons/info';
	import ErrorMessage from '$lib/components/ErrorMessage.svelte';
	import { resolve } from '$app/paths';
	import SearchBar from '$lib/components/SearchBar.svelte';
	import QueryParams from '$lib/interfaces/QueryParams.svelte';
	import { useDebounce } from 'runed';

	let { data } = $props();
</script>

<svelte:head>
	<title>Comhairle</title>
</svelte:head>

<!-- No minimum height: the conversation list follows straight after the welcome text,
	so it shows in the first screen. -->
<div class="flex flex-col">
	<section class="bg-primary text-primary-foreground rounded-b-4xl md:rounded-b-[3rem]">
		<div class="mx-auto flex w-full max-w-7xl flex-col gap-y-10 px-6 pt-16 pb-20 md:pb-28">
			<h1 class="scroll-m-20 text-4xl font-bold tracking-tight sm:text-5xl">
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
							Comhairle<Info
								class="text-primary-foreground inline size-5 self-center"
							/>
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
					<p class="w-full text-xl sm:text-2xl">
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
	<div class="relative mx-auto -mt-8 w-full max-w-7xl px-6">
		<div class="bg-card mx-auto max-w-4xl rounded-full p-3 shadow-lg">
			<SearchBar
				placeholder={m.conversations_search()}
				aria-label={m.conversations_search()}
				value={QueryParams.get('search') ?? undefined}
				oninput={useDebounce((event) => {
					QueryParams.setMultiple([
						['search', (event.target as HTMLInputElement).value],
						['page', '0']
					]);
				}, 300)}
				class="bg-muted h-12 rounded-full border-0 pl-11 text-lg shadow-none md:text-lg"
			/>
		</div>
	</div>
	<div class="mx-auto flex h-full w-full max-w-7xl flex-col px-6 pt-10 md:pt-20">
		<header class="pb-5 md:pb-20">
			<p class="sr-only" aria-live="polite">
				{#if data.conversations.ok?.total === 0}
					{m.conversations_no_conversations_found()}
				{:else if data.conversations.ok?.total === 1}
					{m.conversations_found_one()}
				{:else}
					{m.conversations_found_count({ count: data.conversations.ok?.total ?? 0 })}
				{/if}
			</p>
			<h2 class="mb-4 text-4xl font-bold">{m.conversations()}</h2>
			<p class="mb-4">
				{m.conversations_find_open_conversations()}
			</p>
			{#if data.conversations.err !== null}
				<ErrorMessage>{m.conversations_loading_error()}</ErrorMessage>
			{:else if data.conversations.ok.records.length === 0}
				<p class="text-muted-foreground pt-10">
					{m.conversations_no_conversations_found()}
				</p>
			{/if}
		</header>

		{#if data.conversations.ok !== null && data.conversations.ok.records.length > 0}
			<div class="flex grow flex-col items-stretch gap-20 md:gap-10 md:px-2">
				{#each data.conversations.ok.records as conversation (conversation.id)}
					<a
						class="block w-full"
						href={resolve(`/(public)/conversations/[conversation_id]`, {
							conversation_id: conversation.slug || conversation.id
						})}
					>
						<ConversationCard {conversation} variant="public" />
					</a>
				{/each}
			</div>

			<div class="my-10 flex w-full justify-center md:mt-20">
				<Pager
					pageSize={data.CONVERSATION_PAGE_SIZE}
					count={data.conversations.ok.total}
					url={page.url}
				/>
			</div>
		{/if}
	</div>
</div>
