<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import Button from '$lib/components/ui/button/button.svelte';
	import type { VideoCallParticipant } from '$lib/services/videoCallService.svelte';
	import { X, RefreshCw, Clock, Pin, Users } from 'lucide-svelte';

	interface Props {
		open: boolean;
		participants: VideoCallParticipant[];
		defaultMaxPerRoom?: number;
		defaultDuration?: number;
		onClose: () => void;
		onCreate: (config: {
			maxPerRoom: number;
			durationMinutes: number;
			roomAssignments: VideoCallParticipant[][];
		}) => void;
	}

	let {
		open = $bindable(),
		participants,
		defaultMaxPerRoom = 4,
		defaultDuration = 10,
		onClose,
		onCreate
	}: Props = $props();

	let maxPerRoom = $state(defaultMaxPerRoom);
	let durationMinutes = $state(defaultDuration);

	/** Mutable room assignments for drag and drop */
	let roomAssignments = $state<VideoCallParticipant[][]>([]);

	/** Pinned participants stay in their room on reshuffle */
	let pinnedUsers = $state<Set<string>>(new Set());

	let wasOpen = $state(false);

	$effect(() => {
		if (open && !wasOpen) {
			// Dialog just opened — sync defaults and distribute
			maxPerRoom = defaultMaxPerRoom;
			durationMinutes = defaultDuration;
			roomAssignments = [];
			pinnedUsers = new Set();
			if (participants.length > 0) {
				distributeParticipants();
			}
		} else if (!open && wasOpen) {
			// Dialog just closed — reset
			roomAssignments = [];
			pinnedUsers = new Set();
			maxPerRoom = defaultMaxPerRoom;
			durationMinutes = defaultDuration;
		}
		wasOpen = open;
	});

	function shuffle(arr: VideoCallParticipant[]): VideoCallParticipant[] {
		const result = [...arr];
		for (let i = result.length - 1; i > 0; i--) {
			const j = Math.floor(Math.random() * (i + 1));
			[result[i], result[j]] = [result[j], result[i]];
		}
		return result;
	}

	function distributeParticipants() {
		const shuffled = shuffle(participants);
		const count = Math.ceil(shuffled.length / maxPerRoom);
		roomAssignments = Array.from({ length: count }, (_, i) =>
			shuffled.slice(i * maxPerRoom, (i + 1) * maxPerRoom)
		);
	}

	function handleReshuffle() {
		if (pinnedUsers.size === 0) {
			distributeParticipants();
			return;
		}

		// Keep pinned participants in their current rooms, reshuffle the rest
		const roomCount = roomAssignments.length;
		const pinnedByRoom: VideoCallParticipant[][] = Array.from({ length: roomCount }, () => []);
		const unpinned: VideoCallParticipant[] = [];

		for (let r = 0; r < roomCount; r++) {
			for (const p of roomAssignments[r]) {
				if (pinnedUsers.has(p.user_id)) {
					pinnedByRoom[r].push(p);
				} else {
					unpinned.push(p);
				}
			}
		}

		const shuffledUnpinned = shuffle(unpinned);
		let idx = 0;

		// Fill each room: pinned first, then unpinned up to maxPerRoom
		const newAssignments: VideoCallParticipant[][] = pinnedByRoom.map((pinned) => {
			const room = [...pinned];
			while (room.length < maxPerRoom && idx < shuffledUnpinned.length) {
				room.push(shuffledUnpinned[idx++]);
			}
			return room;
		});

		// If there are leftover unpinned, distribute to existing rooms or create new ones
		while (idx < shuffledUnpinned.length) {
			const remaining = shuffledUnpinned.slice(idx, idx + maxPerRoom);
			newAssignments.push(remaining);
			idx += remaining.length;
		}

		roomAssignments = newAssignments.filter((room) => room.length > 0);
	}

	function handleMaxPerRoomChange(e: Event) {
		const next = Math.max(1, Math.min(50, Number((e.target as HTMLInputElement).value) || 1));
		maxPerRoom = next;
		if (roomAssignments.length > 0) {
			handleReshuffle();
		}
	}

	function togglePin(userId: string) {
		const next = new Set(pinnedUsers);
		if (next.has(userId)) {
			next.delete(userId);
		} else {
			next.add(userId);
		}
		pinnedUsers = next;
	}

	function handleCreate() {
		onCreate({ maxPerRoom, durationMinutes, roomAssignments });
		open = false;
	}

	let dragSource: { roomIdx: number; pIdx: number } | null = $state(null);
	let dropTargetRoom: number | null = $state(null);

	function handleDragStart(e: DragEvent, roomIdx: number, pIdx: number) {
		dragSource = { roomIdx, pIdx };
		if (e.dataTransfer) {
			e.dataTransfer.effectAllowed = 'move';
			e.dataTransfer.setData('text/plain', `${roomIdx}:${pIdx}`);
		}
	}

	function handleDragEnd() {
		dragSource = null;
		dropTargetRoom = null;
	}

	function handleDragOver(e: DragEvent, roomIdx: number) {
		e.preventDefault();
		if (e.dataTransfer) {
			e.dataTransfer.dropEffect = 'move';
		}
		dropTargetRoom = roomIdx;
	}

	function handleDragLeave(e: DragEvent, roomIdx: number) {
		// Only clear if leaving the room container itself
		const related = e.relatedTarget as HTMLElement | null;
		const currentTarget = e.currentTarget as HTMLElement;
		if (!related || !currentTarget.contains(related)) {
			if (dropTargetRoom === roomIdx) {
				dropTargetRoom = null;
			}
		}
	}

	function handleDrop(e: DragEvent, targetRoomIdx: number) {
		e.preventDefault();
		dropTargetRoom = null;

		if (!dragSource) return;
		const { roomIdx: srcRoom, pIdx: srcIdx } = dragSource;

		if (srcRoom === targetRoomIdx) {
			dragSource = null;
			return;
		}

		// Move participant
		const participant = roomAssignments[srcRoom][srcIdx];
		const newAssignments = roomAssignments.map((room) => [...room]);
		newAssignments[srcRoom].splice(srcIdx, 1);
		newAssignments[targetRoomIdx].push(participant);

		// Remove empty rooms
		roomAssignments = newAssignments.filter((room) => room.length > 0);
		dragSource = null;
	}

	const avatarColors = [
		'bg-blue-600',
		'bg-emerald-500',
		'bg-indigo-500',
		'bg-orange-500',
		'bg-violet-500',
		'bg-cyan-500'
	];

	/** Stable color per participant based on user_id */
	function getColorForParticipant(p: VideoCallParticipant): string {
		let hash = 0;
		const id = p.user_id;
		for (let i = 0; i < id.length; i++) {
			hash = (hash * 31 + id.charCodeAt(i)) | 0;
		}
		return avatarColors[Math.abs(hash) % avatarColors.length];
	}

	function getInitial(p: VideoCallParticipant): string {
		return (p.username ?? p.user_id).charAt(0).toUpperCase();
	}

	function getName(p: VideoCallParticipant): string {
		return p.username ?? p.user_id.slice(0, 8);
	}

	function isDragging(roomIdx: number, pIdx: number): boolean {
		return dragSource?.roomIdx === roomIdx && dragSource?.pIdx === pIdx;
	}
</script>

<Dialog.Root bind:open onOpenChange={(v) => !v && onClose()}>
	<Dialog.Content
		class="flex max-h-[85vh] w-[70vw] flex-col overflow-hidden rounded-3xl p-6 sm:max-w-none sm:p-9"
		showCloseButton={false}
	>
		<button
			class="text-muted-foreground hover:text-foreground absolute top-6 right-6"
			onclick={onClose}
		>
			<X class="h-5 w-5" />
		</button>

		<div class="flex min-h-0 flex-1 flex-col gap-6">
			<!-- Title -->
			<h2 class="text-foreground shrink-0 text-2xl leading-7 font-semibold">
				Create breakout rooms
			</h2>

			<!-- Time left + room size row -->
			<div class="flex shrink-0 flex-wrap items-center gap-x-6 gap-y-2">
				<div class="flex items-center gap-2">
					<Clock class="text-foreground h-5 w-5" />
					<span class="text-foreground text-sm font-normal">Time left</span>
					<input
						type="number"
						bind:value={durationMinutes}
						min={1}
						max={120}
						class="border-input bg-background h-8 w-14 rounded-lg border px-3 text-center text-sm shadow-sm"
					/>
					<span class="text-foreground text-sm font-normal">minutes</span>
				</div>
				<div class="flex items-center gap-2">
					<Users class="text-foreground h-5 w-5" />
					<span class="text-foreground text-sm font-normal">Max per room</span>
					<input
						type="number"
						value={maxPerRoom}
						onchange={handleMaxPerRoomChange}
						min={1}
						max={50}
						class="border-input bg-background h-8 w-14 rounded-lg border px-3 text-center text-sm shadow-sm"
					/>
					<span class="text-foreground text-sm font-normal">people</span>
				</div>
			</div>

			<!-- Rooms container (dark blue) -->
			{#if participants.length === 0}
				<div
					class="flex flex-1 items-center justify-center rounded-2xl border border-dashed p-8"
				>
					<p class="text-muted-foreground text-sm">
						No participants in the call yet. Participants need to join before you can
						create breakout rooms.
					</p>
				</div>
			{:else if roomAssignments.length > 0}
				<div
					class="bg-sidebar flex min-h-0 flex-1 flex-col gap-3 overflow-y-auto rounded-2xl p-3 sm:p-4"
				>
					{#each roomAssignments as room, roomIdx}
						<div
							class="flex items-center gap-2 rounded-lg border p-5 transition-colors {dropTargetRoom ===
							roomIdx
								? 'bg-accent border-ring ring-ring ring-2'
								: 'bg-muted'}"
							role="group"
							ondragover={(e) => handleDragOver(e, roomIdx)}
							ondragleave={(e) => handleDragLeave(e, roomIdx)}
							ondrop={(e) => handleDrop(e, roomIdx)}
						>
							<!-- Room name -->
							<div
								class="text-foreground line-clamp-1 w-24 shrink-0 text-base leading-6 font-semibold"
							>
								Room #{roomIdx + 1}
							</div>

							<!-- Participant pills -->
							<div class="flex flex-1 flex-wrap gap-3">
								{#each room as p, pIdx (p.user_id)}
									<div
										class="bg-background inline-flex cursor-grab items-center gap-1.5 rounded-full px-2 py-1 shadow-[0px_1px_2px_-1px_rgba(0,0,0,0.10),0px_1px_3px_0px_rgba(0,0,0,0.10)] transition-opacity active:cursor-grabbing {isDragging(
											roomIdx,
											pIdx
										)
											? 'opacity-30'
											: 'opacity-100'}"
										draggable="true"
										ondragstart={(e) => handleDragStart(e, roomIdx, pIdx)}
										ondragend={handleDragEnd}
									>
										<div
											class="{getColorForParticipant(
												p
											)} flex h-6 w-6 items-center justify-center rounded-full text-xs font-medium text-white uppercase"
										>
											{getInitial(p)}
										</div>
										<span class="text-foreground text-sm font-medium">
											{getName(p)}
										</span>
										<button
											class="flex h-5 w-5 items-center justify-center rounded-full transition-colors {pinnedUsers.has(
												p.user_id
											)
												? 'text-primary bg-primary/10'
												: 'text-muted-foreground hover:text-foreground'}"
											onclick={(e) => {
												e.stopPropagation();
												togglePin(p.user_id);
											}}
											title={pinnedUsers.has(p.user_id)
												? 'Unpin (will move on reshuffle)'
												: 'Pin to this room'}
										>
											<Pin class="h-3.5 w-3.5" />
										</button>
									</div>
								{/each}
							</div>
						</div>
					{/each}

					<!-- Reshuffle button -->
					<div>
						<Button
							variant="primaryDark"
							size="sm"
							class="gap-2 text-sm"
							onclick={handleReshuffle}
						>
							<RefreshCw class="h-4 w-4" />
							Reshuffle
						</Button>
					</div>
				</div>
			{/if}

			<!-- Create button -->
			<div class="flex shrink-0 items-center justify-center pt-2">
				<Button
					variant="primaryDark"
					class="h-10 min-w-32 px-5 text-base font-medium"
					disabled={participants.length === 0 || roomAssignments.length === 0}
					onclick={handleCreate}
				>
					Create
				</Button>
			</div>
		</div>
	</Dialog.Content>
</Dialog.Root>
