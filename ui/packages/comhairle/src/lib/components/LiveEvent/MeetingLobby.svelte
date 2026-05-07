<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import AnimatedLoader from './AnimatedLoader.svelte';
	import type {
		VideoCallParticipant,
		VideoCallStatus
	} from '$lib/services/videoCallService.svelte';

	interface Props {
		title: string;
		scheduledTime: string;
		endedTime?: string;
		startTimeIso?: string;
		endTimeIso?: string;
		participants: VideoCallParticipant[];
		callStatus: VideoCallStatus | null;
		isModerator: boolean;
		hostPresent?: boolean;
		onStartMeeting: () => void;
		onResetCall?: () => void;
	}

	let {
		title,
		scheduledTime,
		endedTime,
		startTimeIso,
		endTimeIso,
		participants,
		callStatus,
		isModerator,
		hostPresent = false,
		onStartMeeting,
		onResetCall
	}: Props = $props();

	let displayParticipants = $derived(participants.slice(0, 3));
	let extraCount = $derived(Math.max(0, participants.length - 3));

	let firstParticipantName = $derived(
		participants[0]?.username ?? participants[0]?.user_id?.slice(0, 8) ?? 'Someone'
	);

	let now = $state(Date.now());
	$effect(() => {
		const id = setInterval(() => (now = Date.now()), 60_000);
		return () => clearInterval(id);
	});

	let endMs = $derived(endTimeIso ? new Date(endTimeIso).getTime() : 0);

	let phase = $derived.by(() => {
		if (callStatus === 'Ended') return 'ended' as const;
		if (endMs && now > endMs && callStatus !== 'InProgress') return 'ended' as const;
		return 'waiting' as const;
	});

	function getInitials(p: VideoCallParticipant): string {
		if (p.username) return p.username.charAt(0).toUpperCase();
		return p.user_id.charAt(0).toUpperCase();
	}

	const avatarColors = [
		'bg-blue-500',
		'bg-emerald-500',
		'bg-amber-500',
		'bg-rose-500',
		'bg-violet-500',
		'bg-cyan-500'
	];

	function getAvatarColor(index: number): string {
		return avatarColors[index % avatarColors.length];
	}
</script>

{#snippet avatarStack()}
	{#if participants.length > 0}
		<div class="inline-flex items-center gap-2">
			<div class="flex items-center">
				{#each displayParticipants as p, i (p.user_id)}
					<div
						class="{getAvatarColor(
							i
						)} ring-background flex h-12 w-12 items-center justify-center rounded-full text-lg font-semibold text-white ring-2 {i >
						0
							? '-ml-3'
							: ''}"
					>
						{getInitials(p)}
					</div>
				{/each}
			</div>
			{#if extraCount > 0}
				<span class="text-foreground text-base font-semibold">+{extraCount} more</span>
			{/if}
		</div>
	{/if}
{/snippet}

{#snippet titleBlock()}
	<div class="flex flex-col items-center gap-3">
		<h1 class="text-foreground text-center text-2xl leading-7 font-semibold">{title}</h1>
		<p class="text-muted-foreground text-center text-xl leading-6 font-semibold">
			{scheduledTime}{#if phase === 'ended' && endedTime}<br />Ended at {endedTime}{/if}
		</p>
	</div>
{/snippet}

<div class="bg-background flex min-h-dvh w-full items-center justify-center px-6">
	<div class="flex w-full max-w-[700px] flex-col items-center gap-8">
		<!-- Logo + heading -->
		<div class="flex flex-col items-center gap-3">
			{#if phase !== 'ended'}
				<AnimatedLoader />
			{/if}

			{#if phase === 'ended'}
				<h2 class="text-foreground text-center text-3xl leading-9 font-semibold">
					The meeting has ended.
				</h2>
			{:else if !isModerator}
				<h2 class="text-foreground text-center text-3xl leading-9 font-semibold">
					We are waiting for the meeting to start...
				</h2>
			{/if}
		</div>

		<!-- Card -->
		<div
			class="flex w-full min-w-[320px] flex-col items-center overflow-hidden rounded-3xl
				{phase === 'ended'
				? 'bg-muted border-border border shadow-sm'
				: !isModerator && phase === 'waiting'
					? 'bg-card border-primary/60 shadow-primary/20 border-[1.5px] shadow-[0_12px_12px_-5px_rgba(115,173,255,0.25)]'
					: 'bg-card border-border border shadow-sm'}"
		>
			<div class="flex w-full flex-col items-center gap-6 px-6 py-8">
				{@render titleBlock()}

				{#if participants.length > 0}
					{@render avatarStack()}
				{/if}

				{#if phase === 'waiting' && !isModerator && hostPresent}
					<p class="text-foreground text-center text-base leading-6 font-medium">
						The admin is about to start the meeting.
					</p>
				{:else if phase === 'waiting' && participants.length > 0}
					<p class="text-foreground text-center text-base leading-6 font-medium">
						{firstParticipantName}{participants.length > 1 ? ' and others are' : ' is'} waiting
						to join.
					</p>
				{/if}
			</div>
		</div>

		<!-- CTA -->
		{#if phase === 'waiting' && isModerator}
			<Button
				variant="primaryDark"
				class="h-12 rounded-full px-8 text-xl font-semibold"
				onclick={onStartMeeting}
			>
				Start meeting
			</Button>
		{:else if phase === 'ended' && isModerator && onResetCall}
			<Button
				variant="outline"
				class="h-10 rounded-full px-6 text-sm font-medium"
				onclick={onResetCall}
			>
				Reset call
			</Button>
		{/if}
	</div>
</div>
