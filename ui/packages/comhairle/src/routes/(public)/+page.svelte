<script lang="ts">
	import { resolve } from '$app/paths';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { m } from '$lib/paraglide/messages';
	import ConversationCard from '$lib/components/ConversationCard.svelte';
	import LayoutToggle from '$lib/components/LayoutToggle.svelte';
	import { listViewFromUrl } from '$lib/utils/listView';
	import Check from 'svelte-radix/Check.svelte';
	import ChevronDown from 'svelte-radix/ChevronDown.svelte';

	import Search from './conversations/Search.svelte';
	import { Pager } from '$lib/pagination';
	import { page } from '$app/state';
	import { buttonVariants } from '$lib/components/ui/button';
	import { getSort, setSort, PAGE_SIZE, parseSort, type SortBy } from './conversations/utils';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import { ArrowRight, Info } from 'lucide-svelte';

	let { data } = $props();
	const isCommunity = $derived(data.isCommunity);
	const pageUrl = $derived(page.url);
	const view = $derived(listViewFromUrl(page.url));
</script>

<svelte:head>
	<title>Comhairle</title>
</svelte:head>

{#snippet sortOption(url: URL, sort: SortBy)}
	<a
		href={setSort(url, sort).toString()}
		aria-current={getSort(url) === sort ? 'page' : undefined}
	>
		<DropdownMenu.Item>
			<span class="w-6">
				{#if getSort(url) === sort}
					<Check class="h-4 w-4" />
				{/if}
			</span>
			{parseSort(sort)}
		</DropdownMenu.Item>
	</a>
{/snippet}

<!-- No minimum height: the conversation list follows straight after the welcome text,
	so it shows in the first screen. -->
<div class="flex flex-col">
	<section class="bg-primary text-primary-foreground rounded-b-4xl md:rounded-b-[3rem]">
		<div class="mx-auto flex w-full max-w-[1280px] flex-col gap-y-10 px-6 pt-16 pb-20 md:pb-28">
			<h1 class="scroll-m-20 text-4xl font-bold tracking-tight sm:text-5xl">
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
	<div class="relative mx-auto -mt-10 w-full max-w-[1280px] px-6">
		<div
			class="bg-card mx-auto max-w-4xl rounded-full p-3 shadow-[0_8px_24px_rgba(0,0,0,0.12)] md:p-4"
		>
			<Search />
		</div>
	</div>
	<div class="mx-auto flex h-full w-full max-w-[1280px] flex-col px-6 pt-10 md:pt-20">
		<header class="pb-5 md:pb-20">
			<p class="sr-only" aria-live="polite">
				{#if data.total === 0}
					{m.conversations_no_conversations_found()}
				{:else if data.total === 1}
					{m.conversations_found_one()}
				{:else}
					{m.conversations_found_count({ count: data.total })}
				{/if}
			</p>
			<h2 class="mb-4 text-4xl font-bold">{m.conversations()}</h2>
			<p class="mb-4">
				{m.conversations_find_open_conversations()}
			</p>
			<div class="flex items-center justify-between">
				<DropdownMenu.Root>
					<DropdownMenu.Trigger
						class={buttonVariants({ variant: 'outline-solid', size: 'sm' })}
					>
						<ChevronDown class="h-4 w-4" />{m.sort()}
					</DropdownMenu.Trigger>
					<DropdownMenu.Content>
						<DropdownMenu.Group>
							{@render sortOption(pageUrl, 'title+asc')}
							{@render sortOption(pageUrl, 'title+desc')}
							{@render sortOption(pageUrl, 'created_at+desc')}
							{@render sortOption(pageUrl, 'created_at+asc')}
						</DropdownMenu.Group>
					</DropdownMenu.Content>
				</DropdownMenu.Root><LayoutToggle
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
						layout={view}
						{conversation}
						href={resolve('/(public)/conversations/[conversation_id]', {
							conversation_id: conversation.slug || conversation.id
						})}
						headingLevel={3}
					>
						{#snippet actions()}
							<span
								class={buttonVariants({ variant: 'primaryDark' })}
								aria-hidden="true"
							>
								{m.take_part()}
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
