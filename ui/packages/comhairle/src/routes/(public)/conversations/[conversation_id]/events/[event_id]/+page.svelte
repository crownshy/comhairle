<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { page } from '$app/stores';
	import Button from '$lib/components/ui/button/button.svelte';
	import { Badge } from '$lib/components/ui/badge';
	import { apiClient } from '@crownshy/api-client/client';
	import { formatDateShort, formatTime } from '$lib/utils';
	import { ArrowLeft, CalendarDays, Clock, Users, UserCheck } from 'lucide-svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	let conversationId = $derived(data.conversationId);
	let event = $derived(data.event);
	let attendances = $derived(data.attendances);
	let user = $derived(data.user);

	let redirectError = $derived($page.url.searchParams.get('error'));

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

	let isToday = $derived.by(() => {
		if (!event) return false;
		const start = new Date(event.startTime);
		const now = new Date();
		return (
			start.getFullYear() === now.getFullYear() &&
			start.getMonth() === now.getMonth() &&
			start.getDate() === now.getDate()
		);
	});

	let userAttendance = $derived(user ? attendances.find((a) => a.userId === user.id) : undefined);
	let liveHref = $derived(`/conversations/${conversationId}/events/${event?.id}/live`);

	let canStartMeeting = $derived(
		userAttendance?.role === 'moderator' || userAttendance?.role === 'facilitator'
	);

	/** Reactive clock so the countdown updates without a refresh. Ticks once per minute. */
	let now = $state(Date.now());
	$effect(() => {
		const id = setInterval(() => (now = Date.now()), 60_000);
		return () => clearInterval(id);
	});

	let msUntilStart = $derived(event ? new Date(event.startTime).getTime() - now : 0);

	let countdownText = $derived.by(() => {
		if (msUntilStart <= 0) return '';
		const totalMins = Math.ceil(msUntilStart / 60_000);
		const days = Math.floor(totalMins / (60 * 24));
		const hours = Math.floor((totalMins % (60 * 24)) / 60);
		const mins = totalMins % 60;
		if (days > 0) return `${days}d ${hours}h`;
		if (hours > 0) return `${hours}h ${mins}m`;
		return `${mins}m`;
	});

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
		<!-- Hero section -->
		<div class="flex flex-col items-center gap-6 pb-8">
			<div class="flex items-center gap-3">
				<h1 class="text-foreground text-center text-4xl leading-[48px] font-semibold">
					{event.name}
				</h1>
				{#if status === 'live'}
					<Badge
						variant="outline"
						class="shrink-0 border-green-200 bg-green-50 text-green-700"
					>
						<span class="mr-1 h-1.5 w-1.5 animate-pulse rounded-full bg-green-500"
						></span>
						Live
					</Badge>
				{:else if status === 'past'}
					<Badge variant="secondary" class="shrink-0">Past</Badge>
				{:else}
					<Badge variant="outline" class="bg-primary/10 shrink-0">Upcoming</Badge>
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
			{#if redirectError === 'not-registered'}
				<div
					class="bg-destructive/10 text-destructive mb-2 max-w-md rounded-xl px-4 py-3 text-center text-sm font-medium"
				>
					You need to register for this event before joining the live session.
				</div>
			{/if}

			{#if status === 'live' && userAttendance}
				<Button variant="primaryDark" size="lg" class="h-12 px-8 text-base" href={liveHref}>
					Join meeting
				</Button>
			{:else if canStartMeeting && status === 'upcoming'}
				<Button variant="primaryDark" size="lg" class="h-12 px-8 text-base" href={liveHref}>
					Start meeting{msUntilStart > 0 ? ` (in ${countdownText})` : ''}
				</Button>
				<p class="text-muted-foreground text-xs">
					As a facilitator, you can start the meeting at any time.
				</p>
			{:else if userAttendance && status === 'upcoming'}
				<Button variant="primaryDark" size="lg" class="h-12 px-8 text-base" href={liveHref}>
					Go to lobby{msUntilStart > 0 ? ` (starts in ${countdownText})` : ''}
				</Button>
				<p class="text-muted-foreground text-xs">
					You can wait in the lobby. The meeting will begin when the facilitator starts
					it.
				</p>
			{:else if !userAttendance && user && status !== 'past'}
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
