<script lang="ts">
	import type { LocalizedEventDto } from '@crownshy/api-client/api';
	import { Badge } from './ui/badge';
	import { ArrowRight } from 'lucide-svelte';
	import { cn } from '$lib/utils';
	import { buttonVariants } from '$lib/components/ui/button';

	type Props = {
		event: LocalizedEventDto;
		conversationId: string;
	};

	let { event, conversationId }: Props = $props();

	function isUpcoming(event: LocalizedEventDto) {
		return new Date(event.startTime).getTime() > Date.now();
	}
</script>

<article>
	<a href={`/admin/conversations/${conversationId}/events/${event.id}`}>
		<div
			class="flex w-full max-w-7xl min-w-95 flex-col items-start justify-start gap-6 md:min-w-120 lg:flex-row lg:justify-between lg:gap-16"
		>
			<div class="flex flex-col items-start justify-start gap-3 self-stretch lg:gap-4">
				{#if isUpcoming(event)}
					<Badge variant="default" class="h-7 text-sm">Upcoming</Badge>
				{:else}
					<Badge variant="default" class="h-7 text-sm">Past</Badge>
				{/if}

				<h2
					class="text-foreground self-stretch text-xl leading-7 font-semibold lg:text-2xl"
				>
					{event.name}
				</h2>

				<p class="text-foreground self-stretch text-sm leading-5 font-medium">
					{event.description}
				</p>

				<span
					class={cn(
						buttonVariants({ variant: 'default' }),
						'bg-sidebar hover:bg-sidebar/90 text-sidebar-foreground rounded-full px-4 py-3'
					)}
				>
					Edit event
					<ArrowRight class="ml-1 size-4" />
				</span>
			</div>
			<div>
				<p class="text-foreground text-lg font-semibold">
					Current attendance: <Badge class="text-base">{event.currentAttendance}</Badge>
				</p>
			</div>
		</div>
	</a>
</article>
