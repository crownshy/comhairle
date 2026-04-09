<script lang="ts">
	import type { LocalizedEventDto } from '@crownshy/api-client/api';
	import { Badge } from './ui/badge';
	import Button from '$lib/components/ui/button/button.svelte';
	import { formatDateShort, formatTime } from '$lib/utils';
	import { ArrowRight, CalendarDays, Users } from 'lucide-svelte';

	type Props = {
		event: LocalizedEventDto;
		conversationId: string;
	};

	let { event, conversationId }: Props = $props();

	function isUpcoming(event: LocalizedEventDto) {
		return new Date(event.startTime).getTime() > Date.now();
	}
</script>

<article class="flex flex-col">
	<!-- Card body -->
	<div class="flex flex-col gap-4 px-6 pb-6">
		<!-- Title + badge -->
		<div class="flex items-center gap-2">
			<h2 class="text-xl font-semibold">{event.name}</h2>
			{#if isUpcoming(event)}
				<Badge variant="outline" class="bg-primary/10">Upcoming</Badge>
			{:else}
				<Badge variant="secondary">Past</Badge>
			{/if}
		</div>

		<!-- Description -->
		{#if event.description}
			<p class="text-muted-foreground text-base font-medium">
				{event.description}
			</p>
		{/if}

		<!-- Info rows -->
		<div class="flex flex-col gap-2">
			<div class="flex items-center gap-2 text-sm">
				<CalendarDays class="text-foreground h-4 w-4 shrink-0" />
				<span class="font-medium">{formatDateShort(event.startTime)}</span>
				<span class="text-muted-foreground line-clamp-1"
					>{formatTime(event.startTime)} - {formatTime(event.endTime)}</span
				>
			</div>
			<div class="flex items-center gap-2 text-sm">
				<Users class="text-foreground h-4 w-4 shrink-0" />
				<span class="font-medium">Current attendees</span>
				<span class="text-muted-foreground line-clamp-1"
					>{event.currentAttendance}{event.capacity ? ` / ${event.capacity}` : ''}</span
				>
			</div>
		</div>
	</div>

	<!-- Card footer -->
	<div class="border-border flex items-center justify-center border-t px-6 py-4">
		<Button
			variant="default"
			size="sm"
			href="/admin/conversations/{conversationId}/events/{event.id}"
		>
			Edit event
			<ArrowRight class="ml-1 h-4 w-4" />
		</Button>
	</div>
</article>
