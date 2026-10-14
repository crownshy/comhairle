<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { cn } from '$lib/utils';
	import {
		conversationStatusFilters,
		statusFilterHref,
		statusFilterLabels,
		type ConversationStatusFilter
	} from './conversationFilters';

	let { url, active }: { url: URL; active: ConversationStatusFilter } = $props();
</script>

<nav aria-label="Filter conversations">
	<ul class="flex flex-wrap gap-2">
		{#each conversationStatusFilters as filter (filter)}
			{@const isActive = filter === active}
			<li>
				<Button
					href={statusFilterHref(url, filter)}
					data-sveltekit-replacestate
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
