<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import Button from '$lib/components/ui/button/button.svelte';
	import type { VideoCallParticipant } from '$lib/services/videoCallService.svelte';
	import { isFacilitatorRole, peopleOutsideRooms } from '$lib/utils/breakoutRoomAssignments';
	import X from '@lucide/svelte/icons/x';
	import RefreshCw from '@lucide/svelte/icons/refresh-cw';
	import Clock from '@lucide/svelte/icons/clock';
	import Pin from '@lucide/svelte/icons/pin';
	import Users from '@lucide/svelte/icons/users';
	import Crown from '@lucide/svelte/icons/crown';

	interface Props {
		open: boolean;
		participants: VideoCallParticipant[];
		defaultMaxPerRoom?: number;
		defaultDuration?: number;
		/** When provided, the dialog opens with this layout instead of auto-distributing. */
		initialAssignments?: VideoCallParticipant[][];
		/** Show a per-person moderator (crown) toggle. Used for pre-assignment. */
		enableModerators?: boolean;
		/** user_ids that start out flagged as moderators. */
		moderatorIds?: string[];
		/**
		 * Label facilitators and moderators (by their event role), read-only, and count the
		 * rooms that have none. Used in the live call.
		 */
		showFacilitators?: boolean;
		/** Show the "Time left" duration input. Off for pre-assignment (no live timer). */
		showDuration?: boolean;
		/** Label for the confirm button. */
		confirmLabel?: string;
		title?: string;
		/**
		 * Keep the number of rooms as it is: no reshuffle or room size, and a room you empty
		 * stays in the list. Used to edit rooms that are already open in the call.
		 */
		fixedRooms?: boolean;
		/**
		 * Show a Plenary row above the rooms for people who stay in the main call. Anyone not
		 * placed in a room starts there.
		 */
		showPlenary?: boolean;
		/** The person assigning. Shown as "You", starting in Plenary, so they can place themselves. */
		currentUser?: VideoCallParticipant;
		onClose: () => void;
		onCreate: (config: {
			maxPerRoom: number;
			durationMinutes: number;
			roomAssignments: VideoCallParticipant[][];
			plenary: VideoCallParticipant[];
			moderatorIds: string[];
		}) => void;
	}

	let {
		open = $bindable(),
		participants,
		defaultMaxPerRoom = 4,
		defaultDuration = 10,
		initialAssignments,
		enableModerators = false,
		moderatorIds,
		showFacilitators = false,
		showDuration = true,
		confirmLabel = 'Create',
		title = 'Create breakout rooms',
		fixedRooms = false,
		showPlenary = false,
		currentUser,
		onClose,
		onCreate
	}: Props = $props();

	let maxPerRoom = $state(defaultMaxPerRoom);
	let durationMinutes = $state(defaultDuration);

	/** Mutable room assignments for drag and drop */
	let roomAssignments = $state<VideoCallParticipant[][]>([]);

	/** People staying in the main call. Only used when showPlenary is on. */
	let plenary = $state<VideoCallParticipant[]>([]);

	/** Row index the drag handlers use for the Plenary row, so room indices stay as they are. */
	const PLENARY_ROW = -1;

	/** Pinned participants stay in their room on reshuffle */
	let pinnedUsers = $state<Set<string>>(new Set());

	/** user_ids currently flagged as room moderators */
	let moderators = $state<Set<string>>(new Set());

	let wasOpen = $state(false);

	$effect(() => {
		if (open && !wasOpen) {
			// Dialog just opened — sync defaults and lay out participants
			maxPerRoom = defaultMaxPerRoom;
			durationMinutes = defaultDuration;
			pinnedUsers = new Set();
			moderators = new Set(moderatorIds ?? []);
			plenary = [];
			// A planned room whose people are not on the call would only be created empty,
			// unless the rooms are already open and their numbering has to stay.
			const plannedRooms = fixedRooms
				? (initialAssignments ?? [])
				: (initialAssignments ?? []).filter((room) => room.length > 0);
			if (plannedRooms.length > 0) {
				roomAssignments = plannedRooms.map((room) => [...room]);
			} else if (participants.length > 0) {
				distributeParticipants();
			} else {
				roomAssignments = [];
			}
			if (showPlenary) {
				const everyone = currentUser ? [currentUser, ...participants] : participants;
				plenary = peopleOutsideRooms(everyone, roomAssignments);
			}
		} else if (!open && wasOpen) {
			// Dialog just closed — reset
			roomAssignments = [];
			plenary = [];
			pinnedUsers = new Set();
			moderators = new Set();
			maxPerRoom = defaultMaxPerRoom;
			durationMinutes = defaultDuration;
		}
		wasOpen = open;
	});

	function withToggled(userIds: Set<string>, userId: string): Set<string> {
		return userIds.has(userId)
			? new Set([...userIds].filter((id) => id !== userId))
			: new Set([...userIds, userId]);
	}

	function toggleModerator(userId: string) {
		moderators = withToggled(moderators, userId);
	}

	function leadsRoom(p: VideoCallParticipant): boolean {
		if (enableModerators) return moderators.has(p.user_id);
		return showFacilitators && isFacilitatorRole(p.role);
	}

	let roomsMissingModerator = $derived(
		enableModerators || showFacilitators
			? roomAssignments.filter((room) => !room.some(leadsRoom)).length
			: 0
	);

	function shuffle(arr: VideoCallParticipant[]): VideoCallParticipant[] {
		const result = [...arr];
		for (let i = result.length - 1; i > 0; i--) {
			const j = Math.floor(Math.random() * (i + 1));
			[result[i], result[j]] = [result[j], result[i]];
		}
		return result;
	}

	function distributeParticipants() {
		const keptInPlenary = new Set(plenary.map((p) => p.user_id));
		const shuffled = shuffle(participants.filter((p) => !keptInPlenary.has(p.user_id)));
		// At least one room, so there is somewhere to drag people back to from Plenary.
		const count = Math.max(1, Math.ceil(shuffled.length / maxPerRoom));
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

		roomAssignments = withoutEmptyRooms(newAssignments);
	}

	/** Drops empty rooms but keeps one, so there is always a room to drag someone into. */
	function withoutEmptyRooms(rooms: VideoCallParticipant[][]): VideoCallParticipant[][] {
		const filled = rooms.filter((room) => room.length > 0);
		return filled.length > 0 ? filled : [[]];
	}

	function handleMaxPerRoomChange(e: Event) {
		const next = Math.max(1, Math.min(50, Number((e.target as HTMLInputElement).value) || 1));
		maxPerRoom = next;
		if (roomAssignments.length > 0) {
			handleReshuffle();
		}
	}

	function togglePin(userId: string) {
		pinnedUsers = withToggled(pinnedUsers, userId);
	}

	let nobodyInARoom = $derived(roomAssignments.every((room) => room.length === 0));

	function handleCreate() {
		onCreate({
			maxPerRoom,
			durationMinutes,
			roomAssignments: fixedRooms
				? roomAssignments
				: roomAssignments.filter((room) => room.length > 0),
			plenary,
			moderatorIds: [...moderators]
		});
		open = false;
	}

	function peopleInRow(rowIdx: number): VideoCallParticipant[] {
		return rowIdx === PLENARY_ROW ? plenary : roomAssignments[rowIdx];
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

		const participant = peopleInRow(srcRoom)[srcIdx];
		const newAssignments = roomAssignments.map((room) => [...room]);
		const newPlenary = [...plenary];
		const rowOf = (rowIdx: number) =>
			rowIdx === PLENARY_ROW ? newPlenary : newAssignments[rowIdx];
		rowOf(srcRoom).splice(srcIdx, 1);
		rowOf(targetRoomIdx).push(participant);

		plenary = newPlenary;
		roomAssignments = fixedRooms ? newAssignments : withoutEmptyRooms(newAssignments);
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

	function isCurrentUser(p: VideoCallParticipant): boolean {
		return p.user_id === currentUser?.user_id;
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
				{title}
			</h2>

			<!-- Time left + room size row -->
			<div class="flex shrink-0 flex-wrap items-center gap-x-6 gap-y-2">
				{#if showDuration}
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
				{/if}
				{#if !fixedRooms}
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
				{/if}
			</div>

			{#if roomsMissingModerator > 0 && roomAssignments.length > 0}
				<p class="shrink-0 text-sm text-amber-600">
					{#if enableModerators}
						{roomsMissingModerator} room{roomsMissingModerator === 1 ? '' : 's'} without a
						moderator. Use the crown to assign one per room.
					{:else}
						{roomsMissingModerator} room{roomsMissingModerator === 1 ? '' : 's'} without a
						facilitator.
					{/if}
				</p>
			{/if}

			<!-- Rooms container (dark blue) -->
			{#if roomAssignments.length === 0}
				<div
					class="flex flex-1 items-center justify-center rounded-2xl border border-dashed p-8"
				>
					<p class="text-muted-foreground text-sm">
						No one to assign yet. People need to sign up or join before you can create
						breakout rooms.
					</p>
				</div>
			{:else if roomAssignments.length > 0}
				<div
					class="bg-sidebar flex min-h-0 flex-1 flex-col gap-3 overflow-y-auto rounded-2xl p-3 sm:p-4"
				>
					{#if showPlenary}
						{@render row(
							'Plenary',
							plenary,
							PLENARY_ROW,
							'Drag someone here to keep them in the main call'
						)}
					{/if}
					{#each roomAssignments as room, roomIdx (roomIdx)}
						{@render row(`Room #${roomIdx + 1}`, room, roomIdx, 'Drag someone here')}
					{/each}

					{#if !fixedRooms}
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
					{/if}
				</div>
			{/if}

			<!-- Create button -->
			<div class="flex shrink-0 items-center justify-center pt-2">
				<Button
					variant="primaryDark"
					class="h-10 min-w-32 px-5 text-base font-medium"
					disabled={roomAssignments.length === 0 || (!fixedRooms && nobodyInARoom)}
					onclick={handleCreate}
				>
					{confirmLabel}
				</Button>
			</div>
		</div>
	</Dialog.Content>
</Dialog.Root>

{#snippet row(label: string, people: VideoCallParticipant[], rowIdx: number, emptyText: string)}
	<div
		class="flex items-center gap-2 rounded-lg border p-5 transition-colors {dropTargetRoom ===
		rowIdx
			? 'bg-accent border-ring ring-ring ring-2'
			: 'bg-muted'}"
		role="group"
		aria-label={label}
		ondragover={(e) => handleDragOver(e, rowIdx)}
		ondragleave={(e) => handleDragLeave(e, rowIdx)}
		ondrop={(e) => handleDrop(e, rowIdx)}
	>
		<div class="text-foreground line-clamp-1 w-24 shrink-0 text-base leading-6 font-semibold">
			{label}
		</div>

		<div class="flex flex-1 flex-wrap gap-3">
			{#each people as p, pIdx (p.user_id)}
				{@render pill(p, rowIdx, pIdx)}
			{:else}
				<span class="text-muted-foreground text-sm">{emptyText}</span>
			{/each}
		</div>
	</div>
{/snippet}

{#snippet pill(p: VideoCallParticipant, rowIdx: number, pIdx: number)}
	<div
		class="bg-background inline-flex cursor-grab items-center gap-1.5 rounded-full px-2 py-1 shadow-[0px_1px_2px_-1px_rgba(0,0,0,0.10),0px_1px_3px_0px_rgba(0,0,0,0.10)] transition-opacity active:cursor-grabbing {isDragging(
			rowIdx,
			pIdx
		)
			? 'opacity-30'
			: 'opacity-100'}"
		draggable="true"
		ondragstart={(e) => handleDragStart(e, rowIdx, pIdx)}
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
		{#if isCurrentUser(p)}
			<span class="text-muted-foreground text-xs">You</span>
		{:else if enableModerators}
			<button
				class="flex h-5 w-5 items-center justify-center rounded-full transition-colors {moderators.has(
					p.user_id
				)
					? 'bg-amber-100 text-amber-600'
					: 'text-muted-foreground hover:text-foreground'}"
				onclick={(e) => {
					e.stopPropagation();
					toggleModerator(p.user_id);
				}}
				title={moderators.has(p.user_id)
					? 'Room moderator — click to unset'
					: 'Make room moderator'}
			>
				<Crown class="h-3.5 w-3.5" />
			</button>
		{:else if leadsRoom(p)}
			<span class="text-muted-foreground text-xs">Facilitator</span>
		{/if}
		{#if !fixedRooms && rowIdx !== PLENARY_ROW}
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
		{/if}
	</div>
{/snippet}
