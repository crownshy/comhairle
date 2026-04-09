<script lang="ts">
	import { page } from '$app/state';
	import { goto, invalidateAll } from '$app/navigation';
	import Button from '$lib/components/ui/button/button.svelte';
	import { Badge } from '$lib/components/ui/badge';
	import { apiClient } from '@crownshy/api-client/client';
	import { ArrowLeft, CalendarDays, Clock, Users, UserCheck } from 'lucide-svelte';
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

	let userAttendance = $derived(user ? attendances.find((a) => a.userId === user.id) : undefined);

	function formatDateShort(iso: string) {
		return new Date(iso).toLocaleDateString(undefined, {
			weekday: 'short',
			month: 'long',
			day: 'numeric'
		});
	}

	function formatTime(iso: string) {
		return new Date(iso).toLocaleTimeString(undefined, {
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
			await apiClient.CreateEventAttendance(
				{ role: 'participant' },
				{
					params: {
						conversation_id: conversationId,
						event_id: event.id
					}
				}
			);
			// Reload to refresh attendance data
			await invalidateAll();
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

<div class="flex flex-col items-center">
	<!-- Back navigation -->
	<div class="w-full max-w-[1280px] px-6 py-12">
		<div class="flex items-center gap-3">
			<Button
				variant="outline"
				size="icon"
				href="/conversations/{conversationId}/events"
				class="h-9 w-9"
			>
				<ArrowLeft class="h-4 w-4" />
			</Button>
			<span class="text-foreground text-sm font-medium">All events</span>
		</div>
	</div>

	{#if !event}
		<div
			class="border-border w-full max-w-[700px] rounded-3xl border border-dashed p-12 text-center"
		>
			<p class="text-muted-foreground">Event not found.</p>
		</div>
	{:else}
		<div class="space-y-6">
			<!-- Header -->
			<div>
				<div class="flex items-center gap-3">
					<h1 class="text-2xl font-bold">{event.name}</h1>
					{#if status === 'live'}
						<span
							class="inline-flex items-center gap-1 rounded-full bg-green-500/10 px-2.5 py-1 text-xs font-medium text-green-600"
						>
							<span class="h-1.5 w-1.5 animate-pulse rounded-full bg-green-500"
							></span>
							Live Now
						</span>
					{:else if status === 'past'}
						<span
							class="bg-muted text-muted-foreground rounded-full px-2.5 py-1 text-xs"
							>Past</span
						>
					{:else}
						<span
							class="bg-primary/10 text-primary rounded-full px-2.5 py-1 text-xs font-medium"
							>Upcoming</span
						>
					{/if}
				</div>
				{#if event.description}
					<p class="text-muted-foreground mt-2">{event.description}</p>
				{/if}
			</div>

			<!-- Details card -->
			<div class="border-border bg-card rounded-xl border p-5">
				<dl class="grid gap-4 sm:grid-cols-2">
					<div>
						<dt
							class="text-muted-foreground text-xs font-medium tracking-wide uppercase"
						>
							Starts
						</dt>
						<dd class="mt-1 text-sm">{formatTime(event.startTime)}</dd>
					</div>
					<div>
						<dt
							class="text-muted-foreground text-xs font-medium tracking-wide uppercase"
						>
							Ends
						</dt>
						<dd class="mt-1 text-sm">{formatTime(event.endTime)}</dd>
					</div>
					<div>
						<dt
							class="text-muted-foreground text-xs font-medium tracking-wide uppercase"
						>
							Duration
						</dt>
						<dd class="mt-1 text-sm">
							{formatDuration(event.startTime, event.endTime)}
						</dd>
					</div>
					<div>
						<dt
							class="text-muted-foreground text-xs font-medium tracking-wide uppercase"
						>
							Attendance
						</dt>
						<dd class="mt-1 text-sm">
							{event.currentAttendance} registered{event.capacity
								? ` / ${event.capacity} capacity`
								: ''}
						</dd>
					</div>
					<div>
						<dt
							class="text-muted-foreground text-xs font-medium tracking-wide uppercase"
						>
							Signup Mode
						</dt>
						<dd class="mt-1 text-sm capitalize">{event.signupMode}</dd>
					</div>
				</dl>
			</div>

			<!-- Actions -->
			<div class="flex flex-wrap gap-3">
				{#if status === 'live' && userAttendance}
					<Button
						variant="default"
						onclick={() =>
							goto(`/conversations/${conversationId}/events/${event.id}/live`)}
					>
						Join Live Event
					</Button>
				{/if}

				{#if status === 'live' && !userAttendance}
					<span
						class="inline-flex items-center rounded-full bg-green-500/10 px-3 py-1.5 text-sm font-medium text-green-600"
					>
						Registration for this event is closed
					</span>
				{/if}

				{#if status === 'upcoming' && !userAttendance && user}
					<Button variant="default" onclick={registerAttendance} disabled={joining}>
						{joining ? 'Registering…' : 'Register to Attend'}
					</Button>
				{/if}

				{#if userAttendance}
					<span
						class="inline-flex items-center rounded-full bg-green-500/10 px-3 py-1.5 text-sm font-medium text-green-600"
					>
						✓ You're registered ({userAttendance.role})
					</span>
				{/if}

				{#if !user && status !== 'past'}
					<p class="text-muted-foreground text-sm">Log in to register for this event.</p>
				{/if}
			</div>

			{#if event.description}
				<p
					class="text-muted-foreground max-w-[1280px] px-6 text-center text-lg leading-7 font-medium"
				>
					{event.description}
				</p>
			{/if}
		</div>

		<!-- Details card -->
		<div class="w-full max-w-[700px] px-6 pt-8 pb-8">
			<div class="bg-card border-border flex rounded-3xl border p-6 shadow-sm">
				<div class="flex flex-1 gap-6">
					<!-- Left column: Date, Time, Duration -->
					<div class="flex flex-col gap-6">
						<div class="flex flex-col gap-1">
							<div class="flex items-center gap-2">
								<CalendarDays class="text-muted-foreground h-4 w-4" />
								<span class="text-muted-foreground text-sm font-medium">Date</span>
							</div>
							<p class="text-foreground line-clamp-1 text-base leading-6 font-medium">
								{formatDateShort(event.startTime)}
							</p>
						</div>

						<div class="flex flex-col gap-1">
							<div class="flex items-center gap-2">
								<Clock class="text-muted-foreground h-4 w-4" />
								<span class="text-muted-foreground text-sm font-medium">Time</span>
							</div>
							<p class="text-foreground line-clamp-1 text-base leading-6 font-medium">
								{formatTime(event.startTime)} - {formatTime(event.endTime)}
							</p>
						</div>

						<div class="flex flex-col gap-1">
							<div class="flex items-center gap-2">
								<span class="text-muted-foreground text-sm font-medium"
									>Duration</span
								>
							</div>
							<p class="text-foreground line-clamp-1 text-base leading-6 font-medium">
								{formatDuration(event.startTime, event.endTime)}
							</p>
						</div>
					</div>

					<!-- Right column: Attendance, Signup Mode -->
					<div class="flex flex-col gap-6">
						<div class="flex flex-col gap-1">
							<div class="flex items-center gap-2">
								<Users class="text-muted-foreground h-4 w-4" />
								<span class="text-muted-foreground text-sm font-medium"
									>Attendance</span
								>
							</div>
							<p class="text-foreground line-clamp-1 text-base leading-6 font-medium">
								{event.currentAttendance} registered{event.capacity
									? ` / ${event.capacity} capacity`
									: ''}
							</p>
						</div>

						<div class="flex flex-col gap-1">
							<div class="flex items-center gap-2">
								<UserCheck class="text-muted-foreground h-4 w-4" />
								<span class="text-muted-foreground text-sm font-medium"
									>Signup Mode</span
								>
							</div>
							<p
								class="text-foreground line-clamp-1 text-base leading-6 font-medium capitalize"
							>
								{event.signupMode}
							</p>
						</div>
					</div>
				</div>
			</div>
		</div>

		<!-- Actions -->
		<div class="flex flex-col items-center gap-4 pt-4 pb-24">
			{#if status === 'live'}
				<Button
					variant="primaryDark"
					size="lg"
					class="h-12 px-8 text-base"
					onclick={() => goto(`/conversations/${conversationId}/events/${event.id}/live`)}
				>
					Join Live Event
				</Button>
			{/if}

			{#if status === 'upcoming' && !userAttendance && user}
				<Button
					variant="primaryDark"
					size="lg"
					class="h-12 px-8 text-base"
					onclick={registerAttendance}
					disabled={joining}
				>
					{joining ? 'Registering…' : 'Register to Attend'}
				</Button>
			{/if}

			{#if userAttendance}
				<span
					class="inline-flex items-center rounded-full bg-green-500/10 px-4 py-2 text-sm font-medium text-green-600"
				>
					✓ You're registered ({userAttendance.role})
				</span>
			{/if}

			{#if !user && status !== 'past'}
				<p class="text-muted-foreground text-sm">Log in to register for this event.</p>
			{/if}

			{#if error}
				<p class="text-destructive text-sm">{error}</p>
			{/if}
		</div>
	{/if}
</div>
