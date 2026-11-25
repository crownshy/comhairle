<script lang="ts">
	import { invalidate } from '$app/navigation';
	import { key } from '$lib/utils/invalidationKey';
	import { page } from '$app/stores';
	import Button from '$lib/components/ui/button/button.svelte';
	import { Badge } from '$lib/components/ui/badge';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { apiClient } from '@crownshy/api-client/client';
	import { formatDateShort, formatTime } from '$lib/utils';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { lobbyHref } from './lobbyHref';
	import ArrowLeft from '@lucide/svelte/icons/arrow-left';
	import CalendarDays from '@lucide/svelte/icons/calendar-days';
	import Clock from '@lucide/svelte/icons/clock';
	import Users from '@lucide/svelte/icons/users';
	import UserCheck from '@lucide/svelte/icons/user-check';
	import Info from '@lucide/svelte/icons/info';
	import { onMount } from 'svelte';
	import type { PageProps } from './$types';
	import { m } from '$lib/paraglide/messages';

	let { data }: PageProps = $props();

	let conversationId = $derived(data.conversationId);
	let event = $derived(data.event);
	let attendances = $derived(data.attendances);
	let totalAttendances = $derived(data.totalAttendances);
	let user = $derived(data.user);

	let redirectError = $derived($page.url.searchParams.get('error'));

	let joining = $state(false);
	let error = $state<string | null>(null);

	/**
	 * Time + date formatting depends on the user's local timezone, which differs between server
	 * (UTC) and client. Render placeholders during SSR/hydration to avoid the visible flash, then
	 * swap to formatted local time after mount.
	 */
	let mounted = $state(false);
	onMount(() => {
		mounted = true;
	});

	let attendanceCount = $derived(event?.currentAttendance ?? totalAttendances ?? 0);

	let status = $derived.by(() => {
		if (!event) return 'unknown';
		const now = Date.now();
		const s = new Date(event.startTime).getTime();
		const e = new Date(event.endTime).getTime();
		if (now < s) return 'upcoming' as const;
		if (now > e) return 'past' as const;
		return 'live' as const;
	});

	let isInviteOnly = $derived(event?.signupMode === 'invite');
	let userAttendance = $derived(user ? attendances.find((a) => a.userId === user.id) : undefined);
	let liveHref = $derived(event ? lobbyHref(conversationId, event) : undefined);

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
		const eventId = event.id;
		const registered = await tryCatchAsync(async () => {
			await apiClient.CreateEventAttendance(
				{ role: 'participant' },
				{
					params: {
						conversation_id: conversationId,
						event_id: eventId
					}
				}
			);
			// Reload to refresh attendance data
			await invalidate(key('public/event'));
		});
		if (registered.err !== null) {
			error =
				registered.err instanceof Error
					? registered.err.message
					: m.event_register_failed();
		}
		joining = false;
	}
</script>

<svelte:head>
	<title>{event?.name ?? m.event_title_fallback()}</title>
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
			<p class="text-muted-foreground">{m.event_not_found()}</p>
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
						{m.event_status_live()}
					</Badge>
				{:else if status === 'past'}
					<Badge variant="secondary" class="shrink-0">{m.event_status_past()}</Badge>
				{:else}
					<Badge variant="outline" class="bg-primary/10 shrink-0"
						>{m.event_status_upcoming()}</Badge
					>
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
								<span class="text-muted-foreground text-sm font-medium"
									>{m.event_date()}</span
								>
							</div>
							{#if mounted}
								<p
									class="text-foreground line-clamp-1 text-base leading-6 font-medium"
								>
									{formatDateShort(event.startTime)}
								</p>
							{:else}
								<Skeleton class="h-6 w-28" />
							{/if}
						</div>

						<div class="flex flex-col gap-1">
							<div class="flex items-center gap-2">
								<Clock class="text-muted-foreground h-4 w-4" />
								<span class="text-muted-foreground text-sm font-medium"
									>{m.event_time()}</span
								>
							</div>
							{#if mounted}
								<p
									class="text-foreground line-clamp-1 text-base leading-6 font-medium"
								>
									{formatTime(event.startTime)} - {formatTime(event.endTime)}
								</p>
							{:else}
								<Skeleton class="h-6 w-36" />
							{/if}
						</div>

						<div class="flex flex-col gap-1">
							<div class="flex items-center gap-2">
								<span class="text-muted-foreground text-sm font-medium"
									>{m.event_duration()}</span
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
									>{m.event_attendance()}</span
								>
							</div>
							<p class="text-foreground line-clamp-1 text-base leading-6 font-medium">
								{event.capacity
									? m.event_registered_count_capacity({
											count: attendanceCount,
											capacity: event.capacity
										})
									: m.event_registered_count({ count: attendanceCount })}
							</p>
						</div>

						<div class="flex flex-col gap-1">
							<div class="flex items-center gap-2">
								<UserCheck class="text-muted-foreground h-4 w-4" />
								<span class="text-muted-foreground text-sm font-medium"
									>{m.event_signup_mode()}</span
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
			{#if redirectError === 'not-registered' && !userAttendance}
				<div
					class="bg-primary/10 text-foreground mb-2 flex max-w-md items-start gap-2 rounded-xl px-4 py-3 text-sm"
				>
					<Info class="text-primary mt-0.5 h-4 w-4 shrink-0" />
					<span>
						{#if isInviteOnly}
							{m.event_needs_invite()}
						{:else if user}
							{m.event_register_to_join()}
						{:else}
							{m.event_log_in_to_join()}
						{/if}
					</span>
				</div>
			{/if}

			{#if status === 'live' && userAttendance}
				<Button variant="primaryDark" size="lg" class="h-12 px-8 text-base" href={liveHref}>
					{m.event_join_meeting()}
				</Button>
			{:else if canStartMeeting && status === 'upcoming'}
				<Button variant="primaryDark" size="lg" class="h-12 px-8 text-base" href={liveHref}>
					{msUntilStart > 0
						? m.event_start_meeting_in({ countdown: countdownText })
						: m.event_start_meeting()}
				</Button>
				<p class="text-muted-foreground text-xs">
					{m.event_facilitator_can_start()}
				</p>
			{:else if userAttendance && status === 'upcoming'}
				<Button variant="primaryDark" size="lg" class="h-12 px-8 text-base" href={liveHref}>
					{msUntilStart > 0
						? m.event_go_to_lobby_starts_in({ countdown: countdownText })
						: m.event_go_to_lobby()}
				</Button>
				<p class="text-muted-foreground text-xs">
					{m.event_lobby_wait()}
				</p>
			{:else if !userAttendance && user && status !== 'past' && isInviteOnly}
				<p class="text-muted-foreground text-base">
					{m.event_invite_only()}
				</p>
			{:else if !userAttendance && user && status !== 'past'}
				<Button
					variant="primaryDark"
					size="lg"
					class="h-12 px-8 text-base"
					onclick={registerAttendance}
					disabled={joining}
				>
					{joining ? m.event_registering() : m.event_register()}
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
				<p class="text-muted-foreground text-sm">
					{isInviteOnly ? m.event_invite_only() : m.event_log_in_to_register()}
				</p>
			{/if}

			{#if error}
				<p class="text-destructive text-sm">{error}</p>
			{/if}
		</div>
	{/if}
</div>
