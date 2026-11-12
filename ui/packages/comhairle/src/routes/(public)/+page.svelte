<script lang="ts">
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { m } from '$lib/paraglide/messages';
	import ConversationCard from '$lib/components/ConversationCard.svelte';
	import Check from 'svelte-radix/Check.svelte';
	import ChevronDown from 'svelte-radix/ChevronDown.svelte';

	import Search from './conversations/Search.svelte';
	import { Pager } from '$lib/pagination';
	import { page } from '$app/state';
	import { buttonVariants } from '$lib/components/ui/button';
	import { getSort, setSort, PAGE_SIZE, parseSort, type SortBy } from './conversations/utils';
	import Button from '$lib/components/ui/button/button.svelte';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import { Info } from 'lucide-svelte';

	let { data } = $props();
	const isCommunity = $derived(data.isCommunity);
	const pageUrl = $derived(page.url);
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
	<div class="flex flex-col gap-y-10 pt-16">
		<h1 class="text-foreground scroll-m-20 text-4xl font-bold tracking-tight sm:text-5xl">
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
						Comhairle<Info class="text-primary inline size-5 self-center" />
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
				<p class="text-foreground w-full text-xl sm:text-2xl">
					{description}
				</p>
			{/each}
		{:else}
			<p class="text-foreground max-w-xl text-xl sm:text-2xl">
				{m.landing_page_subtitle()}
			</p>
		{/if}

		<Button
			size="lg"
			class="bg-sidebar text-sidebar-foreground px-8 py-6 text-lg"
			href="/conversations"
		>
			{m.explore_conversations()}
		</Button>
	</div>
	<div class="flex h-full flex-col pt-10 md:pt-20">
		<header class="pb-5 md:pb-20">
			<h2 class="mb-4 text-4xl font-bold">{m.conversations()}</h2>
			<p class="mb-4">
				{m.conversations_find_open_conversations()}
			</p>
			<div class="flex justify-between">
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
				</DropdownMenu.Root>
				<Search />
			</div>
			{#if data.records.length === 0}
				<p class="text-muted-foreground pt-10">
					{m.conversations_no_conversations_found()}
				</p>
			{/if}
		</header>

		{#if data.records.length > 0}
			<div class="flex grow flex-col items-stretch gap-20 md:gap-10 md:px-2">
				{#each data.records as conversation (conversation.id)}
					<a
						class="block w-full"
						href={`/conversations/${conversation.slug || conversation.id}`}
					>
						<ConversationCard {conversation} variant="public" />
					</a>
				{/each}
			</div>

			<div class="my-10 flex w-full justify-center md:mt-20">
				<Pager pageSize={PAGE_SIZE} count={data.total} url={pageUrl} />
			</div>
		{/if}
	</div>
</div>
