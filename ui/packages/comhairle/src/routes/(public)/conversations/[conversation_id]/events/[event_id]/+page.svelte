<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import Button from '$lib/components/ui/button/button.svelte';
	import { apiClient } from '@crown-shy/api-client/client';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	let conversationId = $derived(data.conversationId);
	let event = $derived(data.event);
	let attendances = $derived(data.attendances);
	let user = $derived(data.user);

	let joining = $state(false);
	let error = $state<string | null>(null);

	let status = $derived.by(() => {
		if (!event) return 'unknown';
		const now = Date.now();
		const s = new Date(event.startTime).getTime();
		const e = new Date(event.endTime).getTime();
		if (now < s) return 'upcoming' as const;
		if (now > e) return 'past' as const;
		return 'live' as const;
	});

	let userAttendance = $derived(
		user ? attendances.find((a) => a.userId === user.id) : undefined
	);

	function formatDate(iso: string) {
		return new Date(iso).toLocaleDateString(undefined, {
			weekday: 'long',
			year: 'numeric',
			month: 'long',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function formatDuration(start: string, end: string) {
		const ms = new Date(end).getTime() - new Date(start).getTime();
		const mins = Math.round(ms / 60000);
		if (mins < 60) return `${mins} min`;
		const hours = Math.floor(mins / 60);
		const remMins = mins % 60;
		return remMins > 0 ? `${hours}h ${remMins}m` : `${hours}h`;
	}

	async function registerAttendance() {
		if (!event || !user) return;
		joining = true;
		error = null;
		try {
			await apiClient.CreateEventAttendance({
				params: {
					conversation_id: conversationId,
					event_id: event.id
				},
				body: { role: 'attendee' }
			});
			// Reload to refresh attendance data
			window.location.reload();
		} catch (e: any) {
			error = e?.message || 'Failed to register';
		} finally {
			joining = false;
		}
	}
</script>

<svelte:head>
	<title>{event?.name ?? 'Event'}</title>
</svelte:head>

<div class="mx-auto max-w-3xl py-8">
	<a
		href="/conversations/{conversationId}/events"
		class="mb-4 inline-block text-sm text-muted-foreground hover:text-foreground"
	>
		← All events
	</a>

	{#if !event}
		<div class="rounded-xl border border-dashed border-border p-12 text-center">
			<p class="text-muted-foreground">Event not found.</p>
		</div>
	{:else}
		<div class="space-y-6">
			<!-- Header -->
			<div>
				<div class="flex items-center gap-3">
					<h1 class="text-2xl font-bold">{event.name}</h1>
					{#if status === 'live'}
						<span class="inline-flex items-center gap-1 rounded-full bg-green-500/10 px-2.5 py-1 text-xs font-medium text-green-600">
							<span class="h-1.5 w-1.5 animate-pulse rounded-full bg-green-500"></span>
							Live Now
						</span>
					{:else if status === 'past'}
						<span class="rounded-full bg-muted px-2.5 py-1 text-xs text-muted-foreground">Past</span>
					{:else}
						<span class="rounded-full bg-primary/10 px-2.5 py-1 text-xs font-medium text-primary">Upcoming</span>
					{/if}
				</div>
				{#if event.description}
					<p class="mt-2 text-muted-foreground">{event.description}</p>
				{/if}
			</div>

			<!-- Details card -->
			<div class="rounded-xl border border-border bg-card p-5">
				<dl class="grid gap-4 sm:grid-cols-2">
					<div>
						<dt class="text-xs font-medium uppercase tracking-wide text-muted-foreground">Starts</dt>
						<dd class="mt-1 text-sm">{formatDate(event.startTime)}</dd>
					</div>
					<div>
						<dt class="text-xs font-medium uppercase tracking-wide text-muted-foreground">Ends</dt>
						<dd class="mt-1 text-sm">{formatDate(event.endTime)}</dd>
					</div>
					<div>
						<dt class="text-xs font-medium uppercase tracking-wide text-muted-foreground">Duration</dt>
						<dd class="mt-1 text-sm">{formatDuration(event.startTime, event.endTime)}</dd>
					</div>
					<div>
						<dt class="text-xs font-medium uppercase tracking-wide text-muted-foreground">Attendance</dt>
						<dd class="mt-1 text-sm">
							{event.currentAttendance} registered{event.capacity ? ` / ${event.capacity} capacity` : ''}
						</dd>
					</div>
					<div>
						<dt class="text-xs font-medium uppercase tracking-wide text-muted-foreground">Signup Mode</dt>
						<dd class="mt-1 text-sm capitalize">{event.signupMode}</dd>
					</div>
				</dl>
			</div>

			<!-- Actions -->
			<div class="flex flex-wrap gap-3">
				{#if status === 'live'}
					<Button
						variant="default"
						onclick={() => goto(`/conversations/${conversationId}/events/${event.id}/live`)}
					>
						Join Live Event
					</Button>
				{/if}

				{#if status === 'upcoming' && !userAttendance && user}
					<Button variant="default" onclick={registerAttendance} disabled={joining}>
						{joining ? 'Registering…' : 'Register to Attend'}
					</Button>
				{/if}

				{#if userAttendance}
					<span class="inline-flex items-center rounded-full bg-green-500/10 px-3 py-1.5 text-sm font-medium text-green-600">
						✓ You're registered ({userAttendance.role})
					</span>
				{/if}

				{#if !user && status !== 'past'}
					<p class="text-sm text-muted-foreground">
						Log in to register for this event.
					</p>
				{/if}
			</div>

			{#if error}
				<p class="text-sm text-destructive">{error}</p>
			{/if}

			<!-- Attendees list -->
			{#if attendances.length > 0}
				<div>
					<h2 class="mb-3 text-sm font-semibold">
						Registered Attendees ({attendances.length})
					</h2>
					<div class="space-y-2">
						{#each attendances as attendance (attendance.id)}
							<div class="flex items-center gap-3 rounded-lg border border-border p-2.5">
								<div class="flex h-8 w-8 items-center justify-center rounded-full bg-primary/10 text-xs font-medium text-primary">
									{attendance.userId.charAt(0).toUpperCase()}
								</div>
								<div class="min-w-0 flex-1">
									<span class="text-sm">{attendance.userId}</span>
									<span class="ml-2 text-xs text-muted-foreground capitalize">{attendance.role}</span>
								</div>
							</div>
						{/each}
					</div>
				</div>
			{/if}
		</div>
	{/if}
</div>
