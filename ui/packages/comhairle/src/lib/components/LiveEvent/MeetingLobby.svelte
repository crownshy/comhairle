<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import type {
		VideoCallParticipant,
		VideoCallStatus
	} from '$lib/services/videoCallService.svelte';

	interface Props {
		title: string;
		scheduledTime: string;
		endedTime?: string;
		participants: VideoCallParticipant[];
		callStatus: VideoCallStatus | null;
		isModerator: boolean;
		onStartMeeting: () => void;
	}

	let {
		title,
		scheduledTime,
		endedTime,
		participants,
		callStatus,
		isModerator,
		onStartMeeting
	}: Props = $props();

	let displayParticipants = $derived(participants.slice(0, 3));
	let extraCount = $derived(Math.max(0, participants.length - 3));

	let isStarted = $derived(callStatus === 'InProgress');
	let isEnded = $derived(callStatus === 'Ended');
	let isWaiting = $derived(!isStarted && !isEnded);

	let firstParticipantName = $derived(
		participants[0]?.username ?? participants[0]?.user_id?.slice(0, 8) ?? 'Someone'
	);

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

<div class="bg-background flex min-h-dvh w-full items-center justify-center">
	<div class="flex max-w-2xl flex-col items-center gap-24">
		{#if isEnded}
			<!-- Ended state -->
			<h2 class="text-foreground text-center text-5xl leading-[52px] font-bold">
				The meeting has ended.
			</h2>
			<div class="flex flex-col items-center gap-4">
				<h1 class="text-foreground text-center text-5xl leading-[52px] font-bold">
					{title}
				</h1>
				<p class="text-foreground text-center text-2xl leading-7 font-semibold">
					{scheduledTime}
					{#if endedTime}<br />Ended at {endedTime}{/if}
				</p>
			</div>
		{:else if isModerator && isWaiting}
			<!-- Host waiting state -->
			<div class="flex flex-col items-center gap-14">
				<div class="flex flex-col items-center gap-4">
					<h1 class="text-foreground text-center text-5xl leading-[52px] font-bold">
						{title}
					</h1>
					<p class="text-foreground text-center text-2xl leading-7 font-semibold">
						{scheduledTime}
					</p>
				</div>

				{#if participants.length > 0}
					<div class="flex flex-col items-start gap-3.5">
						<div class="flex items-center gap-2">
							<div class="flex items-center">
								{#each displayParticipants as p, i}
									<div
										class="{getAvatarColor(
											i
										)} -ring-offset-2 ring-background flex h-12 w-12 items-center justify-center rounded-full text-lg font-semibold text-white ring-2 {i >
										0
											? '-ml-3'
											: ''}"
									>
										{getInitials(p)}
									</div>
								{/each}
							</div>
							{#if extraCount > 0}
								<span class="text-foreground text-xl font-semibold"
									>+{extraCount} more</span
								>
							{/if}
						</div>
						<p class="text-foreground w-80 text-xl font-normal">
							{firstParticipantName}{participants.length > 1
								? ' and others are'
								: ' is'} waiting to join.
						</p>
					</div>
				{/if}

				<Button
					variant="primaryDark"
					class="h-12 px-8 text-xl font-semibold"
					onclick={onStartMeeting}
				>
					Start meeting
				</Button>
			</div>
		{:else if !isModerator && isWaiting}
			<!-- Participant waiting state -->
			<h2 class="text-foreground text-center text-5xl leading-[52px] font-bold">
				We are waiting for the meeting to start...
			</h2>

			<div class="flex flex-col items-center gap-14">
				<div class="flex flex-col items-center gap-4">
					<h1 class="text-foreground text-center text-5xl leading-[52px] font-bold">
						{title}
					</h1>
					<p class="text-foreground text-center text-2xl leading-7 font-semibold">
						{scheduledTime}
					</p>
				</div>

				{#if participants.length > 0}
					<div class="flex flex-col items-start gap-3.5">
						<div class="flex items-center gap-2">
							<div class="flex items-center">
								{#each displayParticipants as p, i}
									<div
										class="{getAvatarColor(
											i
										)} -ring-offset-2 ring-background flex h-12 w-12 items-center justify-center rounded-full text-lg font-semibold text-white ring-2 {i >
										0
											? '-ml-3'
											: ''}"
									>
										{getInitials(p)}
									</div>
								{/each}
							</div>
							{#if extraCount > 0}
								<span class="text-foreground text-xl font-semibold"
									>+{extraCount} more</span
								>
							{/if}
						</div>
						<p class="text-foreground w-80 text-xl font-normal">
							{firstParticipantName}{participants.length > 1
								? ' and others are'
								: ' is'} waiting to join.
						</p>
					</div>
				{/if}
			</div>
		{/if}
	</div>
</div>
