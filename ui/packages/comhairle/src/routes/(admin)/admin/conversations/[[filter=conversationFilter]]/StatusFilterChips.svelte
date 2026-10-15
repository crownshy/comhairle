<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { Button } from '$lib/components/ui/button';
	import * as Select from '$lib/components/ui/select';
	import { cn } from '$lib/utils';
	import {
		conversationStatusFilterLabels,
		conversationStatusFilters,
		filterToSegment,
		isConversationStatusFilter,
		type ConversationStatusFilter
	} from '$lib/utils/conversationFilters';

	let { active }: { active: ConversationStatusFilter } = $props();

	const listRoute = '/(admin)/admin/conversations/[[filter=conversationFilter]]';

	function routeParams(filter: ConversationStatusFilter) {
		return { filter: filterToSegment(filter) };
	}
</script>

<!-- Chips wrap awkwardly in a narrow column, so below @lg they collapse into a dropdown. -->
<nav aria-label="Filter conversations" class="hidden @lg:block">
	<ul class="flex flex-wrap gap-2">
		{#each conversationStatusFilters as filter (filter)}
			{@const isActive = filter === active}
			<li>
				<Button
					href={resolve(listRoute, routeParams(filter))}
					data-sveltekit-noscroll
					aria-current={isActive ? 'page' : undefined}
					variant={isActive ? 'default' : 'ghost'}
					size="lg"
					class={cn('px-4 text-base', !isActive && 'bg-card text-foreground')}
				>
					{conversationStatusFilterLabels[filter]}
				</Button>
			</li>
		{/each}
	</ul>
</nav>

<div class="@lg:hidden">
	<Select.Root
		type="single"
		value={active}
		onValueChange={(value) => {
			if (isConversationStatusFilter(value))
				goto(resolve(listRoute, routeParams(value)), { noScroll: true });
		}}
	>
		<Select.Trigger aria-label="Filter conversations" class="bg-card h-10 w-full text-base">
			{conversationStatusFilterLabels[active]}
		</Select.Trigger>
		<Select.Content class="w-(--bits-select-anchor-width)">
			{#each conversationStatusFilters as filter (filter)}
				<Select.Item
					value={filter}
					label={conversationStatusFilterLabels[filter]}
					class="text-base"
				/>
			{/each}
		</Select.Content>
	</Select.Root>
</div>
