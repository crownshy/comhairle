<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import type { BreakoutRoomDisplay } from './types';
	import {
		PenLine,
		MoveUpRight,
		AlignLeft,
		Megaphone,
		Clock,
		CircleStop,
		ArrowRightLeft
	} from 'lucide-svelte';

	interface Props {
		rooms: BreakoutRoomDisplay[];
		timeLeftFormatted: string;
		isModerator: boolean;
		onEnterRoom: (roomIndex: number) => void;
		onMoveParticipant?: (userId: string, targetRoomIndex: number) => void;
		onAddTime: () => void;
		onEndSession: () => void;
		onBroadcastMessage: () => void;
	}

	let {
		rooms,
		timeLeftFormatted,
		isModerator,
		onEnterRoom,
		onMoveParticipant,
		onAddTime,
		onEndSession,
		onBroadcastMessage
	}: Props = $props();

	/** User ID whose move-to-room picker is currently visible */
	let moveTargetUserId = $state<string | null>(null);

	function toggleMovePicker(userId: string) {
		moveTargetUserId = moveTargetUserId === userId ? null : userId;
	}

	function handleMove(userId: string, targetRoomIndex: number) {
		onMoveParticipant?.(userId, targetRoomIndex);
		moveTargetUserId = null;
	}

	const avatarColors = [
		'bg-blue-500',
		'bg-emerald-500',
		'bg-amber-500',
		'bg-rose-500',
		'bg-violet-500',
		'bg-cyan-500'
	];

	function getInitial(name: string | null, fallback: string): string {
		if (name) return name.charAt(0).toUpperCase();
		return fallback.charAt(0).toUpperCase();
	}
</script>

<div class="flex h-full flex-col overflow-hidden">
	<!-- Time left + controls -->
	<div class="flex flex-col items-center gap-2 pt-1 pb-2">
		{#if isModerator}
			<button
				class="text-primary flex cursor-pointer items-center gap-1 text-xs font-medium hover:underline"
				onclick={() => onAddTime()}
			>
				<span>Time left {timeLeftFormatted}</span>
				<PenLine class="h-2.5 w-2.5" />
			</button>
		{:else}
			<span class="text-primary text-xs font-medium">
				Time left {timeLeftFormatted}
			</span>
		{/if}
	</div>

	<!-- Room list -->
	<div class="flex flex-1 flex-col gap-2 overflow-y-auto px-3">
		{#each rooms as room (room.index)}
			<div
				class="bg-card border-border flex flex-col overflow-hidden rounded-xl border shadow-sm"
			>
				<div class="flex flex-col gap-4 px-5 py-4">
					<div class="flex items-center justify-between">
						<span
							class="text-foreground line-clamp-1 text-base leading-6 font-semibold"
						>
							{room.name}
						</span>
						{#if room.hasAssistanceRequest}
							<span class="bg-destructive h-2 w-2 shrink-0 rounded-full"></span>
						{/if}
					</div>

					<!-- Participant list -->
					<div class="flex flex-col gap-1.5">
						{#each room.participants as p, i (p.user_id)}
							<div class="flex flex-col">
								<div class="flex items-center gap-1.5">
									<div
										class="{avatarColors[
											i % avatarColors.length
										]} flex h-6 w-6 items-center justify-center rounded-full text-xs font-medium text-white uppercase"
									>
										{getInitial(p.username, p.user_id)}
									</div>
									<span class="text-foreground text-sm font-medium">
										{p.username ?? p.user_id.slice(0, 8)}
									</span>
									{#if isModerator && rooms.length > 1}
										<button
											class="text-muted-foreground hover:text-foreground ml-auto flex h-5 w-5 items-center justify-center rounded transition-colors {moveTargetUserId ===
											p.user_id
												? 'text-primary bg-primary/10'
												: ''}"
											title="Move to another room"
											onclick={() => toggleMovePicker(p.user_id)}
										>
											<ArrowRightLeft class="h-3.5 w-3.5" />
										</button>
									{/if}
								</div>
								{#if isModerator && moveTargetUserId === p.user_id}
									<div class="mt-1 ml-7.5 flex flex-wrap items-center gap-1">
										<span class="text-muted-foreground text-xs">Move to:</span>
										{#each rooms as targetRoom (targetRoom.index)}
											{#if targetRoom.index !== room.index}
												<button
													class="bg-primary/10 text-primary hover:bg-primary hover:text-primary-foreground rounded-full px-2.5 py-0.5 text-xs font-medium transition-colors"
													onclick={() =>
														handleMove(p.user_id, targetRoom.index)}
												>
													#{targetRoom.index + 1}
												</button>
											{/if}
										{/each}
									</div>
								{/if}
							</div>
						{/each}
					</div>
				</div>

				<!-- Card footer: Enter | View Transcription -->
				<div class="border-border flex items-center border-t">
					<button
						class="text-foreground hover:bg-muted flex flex-1 items-center justify-center gap-2 px-1 py-3 text-xs font-medium"
						onclick={() => onEnterRoom(room.index)}
					>
						<MoveUpRight class="h-4 w-4" />
						Enter
						{#if room.hasAssistanceRequest}
							<span class="bg-destructive h-2 w-2 shrink-0 rounded-full"></span>
						{/if}
					</button>
					<div class="border-border h-6 border-l"></div>
				</div>
			</div>
		{/each}
	</div>

	<!-- Footer controls -->
	<div class="flex flex-col items-center gap-2 border-t px-3 pt-4 pb-3">
		<Button
			variant="primaryDark"
			class="h-10 w-full text-sm font-medium"
			onclick={onBroadcastMessage}
		>
			<Megaphone class="mr-1.5 h-4 w-4" />
			Broadcast message
		</Button>
		<Button
			variant="destructive"
			class="h-10 w-full text-sm font-medium"
			onclick={onEndSession}
		>
			<CircleStop class="mr-1.5 h-4 w-4" />
			End session
		</Button>
	</div>
</div>
