<script lang="ts">
	import { LayoutGrid, List, type IconProps } from '@lucide/svelte';
	import * as ToggleGroup from '$lib/components/ui/toggle-group';
	import type { ConversationCardLayout } from '$lib/components/ConversationCard.svelte';
	import type { Component } from 'svelte';
	import { m } from '$lib/paraglide/messages';
	import QueryParams from '$lib/interfaces/QueryParams.svelte';

	type Params = {
		view: ConversationCardLayout;
	};

	const options: [value: ConversationCardLayout, label: string, icon: Component<IconProps>][] = [
		['card', m.filter_card_view(), LayoutGrid],
		['list', m.filter_list_view(), List]
	];
</script>

<ToggleGroup.Root
	type="single"
	value={QueryParams.get<Params>('view')}
	onValueChange={(value) =>
		void QueryParams.set<Params>('view', value as ConversationCardLayout).update()}
	aria-label={m.filter_layouts()}
	class="bg-muted h-10 shrink-0 gap-1 rounded-full p-2"
>
	{#each options as [value, label, Icon] (value)}
		<ToggleGroup.Item
			{value}
			title={label}
			aria-label={label}
			class="text-muted-foreground hover:text-foreground data-[state=on]:bg-card data-[state=on]:text-foreground size-8 rounded-full! px-2 py-1 hover:bg-transparent data-[state=on]:shadow-sm"
		>
			<Icon class="size-4" />
		</ToggleGroup.Item>
	{/each}
</ToggleGroup.Root>
