<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { Button } from '$lib/components/ui/button';
	import * as Select from '$lib/components/ui/select';
	import { cn } from '$lib/utils';
	import {
		CONVERSATION_STATUS_FILTER_LABELS,
		CONVERSATION_STATUS_FILTERS,
		isConversationStatusFilter,
		statusFilterHref,
		type ConversationStatusFilter
	} from './conversationFilters';

	type Props = { active: ConversationStatusFilter };

	let { active }: Props = $props();

	function selectFilter(value: string) {
		if (!isConversationStatusFilter(value)) return;
		// statusFilterHref only swaps the query on the current page's URL.
		// eslint-disable-next-line svelte/no-navigation-without-resolve
		goto(statusFilterHref(page.url, value), { noScroll: true });
	}
</script>

<!-- Chips wrap awkwardly in a narrow column, so below @lg they collapse into a dropdown. -->
<nav aria-label="Filter conversations" class="hidden @lg:block">
	<ul class="flex flex-wrap gap-2">
		{#each CONVERSATION_STATUS_FILTERS as filter (filter)}
			{@const isActive = filter === active}
			<li>
				<Button
					href={statusFilterHref(page.url, filter)}
					data-sveltekit-noscroll
					aria-current={isActive ? 'page' : undefined}
					variant={isActive ? 'default' : 'ghost'}
					size="lg"
					class={cn('px-4 text-base', !isActive && 'bg-card text-foreground')}
				>
					{CONVERSATION_STATUS_FILTER_LABELS[filter]}
				</Button>
			</li>
		{/each}
	</ul>
</nav>

<div class="@lg:hidden">
	<Select.Root type="single" value={active} onValueChange={selectFilter}>
		<Select.Trigger aria-label="Filter conversations" class="bg-card h-10 w-full text-base">
			{CONVERSATION_STATUS_FILTER_LABELS[active]}
		</Select.Trigger>
		<Select.Content class="w-(--bits-select-anchor-width)">
			{#each CONVERSATION_STATUS_FILTERS as filter (filter)}
				<Select.Item
					value={filter}
					label={CONVERSATION_STATUS_FILTER_LABELS[filter]}
					class="text-base"
				/>
			{/each}
		</Select.Content>
	</Select.Root>
</div>
