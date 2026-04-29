<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import type { BreakoutRoomDisplay } from './types';
	import { PenLine, MoveUpRight, AlignLeft, Megaphone, Clock, CircleStop } from 'lucide-svelte';

	interface Props {
		rooms: BreakoutRoomDisplay[];
		timeLeftFormatted: string;
		isModerator: boolean;
		onEnterRoom: (roomIndex: number) => void;
		onAddTime: () => void;
		onEndSession: () => void;
		onBroadcastMessage: () => void;
	}

	let {
		rooms,
		timeLeftFormatted,
		isModerator,
		onEnterRoom,
		onAddTime,
		onEndSession,
		onBroadcastMessage
	}: Props = $props();

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
					<div class="flex flex-wrap gap-x-5 gap-y-2">
						{#each room.participants as p, i}
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
					<button
						class="text-foreground hover:bg-muted flex flex-1 items-center justify-center gap-2 px-4 py-3 text-xs font-medium"
					>
						<AlignLeft class="h-4 w-4" />
						View Transcription
					</button>
				</div>
			</div>
		{/each}
	</div>

	<!-- Footer controls -->
	<div class="flex flex-col items-center gap-3 border-t px-3 pt-4 pb-3">
		<Button
			variant="primaryDark"
			class="h-10 w-full text-sm font-medium"
			onclick={onBroadcastMessage}
		>
			<Megaphone class="mr-1.5 h-4 w-4" />
			Broadcast message
		</Button>
		<div class="flex w-full items-center gap-2">
			<Button
				variant="primaryDark"
				class="h-10 flex-1 text-sm font-medium"
				onclick={onAddTime}
			>
				<Clock class="mr-1.5 h-4 w-4" />
				Add time
			</Button>
			<Button
				variant="destructive"
				class="h-10 flex-1 text-sm font-medium"
				onclick={onEndSession}
			>
				<CircleStop class="mr-1.5 h-4 w-4" />
				End session
			</Button>
		</div>
	</div>
</div>
