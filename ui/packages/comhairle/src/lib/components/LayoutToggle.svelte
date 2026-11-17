<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { LayoutGrid, List } from 'lucide-svelte';
	import * as ToggleGroup from '$lib/components/ui/toggle-group';
	import type { ConversationCardLayout } from '$lib/components/ConversationCard.svelte';
	import { listViewHref } from '$lib/utils/listView';

	type Props = {
		value: ConversationCardLayout;
		/** Accessible names, so public pages can pass translated text. */
		groupLabel?: string;
		cardLabel?: string;
		listLabel?: string;
	};

	let {
		value,
		groupLabel = 'Layout',
		cardLabel = 'Card view',
		listLabel = 'List view'
	}: Props = $props();

	// In the URL so the choice survives a reload and the pager's links.
	function showView(next: string) {
		if (next !== 'card' && next !== 'list') return;
		// eslint-disable-next-line svelte/no-navigation-without-resolve -- only swaps the query on this page's URL
		goto(listViewHref(page.url, next), { replaceState: true, noScroll: true, keepFocus: true });
	}

	const VIEW_ITEM_CLASS =
		'text-muted-foreground hover:text-foreground data-[state=on]:bg-card data-[state=on]:text-foreground rounded-full p-2 first:rounded-full last:rounded-full hover:bg-transparent data-[state=on]:shadow-sm';
</script>

<ToggleGroup.Root
	type="single"
	{value}
	onValueChange={showView}
	aria-label={groupLabel}
	class="bg-muted shrink-0 gap-1 rounded-full p-1"
>
	<ToggleGroup.Item value="card" aria-label={cardLabel} class={VIEW_ITEM_CLASS}>
		<LayoutGrid class="size-4" />
	</ToggleGroup.Item>
	<ToggleGroup.Item value="list" aria-label={listLabel} class={VIEW_ITEM_CLASS}>
		<List class="size-4" />
	</ToggleGroup.Item>
</ToggleGroup.Root>
