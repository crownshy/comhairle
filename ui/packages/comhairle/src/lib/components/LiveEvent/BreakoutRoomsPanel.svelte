<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import type { BreakoutRoomDisplay } from './types';
	import { MoveUpRight, Megaphone, CircleStop } from 'lucide-svelte';

	interface Props {
		rooms: BreakoutRoomDisplay[];
		timeLeftFormatted: string;
		isModerator: boolean;
		onEnterRoom: (roomIndex: number) => void;
		onAddTime: (minutes: number) => void;
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
		'bg-emerald-500',
		'bg-primary',
		'bg-indigo-500',
		'bg-orange-500',
		'bg-rose-500',
		'bg-cyan-500'
	];

	function getInitial(name: string | null, fallback: string): string {
		if (name) return name.charAt(0).toUpperCase();
		return fallback.charAt(0).toUpperCase();
	}
</script>

<div class="flex h-full flex-col overflow-hidden">
	<!-- Time left + chips -->
	<div class="flex items-center justify-center gap-2.5 px-5">
		<span class="text-ring text-sm font-medium">
			Time left&nbsp;&nbsp;{timeLeftFormatted}
		</span>
		{#if isModerator}
			<div class="flex items-center gap-1.5">
				<button
					class="bg-primary/20 text-ring hover:bg-primary/30 h-6 cursor-pointer rounded-full px-2 text-xs font-medium shadow-sm transition-colors"
					onclick={() => onAddTime(-1)}
				>
					-1min
				</button>
				<button
					class="bg-primary/20 text-ring hover:bg-primary/30 h-6 cursor-pointer rounded-full px-2 text-xs font-medium shadow-sm transition-colors"
					onclick={() => onAddTime(1)}
				>
					+1min
				</button>
				<button
					class="bg-primary/20 text-ring hover:bg-primary/30 h-6 cursor-pointer rounded-full px-2 text-xs font-medium shadow-sm transition-colors"
					onclick={() => onAddTime(2)}
				>
					+2min
				</button>
			</div>
		{/if}
	</div>

	<!-- Room list -->
	<div class="flex flex-1 flex-col gap-2 overflow-y-auto p-5">
		{#each rooms as room (room.index)}
			<div
				class="bg-card border-border flex flex-col overflow-hidden rounded-[10px] border shadow-sm"
			>
				<div class="flex flex-col gap-2 px-5 py-4">
					<span class="text-foreground w-28 truncate text-base leading-6 font-semibold">
						{room.name}
					</span>

					<!-- Participant list (horizontal wrap) -->
					<div class="flex flex-wrap items-start gap-5">
						{#each room.participants as p, i (p.user_id)}
							<div class="flex items-center gap-1.5">
								<div
									class="{avatarColors[
										i % avatarColors.length
									]} flex h-5 w-5 items-center justify-center rounded-full text-xs font-medium text-white uppercase"
								>
									{getInitial(p.username, p.user_id)}
								</div>
								<span class="text-foreground text-xs font-medium">
									{p.username ?? p.user_id.slice(0, 8)}
								</span>
							</div>
						{/each}
					</div>
				</div>

				<!-- Card footer: Enter -->
				<div class="border-border flex items-center border-t p-1">
					<button
						class="text-foreground hover:bg-muted flex flex-1 items-center justify-center gap-2 rounded-lg px-3 py-2 text-xs font-medium"
						onclick={() => onEnterRoom(room.index)}
					>
						<MoveUpRight class="h-4 w-4" />
						Enter
						{#if room.hasAssistanceRequest}
							<span class="bg-destructive h-2 w-2 shrink-0 rounded-full"></span>
						{/if}
					</button>
				</div>
			</div>
		{/each}
	</div>

	<!-- Footer controls -->
	<div class="flex flex-col gap-1.5 p-5">
		<Button
			variant="primaryDark"
			class="h-10 w-full text-sm font-medium"
			onclick={onBroadcastMessage}
		>
			<Megaphone class=" h-4 w-4" />
			Broadcast message
		</Button>
		<Button
			variant="outline"
			class="border-input text-destructive hover:bg-destructive/5 hover:text-destructive h-10 w-full"
			onclick={onEndSession}
		>
			<CircleStop class=" h-4 w-4" />
			End breakout session
		</Button>
	</div>
</div>
