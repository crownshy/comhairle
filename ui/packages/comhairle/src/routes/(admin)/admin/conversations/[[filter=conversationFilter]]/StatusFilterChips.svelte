<script lang="ts">
	import { resolve } from '$app/paths';
	import { Button } from '$lib/components/ui/button';
	import { cn } from '$lib/utils';
	import {
		conversationStatusFilters,
		statusFilterLabels,
		type ConversationStatusFilter
	} from '$lib/utils/conversationFilters';

	let { active }: { active: ConversationStatusFilter } = $props();

	function filterHref(filter: ConversationStatusFilter) {
		return resolve('/(admin)/admin/conversations/[[filter=conversationFilter]]', {
			filter: filter === 'all' ? undefined : filter
		});
	}
</script>

<nav aria-label="Filter conversations">
	<ul class="flex flex-wrap gap-2">
		{#each conversationStatusFilters as filter (filter)}
			{@const isActive = filter === active}
			<li>
				<Button
					href={filterHref(filter)}
					data-sveltekit-noscroll
					aria-current={isActive ? 'page' : undefined}
					variant={isActive ? 'default' : 'ghost'}
					size="lg"
					class={cn('px-4 text-base', !isActive && 'bg-card text-foreground')}
				>
					{statusFilterLabels[filter]}
				</Button>
			</li>
		{/each}
	</ul>
</nav>
