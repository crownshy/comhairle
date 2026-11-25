<script lang="ts">
	import { resolve } from '$app/paths';
	import { buttonVariants } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { cn, formatDateShort, formatTime } from '$lib/utils';
	import CalendarDays from '@lucide/svelte/icons/calendar-days';
	import Users from '@lucide/svelte/icons/users';
	import type { PageProps } from './$types';
	import { m } from '$lib/paraglide/messages';

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
	<title>{m.events_title()}</title>
</svelte:head>

<div class="mx-auto flex max-w-[700px] flex-col items-center gap-6 py-8">
	<div class="w-full">
		<div class="flex items-center gap-3">
			<h1 class="text-3xl font-semibold">{m.events_title()}</h1>
		</div>
		<p class="text-muted-foreground mt-2 text-base font-medium">
			{events.length === 1
				? m.events_count_one({ count: events.length })
				: m.events_count_other({ count: events.length })}
		</p>
	</div>

	{#if events.length === 0}
		<div class="border-border w-full rounded-3xl border border-dashed p-12 text-center">
			<p class="text-muted-foreground">{m.events_none()}</p>
		</div>
	{:else}
		<div class="flex w-full flex-col gap-6">
			{#each events as event (event.id)}
				{@const status = eventStatus(event.startTime, event.endTime)}
				<article
					class="bg-card border-border hover:border-primary/40 has-[a:focus-visible]:ring-ring/50 relative flex flex-col gap-4 overflow-hidden rounded-3xl border p-6 shadow-sm transition-colors has-[a:focus-visible]:ring-[3px]"
				>
					<span
						class={cn(
							buttonVariants({ variant: 'outline', size: 'sm' }),
							'absolute top-6 right-6'
						)}
						aria-hidden="true"
					>
						{m.events_view_event()}
					</span>

					<div class="flex items-center gap-2 pr-28">
						<h2 class="text-2xl leading-7 font-semibold">
							<!-- The after: overlay stretches this link over the whole card. -->
							<a
								href={resolve(
									'/(public)/conversations/[conversation_id]/events/[event_id]',
									{ conversation_id: conversationId, event_id: event.id }
								)}
								class="after:absolute after:inset-0 focus-visible:outline-none"
							>
								{event.name}
							</a>
						</h2>
						{#if status === 'live'}
							<Badge
								variant="outline"
								class="shrink-0 border-green-200 bg-green-50 text-green-700"
							>
								<span
									class="mr-1 h-1.5 w-1.5 animate-pulse rounded-full bg-green-500"
								></span>
								{m.event_status_live()}
							</Badge>
						{:else if status === 'past'}
							<Badge variant="secondary" class="shrink-0"
								>{m.event_status_past()}</Badge
							>
						{:else}
							<Badge variant="outline" class="bg-primary/10 shrink-0"
								>{m.event_status_upcoming()}</Badge
							>
						{/if}
					</div>

					{#if event.description}
						<p class="text-muted-foreground text-base leading-6 font-medium">
							{event.description}
						</p>
					{/if}

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
							<span class="font-medium">{m.events_current_attendees()}</span>
							<span class="text-muted-foreground line-clamp-1"
								>{event.currentAttendance}{event.capacity
									? ` / ${event.capacity}`
									: ''}</span
							>
						</div>
					</div>
				</article>
			{/each}
		</div>
	{/if}

	<a
		href={resolve('/(public)/conversations/[conversation_id]', {
			conversation_id: conversationId
		})}
		class="text-muted-foreground hover:text-foreground text-sm"
	>
		← {m.events_back_to_conversation()}
	</a>
</div>
