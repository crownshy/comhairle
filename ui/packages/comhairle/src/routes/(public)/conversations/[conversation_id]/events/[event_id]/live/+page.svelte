<script lang="ts">
	import { onDestroy } from 'svelte';
	import JitsiMeet from '$lib/components/JitsiMeet/JitsiMeet.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import * as Drawer from '$lib/components/ui/drawer';
	import { ChevronUp, Megaphone, Plus, X as XIcon, Bug } from 'lucide-svelte';
	import type { PageProps } from './$types';
	import type {
		AgendaItem,
		BreakoutRoom,
		ActivePanel,
		RoomContext
	} from '$lib/components/LiveEvent/types';
	import AgendaPanel from '$lib/components/LiveEvent/AgendaPanel.svelte';
	import BreakoutCreateDialog from '$lib/components/LiveEvent/BreakoutCreateDialog.svelte';
	import type { BreakoutConfig } from '$lib/components/LiveEvent/BreakoutCreateDialog.svelte';
	import BreakoutRoomsList from '$lib/components/LiveEvent/BreakoutRoomsList.svelte';
	import BroadcastMessage from '$lib/components/LiveEvent/BroadcastMessage.svelte';
	import DebugPanel from '$lib/components/LiveEvent/DebugPanel.svelte';

	let { data }: PageProps = $props();

	let conversationId = $derived(data.conversationId);
	let eventId = $derived(data.eventId);
	let event = $derived(data.event);
	let jwt = $derived(data.jwt);
	let apiAttendances = $derived(data.attendances);
	let user = $derived(data.user);
	let isModerator = $state(data.isModerator);

	let roomName = $derived(event?.videoMeetingId);

	let jitsiApi: any = $state(null);

	// Panel & room state
	let activePanel: ActivePanel = $state(null);
	let roomContext: RoomContext = $state({ type: 'plenary' });
	let roomLabel = $derived(
		roomContext.type === 'plenary' ? 'Plenary room' : roomContext.roomName
	);

	// Jitsi-synced state
	let jitsiParticipants = $state<Array<{ id: string; displayName: string }>>([]);
	let conferenceJoined = $state(false);
	let audioMuted = $state(false);
	let videoMuted = $state(false);

	// Notification popup state
	let activeNotification = $state<{ message: string; timestamp: number } | null>(null);
	let notificationTimeout: ReturnType<typeof setTimeout> | null = null;

	// Dialog states
	let showBreakoutDialog = $state(false);
	let showBroadcastDialog = $state(false);

	// Agenda items per room
	let plenaryAgenda = $state<AgendaItem[]>([
		{ id: '1', title: 'Ice breaker', isCurrent: true },
		{ id: '2', title: 'Introduction', isCurrent: false },
		{ id: '3', title: 'Discussion', isCurrent: false },
		{ id: '4', title: 'Breakout in groups', isCurrent: false }
	]);

	let breakoutAgendas = $state<Map<string, AgendaItem[]>>(new Map());

	let currentAgendaItems = $derived(
		roomContext.type === 'plenary'
			? plenaryAgenda
			: (breakoutAgendas.get(roomContext.roomId) ?? [])
	);

	// Breakout rooms state
	let breakoutRooms = $state<BreakoutRoom[]>([]);
	let breakoutRoomsActive = $derived(breakoutRooms.length > 0);

	function setCurrentAgendaItem(itemId: string) {
		if (!isModerator) return;
		// TODO: Wire to WebSocket for real-time propagation
		if (roomContext.type === 'plenary') {
			plenaryAgenda = plenaryAgenda.map((item) => ({
				...item,
				isCurrent: item.id === itemId
			}));
		} else {
			const roomId = roomContext.roomId;
			const items = breakoutAgendas.get(roomId) ?? [];
			breakoutAgendas.set(
				roomId,
				items.map((item) => ({ ...item, isCurrent: item.id === itemId }))
			);
			breakoutAgendas = new Map(breakoutAgendas);
		}
	}

	function togglePanel(panel: 'agenda' | 'breakoutRooms' | 'debug') {
		activePanel = activePanel === panel ? null : panel;
	}

	onDestroy(() => {
		if (notificationTimeout) clearTimeout(notificationTimeout);
	});

	function showNotification(message: string) {
		activeNotification = { message, timestamp: Date.now() };
		if (notificationTimeout) clearTimeout(notificationTimeout);
		notificationTimeout = setTimeout(() => {
			activeNotification = null;
		}, 8000);
	}

	function handleApiReady(api: any) {
		jitsiApi = api;

		api.addListener('audioMuteStatusChanged', (data: any) => {
			audioMuted = data.muted;
		});

		api.addListener('videoMuteStatusChanged', (data: any) => {
			videoMuted = data.muted;
		});
	}

	function handleParticipantJoined(data: any) {
		jitsiParticipants = [
			...jitsiParticipants,
			{ id: data.id, displayName: data.displayName || 'Guest' }
		];
	}

	function handleParticipantLeft(data: any) {
		jitsiParticipants = jitsiParticipants.filter((p) => p.id !== data.id);
	}

	async function handleConferenceJoined(data: any) {
		conferenceJoined = true;
	}

	function handleConferenceLeft() {
		conferenceJoined = false;
		jitsiParticipants = [];
	}

	// --- Jitsi raw breakout helpers (for debug panel) ---

	let previousAssignments = $state<Map<string, Set<string>>>(new Map());

	function fisherYatesShuffle<T>(arr: T[]): T[] {
		const a = [...arr];
		for (let i = a.length - 1; i > 0; i--) {
			const j = Math.floor(Math.random() * (i + 1));
			[a[i], a[j]] = [a[j], a[i]];
		}
		return a;
	}

	function buildRawBreakoutRooms(
		participants: Array<{ participantId: string }>,
		maxPerRoom: number
	): Array<{ name: string; participants: string[] }> {
		const ids = participants.map((p) => p.participantId);
		const total = ids.length;
		if (total === 0) return [];
		const roomCount = Math.ceil(total / maxPerRoom);
		const rooms: Array<{ name: string; participants: string[] }> = [];
		for (let i = 0; i < roomCount; i++)
			rooms.push({ name: `Group ${i + 1}`, participants: [] });
		const shuffled = fisherYatesShuffle(ids);
		shuffled.forEach((id, idx) => rooms[idx % roomCount].participants.push(id));
		return rooms;
	}

	async function debugAutoBreakout(maxPerRoom = 6) {
		if (!jitsiApi || !isModerator) return;
		const info = await jitsiApi.getParticipantsInfo();
		const rooms = buildRawBreakoutRooms(info, maxPerRoom);
		if (rooms.length === 0) {
			showNotification('No participants to assign');
			return;
		}
		try {
			jitsiApi.executeCommand('overwriteBreakoutRooms', rooms);
			showNotification(`Created ${rooms.length} raw breakout rooms`);
		} catch (e) {
			console.error('Raw breakout creation failed:', e);
			showNotification('Failed — check console');
		}
	}

	function debugCloseBreakout() {
		if (!jitsiApi || !isModerator) return;
		try {
			jitsiApi.executeCommand('closeBreakoutRooms');
			showNotification('Raw breakout rooms closed');
		} catch (e) {
			console.error('Close failed:', e);
		}
	}

	// --- Breakout room handlers ---

	function handleBreakoutCreate(config: BreakoutConfig) {
		// TODO: Wire to Jitsi API + backend for real room creation
		console.log('[Breakout create]', config);

		const mockRooms: BreakoutRoom[] = [];
		const roomCount = config.rooms ?? 3;
		for (let i = 0; i < roomCount; i++) {
			const roomId = `room-${i + 1}`;
			mockRooms.push({
				id: roomId,
				name: `Room #${i + 1}`,
				participants: []
			});
			breakoutAgendas.set(roomId, [
				{ id: `${roomId}-q1`, title: 'Question 1', isCurrent: true },
				{ id: `${roomId}-q2`, title: 'Question 2', isCurrent: false },
				{ id: `${roomId}-q3`, title: 'Question 3', isCurrent: false }
			]);
		}
		breakoutAgendas = new Map(breakoutAgendas);
		breakoutRooms = mockRooms;
		showNotification(`Created ${roomCount} breakout rooms`);
	}

	function handleBreakoutPreview(config: BreakoutConfig) {
		// TODO: Show preview of room assignments
		console.log('[Breakout preview]', config);
		showNotification('Preview not yet implemented — will show room assignments');
	}

	function handleEnterRoom(roomId: string) {
		const room = breakoutRooms.find((r) => r.id === roomId);
		if (!room) return;
		roomContext = { type: 'breakout', roomId, roomName: room.name };
		// TODO: Actually join the Jitsi breakout room
		showNotification(`Entered ${room.name}`);
	}

	function handleViewTranscript(roomId: string) {
		const room = breakoutRooms.find((r) => r.id === roomId);
		// TODO: Open transcript viewer
		showNotification(`Transcript view for ${room?.name} — not yet implemented`);
	}

	function handleBroadcast(message: string) {
		// TODO: Wire to WebSocket broadcast
		console.log('[Broadcast]', message);
		showNotification(`Broadcast sent: "${message}"`);
	}

	function returnToPlenary() {
		roomContext = { type: 'plenary' };
		// TODO: Actually leave breakout room in Jitsi
		showNotification('Returned to Plenary room');
	}

	function closeAllBreakoutRooms() {
		if (!isModerator) return;
		breakoutRooms = [];
		breakoutAgendas = new Map();
		roomContext = { type: 'plenary' };
		if (jitsiApi) {
			try {
				jitsiApi.executeCommand('closeBreakoutRooms');
			} catch (e) {
				console.error('Close breakout rooms failed:', e);
			}
		}
		showNotification('Breakout rooms closed — participants returning to main room');
	}
</script>

<svelte:head>
	<title>{event?.name ?? 'Live Event'}</title>
</svelte:head>

<div class="relative flex h-dvh flex-col overflow-hidden bg-[#4a4a4a]">
	<!-- Jitsi — full screen -->
	<div class="absolute inset-0">
		<JitsiMeet
			{roomName}
			{jwt}
			onApiReady={handleApiReady}
			onParticipantJoined={handleParticipantJoined}
			onParticipantLeft={handleParticipantLeft}
			onVideoConferenceJoined={handleConferenceJoined}
			onVideoConferenceLeft={handleConferenceLeft}
			startWithAudioMuted={true}
			configOverwrite={{
				toolbarButtons: [
					'microphone',
					'camera',
					'desktop',
					'chat',
					'raisehand',
					'tileview',
					'hangup',
					'fullscreen'
				],
				disableDeepLinking: true,
				hideConferenceSubject: true
			}}
		/>
	</div>

	<!-- Right-edge vertical tabs (desktop) -->
	<div
		class="absolute top-1/2 z-50 hidden -translate-y-1/2 flex-col gap-1.5 md:flex"
		style="right: {activePanel
			? '20rem'
			: '0'}; transition: right 300ms cubic-bezier(0.4, 0, 0.2, 1)"
	>
		<button
			class="flex items-center justify-center rounded-l-lg px-1.5 py-6 text-xs font-semibold shadow-lg transition-colors
				{activePanel === 'agenda'
				? 'bg-primary text-primary-foreground'
				: 'bg-white/95 text-gray-700 hover:bg-white'}"
			style="writing-mode: vertical-rl; text-orientation: mixed"
			onclick={() => togglePanel('agenda')}
		>
			Agenda
		</button>

		{#if breakoutRoomsActive && isModerator}
			<button
				class="flex items-center justify-center rounded-l-lg px-1.5 py-6 text-xs font-semibold shadow-lg transition-colors
					{activePanel === 'breakoutRooms'
					? 'bg-primary text-primary-foreground'
					: 'bg-white/95 text-gray-700 hover:bg-white'}"
				style="writing-mode: vertical-rl; text-orientation: mixed"
				onclick={() => togglePanel('breakoutRooms')}
			>
				Rooms
			</button>
		{/if}

		{#if isModerator}
			<button
				class="flex items-center justify-center rounded-l-lg px-2 py-3 shadow-lg transition-colors
					{activePanel === 'debug' ? 'bg-amber-500 text-white' : 'bg-white/95 text-gray-700 hover:bg-white'}"
				onclick={() => togglePanel('debug')}
				title="Debug panel"
			>
				<Bug class="h-4 w-4" />
			</button>
		{/if}
	</div>

	<!-- Slide-out panel (desktop) -->
	<div
		class="bg-card absolute top-0 bottom-0 z-40 hidden w-80 flex-col shadow-2xl md:flex"
		style="right: 0; transform: translateX({activePanel
			? '0'
			: '100%'}); transition: transform 300ms cubic-bezier(0.4, 0, 0.2, 1)"
	>
		{#if activePanel === 'agenda'}
			<AgendaPanel
				items={currentAgendaItems}
				isFacilitator={isModerator}
				onSetCurrent={setCurrentAgendaItem}
				onClose={() => (activePanel = null)}
			/>
		{:else if activePanel === 'breakoutRooms'}
			<BreakoutRoomsList
				rooms={breakoutRooms}
				onEnterRoom={handleEnterRoom}
				onViewTranscript={handleViewTranscript}
				onClose={() => (activePanel = null)}
			/>
		{:else if activePanel === 'debug'}
			<DebugPanel
				{jitsiApi}
				{audioMuted}
				{videoMuted}
				participantCount={jitsiParticipants.length}
				{conferenceJoined}
				onAutoBreakout={debugAutoBreakout}
				onCloseBreakout={debugCloseBreakout}
				onNotify={showNotification}
				onClose={() => (activePanel = null)}
			/>
		{/if}
	</div>

	<!-- Facilitator floating controls (bottom-right, desktop) -->
	{#if isModerator}
		<div class="absolute right-4 bottom-24 z-50 hidden flex-col gap-2 md:flex">
			{#if !breakoutRoomsActive}
				<Button
					variant="default"
					size="sm"
					class="shadow-lg"
					onclick={() => (showBreakoutDialog = true)}
				>
					<Plus class="mr-1.5 h-4 w-4" />
					Create Breakout Rooms
				</Button>
			{:else}
				<Button
					variant="default"
					size="sm"
					class="shadow-lg"
					onclick={() => (showBroadcastDialog = true)}
				>
					<Megaphone class="mr-1.5 h-4 w-4" />
					Broadcast
				</Button>
				<Button
					variant="secondary"
					size="sm"
					class="shadow-lg"
					onclick={closeAllBreakoutRooms}
				>
					<XIcon class="mr-1.5 h-4 w-4" />
					Close Breakout Rooms
				</Button>
			{/if}
		</div>
	{/if}

	<!-- Mobile drawer -->
	<Drawer.Root>
		<Drawer.Trigger
			class="bg-primary hover:bg-primary/90 fixed bottom-4 left-1/2 z-50 inline-flex -translate-x-1/2 items-center gap-2 rounded-full px-6 py-3 font-semibold text-white shadow-lg transition-colors md:hidden"
		>
			<ChevronUp class="h-4 w-2" />
			<span>Agenda</span>
		</Drawer.Trigger>
		<Drawer.Content class="bg-card flex max-h-[80dvh] flex-col rounded-t-3xl">
			<div class="p-4">
				<AgendaPanel
					items={currentAgendaItems}
					isFacilitator={isModerator}
					onSetCurrent={setCurrentAgendaItem}
					onClose={() => {}}
				/>

				{#if isModerator}
					<div class="border-border mt-4 flex flex-col gap-2 border-t pt-4">
						{#if !breakoutRoomsActive}
							<Button
								variant="default"
								size="sm"
								onclick={() => (showBreakoutDialog = true)}
							>
								<Plus class="mr-1.5 h-4 w-4" />
								Create Breakout Rooms
							</Button>
						{:else}
							<Button
								variant="default"
								size="sm"
								onclick={() => (showBroadcastDialog = true)}
							>
								<Megaphone class="mr-1.5 h-4 w-4" />
								Broadcast
							</Button>
							<Button variant="secondary" size="sm" onclick={closeAllBreakoutRooms}>
								Close Breakout Rooms
							</Button>
						{/if}
					</div>
				{/if}
			</div>
		</Drawer.Content>
	</Drawer.Root>

	<!-- Floating notification popup -->
	{#if activeNotification}
		<div
			class="animate-in fade-in slide-in-from-top-2 pointer-events-auto fixed top-16 left-1/2 z-50 -translate-x-1/2 duration-300"
		>
			<div
				class="bg-card border-border flex max-w-md items-start gap-3 rounded-xl border px-4 py-3 shadow-lg"
			>
				<div class="flex-1">
					<p class="text-foreground text-sm font-medium">Announcement</p>
					<p class="text-muted-foreground mt-0.5 text-sm">{activeNotification.message}</p>
				</div>
				<button
					class="text-muted-foreground hover:text-foreground shrink-0 text-sm"
					onclick={() => (activeNotification = null)}
				>
					✕
				</button>
			</div>
		</div>
	{/if}
</div>

<!-- Dialogs -->
<BreakoutCreateDialog
	open={showBreakoutDialog}
	onOpenChange={(v) => (showBreakoutDialog = v)}
	onPreview={handleBreakoutPreview}
	onCreate={handleBreakoutCreate}
/>

<BroadcastMessage
	open={showBroadcastDialog}
	onOpenChange={(v) => (showBroadcastDialog = v)}
	onSend={handleBroadcast}
/>
