<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import { Badge } from '$lib/components/ui/badge';
	import { formatDateShort, formatTime } from '$lib/utils';
	import { CalendarDays, Users } from 'lucide-svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	let conversationId = $derived(data.conversationId);
	let events = $derived(data.events);

	function eventStatus(start: string, end: string): 'upcoming' | 'live' | 'past' {
		const now = Date.now();
		const s = new Date(start).getTime();
		const e = new Date(end).getTime();
		if (now < s) return 'upcoming';
		if (now > e) return 'past';
		return 'live';
	}
</script>

<svelte:head>
	<title>Events</title>
</svelte:head>

<div class="mx-auto flex max-w-[700px] flex-col items-center gap-6 py-8">
	<div class="w-full">
		<div class="flex items-center gap-3">
			<h1 class="text-3xl font-semibold">Events</h1>
		</div>
		<p class="text-muted-foreground mt-2 text-base font-medium">
			{events.length} event{events.length !== 1 ? 's' : ''} scheduled for this conversation.
		</p>
	</div>

	{#if events.length === 0}
		<div class="border-border w-full rounded-3xl border border-dashed p-12 text-center">
			<p class="text-muted-foreground">No events scheduled for this conversation yet.</p>
		</div>
	{:else}
		<div class="flex w-full flex-col gap-6">
			{#each events as event (event.id)}
				{@const status = eventStatus(event.startTime, event.endTime)}
				<div
					class="bg-card border-border relative flex flex-col gap-4 overflow-hidden rounded-3xl border p-6 shadow-sm"
				>
					<!-- Action button (top-right) -->
					<div class="absolute top-6 right-6">
						{#if status === 'live'}
							<Button
								variant="outline"
								size="sm"
								href="/conversations/{conversationId}/events/{event.id}/live"
							>
								Join Live
							</Button>
						{:else}
							<Button
								variant="outline"
								size="sm"
								href="/conversations/{conversationId}/events/{event.id}"
							>
								View event
							</Button>
						{/if}
					</div>

					<!-- Title + badge -->
					<div class="flex items-center gap-2 pr-28">
						<h2 class="text-2xl leading-7 font-semibold">{event.name}</h2>
						{#if status === 'live'}
							<Badge
								variant="outline"
								class="shrink-0 border-green-200 bg-green-50 text-green-700"
							>
								<span
									class="mr-1 h-1.5 w-1.5 animate-pulse rounded-full bg-green-500"
								></span>
								Live
							</Badge>
						{:else if status === 'past'}
							<Badge variant="secondary" class="shrink-0">Past</Badge>
						{:else}
							<Badge variant="outline" class="bg-primary/10 shrink-0">Upcoming</Badge>
						{/if}
					</div>

					<!-- Description -->
					{#if event.description}
						<p class="text-muted-foreground text-base leading-6 font-medium">
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
								>{event.currentAttendance}{event.capacity
									? ` / ${event.capacity}`
									: ''}</span
							>
						</div>
					</div>
				</div>
			{/each}
		</div>
	{/if}

	<a
		href="/conversations/{conversationId}"
		class="text-muted-foreground hover:text-foreground text-sm"
	>
		← Back to conversation
	</a>
</div>
