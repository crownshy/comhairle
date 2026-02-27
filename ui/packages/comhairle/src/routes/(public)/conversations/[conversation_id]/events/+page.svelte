<script lang="ts">
	import { page } from '$app/state';
	import Button from '$lib/components/ui/button/button.svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	let conversationId = $derived(data.conversationId);
	let events = $derived(data.events);

	function formatDate(iso: string) {
		return new Date(iso).toLocaleDateString(undefined, {
			weekday: 'short',
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

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

<div class="mx-auto max-w-3xl py-8">
	<div class="mb-6 flex items-center justify-between">
		<div>
			<h1 class="text-2xl font-bold">Events</h1>
			<p class="mt-1 text-sm text-muted-foreground">
				{events.length} event{events.length !== 1 ? 's' : ''} scheduled
			</p>
		</div>
		<a href="/conversations/{conversationId}" class="text-sm text-muted-foreground hover:text-foreground">
			← Back to conversation
		</a>
	</div>

	{#if events.length === 0}
		<div class="rounded-xl border border-dashed border-border p-12 text-center">
			<p class="text-muted-foreground">No events scheduled for this conversation yet.</p>
		</div>
	{:else}
		<div class="space-y-3">
			{#each events as event (event.id)}
				{@const status = eventStatus(event.startTime, event.endTime)}
				<a
					href="/conversations/{conversationId}/events/{event.id}"
					class="block rounded-xl border border-border bg-card p-4 transition-colors hover:border-primary/30 hover:bg-accent/50"
				>
					<div class="flex items-start justify-between gap-3">
						<div class="min-w-0 flex-1">
							<div class="flex items-center gap-2">
								<h2 class="truncate text-base font-semibold">{event.name}</h2>
								{#if status === 'live'}
									<span class="inline-flex shrink-0 items-center gap-1 rounded-full bg-green-500/10 px-2 py-0.5 text-xs font-medium text-green-600">
										<span class="h-1.5 w-1.5 animate-pulse rounded-full bg-green-500"></span>
										Live
									</span>
								{:else if status === 'past'}
									<span class="rounded-full bg-muted px-2 py-0.5 text-xs text-muted-foreground">Past</span>
								{:else}
									<span class="rounded-full bg-primary/10 px-2 py-0.5 text-xs font-medium text-primary">Upcoming</span>
								{/if}
							</div>
							{#if event.description}
								<p class="mt-1 line-clamp-2 text-sm text-muted-foreground">{event.description}</p>
							{/if}
							<div class="mt-2 flex flex-wrap items-center gap-x-4 gap-y-1 text-xs text-muted-foreground">
								<span>{formatDate(event.startTime)} — {formatDate(event.endTime)}</span>
								<span>{event.currentAttendance} attending{event.capacity ? ` / ${event.capacity} capacity` : ''}</span>
							</div>
						</div>

						{#if status === 'live'}
							<Button variant="default" size="sm" class="shrink-0">
								Join Live
							</Button>
						{:else if status === 'upcoming'}
							<Button variant="outline" size="sm" class="shrink-0">
								View
							</Button>
						{/if}
					</div>
				</a>
			{/each}
		</div>
	{/if}
</div>
