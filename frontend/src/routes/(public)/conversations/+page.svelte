<script lang="ts">
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as m from '$lib/paraglide/messages';
	import ConversationCard from '$lib/components/ConversationCard.svelte';
	import Check from 'svelte-radix/Check.svelte';
	import ChevronDown from 'svelte-radix/ChevronDown.svelte';

	import Search from './Search.svelte';
	import { Pager } from '$lib/pagination';
	import { getSort, setSort, PAGE_SIZE, parseSort, type SortBy } from './utils';
	import type { Conversation } from '$lib/api/api';
	import { page } from '$app/state';
	import { buttonVariants } from '$lib/components/ui/button';
	let { data }: { data: { records: Array<Conversation>; total: number } } = $props();

	const pageUrl = $derived(page.url);
</script>

{#snippet sortOption(url: URL, sort: SortBy)}
	<a href={setSort(url, sort).toString()} aria-current={getSort(url) === sort ? 'page' : undefined}>
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

<div class="flex h-full flex-col pt-10">
	<header class="mb:pb-20 px-2 pb-5 md:px-0">
		<h1 class="mb-4 text-4xl font-bold">{m.conversations()}</h1>
		<p class="mb-4">
			{m.find_open_conversations()}
		</p>
		<div class="flex justify-between">
			<DropdownMenu.Root>
				<DropdownMenu.Trigger class={buttonVariants({ variant: 'outline-solid', size: 'sm' })}>
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
			<Search url={pageUrl} />
		</div>
	</header>

	<div
		class="grid w-full grow auto-rows-auto grid-cols-1 items-center gap-4 overflow-y-auto md:px-0 md:px-2"
	>
		{#each data.records as conversation}
			<ConversationCard {conversation} />
		{/each}
	</div>

	<div class="flex w-full justify-center md:mt-20">
		<Pager pageSize={PAGE_SIZE} count={data.total} url={pageUrl} />
	</div>
</div>
