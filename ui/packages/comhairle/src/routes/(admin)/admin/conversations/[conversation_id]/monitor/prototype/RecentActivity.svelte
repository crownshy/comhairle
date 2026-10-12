<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { cn } from '$lib/utils';
	import type { ActivityItem } from './sampleMonitorData';

	type Props = {
		activity: ActivityItem[];
	};

	let { activity }: Props = $props();

	const DOT_CLASS: Record<ActivityItem['kind'], string> = {
		progress: 'bg-primary',
		neutral: 'bg-muted-foreground/60',
		drop: 'bg-amber-600'
	};
</script>

<div class="flex flex-col gap-6">
	<Card.Root>
		<Card.Header>
			<Card.Title>Recent activity</Card.Title>
		</Card.Header>
		<Card.Content>
			<ul>
				{#each activity as item, index (index)}
					<li
						class="grid grid-cols-[0.75rem_minmax(0,1fr)_auto] items-start gap-3 border-t py-3 first:border-t-0"
					>
						<span
							class={cn('mt-1.5 size-2.5 rounded-full', DOT_CLASS[item.kind])}
							aria-hidden="true"
						></span>
						<span class="text-base">{item.text}</span>
						<span class="text-muted-foreground text-sm whitespace-nowrap"
							>{item.when}</span
						>
					</li>
				{/each}
			</ul>
		</Card.Content>
	</Card.Root>
</div>
