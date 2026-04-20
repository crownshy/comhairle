<script lang="ts">
	import type { BreakoutRoom } from './types';
	import Button from '$lib/components/ui/button/button.svelte';
	import { X, LogIn, FileText } from 'lucide-svelte';

	interface Props {
		rooms: BreakoutRoom[];
		onEnterRoom: (roomId: string) => void;
		onViewTranscript: (roomId: string) => void;
		onClose: () => void;
	}

	let { rooms, onEnterRoom, onViewTranscript, onClose }: Props = $props();
</script>

<div class="flex h-full flex-col">
	<div class="border-border flex items-center justify-between border-b px-4 py-3">
		<h2 class="text-lg font-semibold">Breakout Rooms</h2>
		<button
			class="text-muted-foreground hover:text-foreground rounded-md p-1"
			onclick={onClose}
		>
			<X class="h-4 w-4" />
		</button>
	</div>

	<div class="flex-1 overflow-y-auto p-4">
		{#if rooms.length === 0}
			<p class="text-muted-foreground py-8 text-center text-sm">No breakout rooms active.</p>
		{:else}
			<div class="flex flex-col gap-3">
				{#each rooms as room (room.id)}
					<div class="border-border bg-muted/30 rounded-lg border p-4">
						<div class="mb-2 flex items-center justify-between">
							<h3 class="text-sm font-semibold">{room.name}</h3>
							<span class="text-muted-foreground text-xs">
								{room.participants.length} participants
							</span>
						</div>

						<p class="text-muted-foreground mb-3 line-clamp-2 text-xs">
							{room.participants.join(', ') || 'No participants yet'}
						</p>

						<div class="flex gap-2">
							<Button
								variant="default"
								size="sm"
								class="flex-1 text-xs"
								onclick={() => onEnterRoom(room.id)}
							>
								<LogIn class="mr-1 h-3 w-3" />
								Enter
							</Button>
							<Button
								variant="outline"
								size="sm"
								class="flex-1 text-xs"
								onclick={() => onViewTranscript(room.id)}
							>
								<FileText class="mr-1 h-3 w-3" />
								View transcript
							</Button>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</div>
</div>
