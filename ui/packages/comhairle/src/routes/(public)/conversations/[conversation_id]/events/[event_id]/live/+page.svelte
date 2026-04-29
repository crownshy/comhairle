<script lang="ts">
	import { onDestroy, untrack } from 'svelte';
	import JitsiMeet from '$lib/components/JitsiMeet/JitsiMeet.svelte';
	import * as Drawer from '$lib/components/ui/drawer';
	import { formatDateShort, formatTime } from '$lib/utils';
	import { videoCallService } from '$lib/services/videoCallService.svelte';
	import MeetingLobby from '$lib/components/LiveEvent/MeetingLobby.svelte';
	import AgendaPanel from '$lib/components/LiveEvent/AgendaPanel.svelte';
	import BreakoutSessionPanel from '$lib/components/LiveEvent/BreakoutSessionPanel.svelte';
	import BreakoutRoomsPanel from '$lib/components/LiveEvent/BreakoutRoomsPanel.svelte';
	import CreateBreakoutDialog from '$lib/components/LiveEvent/CreateBreakoutDialog.svelte';
	import BroadcastMessageDialog from '$lib/components/LiveEvent/BroadcastMessageDialog.svelte';
	import AddTimeDialog from '$lib/components/LiveEvent/AddTimeDialog.svelte';
	import NoticeDialog from '$lib/components/LiveEvent/NoticeDialog.svelte';
	import SidePanel from '$lib/components/LiveEvent/SidePanel.svelte';
	import type {
		AgendaItem,
		RoomContext,
		PanelTab,
		BreakoutRoomDisplay
	} from '$lib/components/LiveEvent/types';
	import type { VideoCallParticipant } from '$lib/services/videoCallService.svelte';
	import { ChevronUp } from 'lucide-svelte';
	import type { PageProps } from './$types';
	import type { EventAgendaItem } from '@crownshy/api-client/api';

	let { data }: PageProps = $props();

	let conversationId = $derived(data.conversationId);
	let eventId = $derived(data.eventId);
	let event = $derived(data.event);
	let jwt = $derived(data.jwt);
	let user = $derived(data.user);
	let isModerator = $derived(data.isModerator);

	// Mock participants for testing (todo: remove)
	const mockParticipants: VideoCallParticipant[] = [
		{ user_id: 'user-1', username: 'Alice Johnson', role: 'participant' },
		{ user_id: 'user-2', username: 'Bob Smith', role: 'participant' },
		{ user_id: 'user-3', username: 'Cathy Lee', role: 'participant' },
		{ user_id: 'user-4', username: 'Dan Rivera', role: 'participant' },
		{ user_id: 'user-5', username: 'Eva Müller', role: 'participant' },
		{ user_id: 'user-6', username: 'Frank Chen', role: 'participant' },
		{ user_id: 'user-7', username: 'Grace Kim', role: 'participant' },
		{ user_id: 'user-8', username: 'Hiro Tanaka', role: 'participant' },
		{ user_id: 'user-9', username: 'Isla Nguyen', role: 'participant' },
		{ user_id: 'user-10', username: 'Jake Brown', role: 'participant' },
		{ user_id: 'user-11', username: 'Karen Walsh', role: 'participant' },
		{ user_id: 'user-12', username: 'Leo Garcia', role: 'participant' }
	];

	// Reactive reads from videoCallService
	let callStatus = $derived(videoCallService.callStatus);
	let realParticipants = $derived(videoCallService.participants);
	let allParticipants = $derived([...realParticipants, ...mockParticipants]);
	let otherParticipants = $derived(allParticipants.filter((p) => p.user_id !== user?.id));
	let currentStep = $derived(videoCallService.currentAgendaStep);
	let breakoutSession = $derived(videoCallService.breakoutSession);
	let breakoutRooms = $derived(videoCallService.breakoutRooms);
	let assistanceRequests = $derived(videoCallService.assistanceRequests);
	let lastBroadcast = $derived(videoCallService.lastBroadcastMessage);

	// Join WS room on mount (registers presence, not Jitsi join)
	$effect(() => {
		videoCallService.joinCall(eventId);
		return () => videoCallService.leaveCall(eventId);
	});

	// Local UI state
	let hasJoinedCall = $state(false);
	let jitsiApi: any = $state(null);
	let roomContext = $state<RoomContext>('plenary');
	let panelOpen = $state(true);
	let activePanel = $state<PanelTab>('agenda');

	// Mock breakout rooms (for testing with mock participants)
	let mockBreakoutRooms = $state<BreakoutRoomDisplay[]>([]);

	// Dialog states
	let showCreateBreakout = $state(false);
	let showBroadcast = $state(false);
	let showAddTime = $state(false);
	let seenAssistanceRequests = $state<Set<string>>(new Set());

	// Notice queue (assistance requests, broadcasts, time warnings, etc.)
	let noticeQueue = $state<{ message: string; actionLabel?: string; onAction?: () => void }[]>(
		[]
	);
	let breakoutEndingNotified = $state(false);

	// Lightweight toast for admin confirmations
	let toastMessage = $state<string | null>(null);
	let toastTimeout: ReturnType<typeof setTimeout> | null = null;

	// Breakout countdown
	let breakoutTimeRemaining = $state<number | null>(null);
	let countdownInterval: ReturnType<typeof setInterval> | null = null;

	// Map API agenda items to live event format
	function mapApiAgenda(items: EventAgendaItem[]): AgendaItem[] {
		return items.map((item, index) => {
			if ('Basic' in item) {
				return {
					id: String(index + 1),
					title: item.Basic.title,
					type: 'plenary' as const
				};
			} else {
				return {
					id: String(index + 1),
					title: item.BreakoutRoom.prompt || 'Breakout session',
					type: 'breakout' as const,
					breakoutQuestion: item.BreakoutRoom.prompt,
					breakoutDescription: item.BreakoutRoom.instructions,
					durationMinutes: item.BreakoutRoom.estimated_time
				};
			}
		});
	}

	let agendaItems = $derived(mapApiAgenda(event?.agenda ?? []));

	// Derived state
	let meetingPhase = $derived.by(() => {
		if (callStatus === null) return 'loading' as const;
		if (callStatus === 'Ended') return 'ended' as const;
		if (callStatus === 'InProgress' && hasJoinedCall) return 'incall' as const;
		return 'lobby' as const;
	});

	let isBreakoutActive = $derived(breakoutSession !== null);
	let inBreakoutRoom = $derived(typeof roomContext !== 'string');

	let currentAgendaItem = $derived(
		currentStep >= 0 && currentStep < agendaItems.length ? agendaItems[currentStep] : null
	);

	let scheduledTimeText = $derived(
		event
			? `Scheduled for ${formatDateShort(event.startTime)} ${formatTime(event.startTime)}`
			: ''
	);

	let currentJitsiRoom = $derived.by(() => {
		const baseRoom = event?.videoMeetingId ?? '';
		if (typeof roomContext === 'string') return baseRoom;
		return `${baseRoom}-breakout-${roomContext.roomIndex}`;
	});

	let roomChipText = $derived.by(() => {
		if (typeof roomContext === 'string') return 'Plenary room';
		return roomContext.roomName;
	});

	let timeLeftFormatted = $derived.by(() => {
		if (breakoutTimeRemaining === null || breakoutTimeRemaining <= 0) return '0:00';
		const totalSecs = Math.floor(breakoutTimeRemaining / 1000);
		const min = Math.floor(totalSecs / 60);
		const sec = totalSecs % 60;
		return `${min}:${sec.toString().padStart(2, '0')}`;
	});

	let breakoutRoomDisplays = $derived.by((): BreakoutRoomDisplay[] => {
		// Use real rooms from backend if available, otherwise use mock rooms
		if (breakoutRooms.length > 0) {
			return breakoutRooms.map((_, index) => ({
				index,
				name: `Room #${index + 1}`,
				participants: videoCallService.getBreakoutRoomParticipants(index),
				hasAssistanceRequest: videoCallService.hasAssistanceRequest(`room-${index}`),
				assistanceRequestUser: videoCallService.getAssistanceRequestUser(`room-${index}`)
			}));
		}
		return mockBreakoutRooms;
	});

	// Breakout countdown effect
	$effect(() => {
		const session = breakoutSession;
		if (session) {
			const endTime = new Date(session.ends).getTime();
			const update = () => {
				breakoutTimeRemaining = Math.max(0, endTime - Date.now());
			};
			update();
			countdownInterval = setInterval(update, 1000);
		} else {
			breakoutTimeRemaining = null;
			if (countdownInterval) {
				clearInterval(countdownInterval);
				countdownInterval = null;
			}
		}
		return () => {
			if (countdownInterval) {
				clearInterval(countdownInterval);
				countdownInterval = null;
			}
		};
	});

	// Watch for new assistance requests (host only)
	$effect(() => {
		if (!isModerator) return;
		const reqs = assistanceRequests;
		for (const rn of Object.keys(reqs)) {
			const key = `${rn}:${reqs[rn].made_by_user}`;
			if (seenAssistanceRequests.has(key)) continue;
			const match = rn.match(/room-(\d+)/);
			const ri = match ? parseInt(match[1]) : 0;
			const roomName = `Breakout room #${ri + 1}`;
			pushNotice({
				message: `${reqs[rn].made_by_user} from ${roomName} requested help.`,
				actionLabel: `Enter ${roomName}`,
				onAction: () => handleEnterBreakoutRoom(ri)
			});
			seenAssistanceRequests = new Set([...seenAssistanceRequests, key]);
			break;
		}
	});

	// Clear stale seen keys when requests are resolved
	$effect(() => {
		const reqs = assistanceRequests;
		const activeRooms = new Set(Object.keys(reqs));
		const seen = untrack(() => seenAssistanceRequests);
		const cleaned = new Set([...seen].filter((k) => activeRooms.has(k.split(':')[0])));
		if (cleaned.size !== seen.size) {
			seenAssistanceRequests = cleaned;
		}
	});

	// Auto-join when arriving at an already in-progress call
	$effect(() => {
		if (callStatus === 'InProgress' && !hasJoinedCall) {
			hasJoinedCall = true;
		}
	});

	// Auto-switch panel when breakout session starts/ends
	$effect(() => {
		if (isBreakoutActive && isModerator) {
			activePanel = 'breakoutRooms';
		}
		if (!isBreakoutActive && activePanel === 'breakoutRooms') {
			activePanel = 'agenda';
		}
	});

	// Show ending notice at 5 seconds, auto-end at 0
	$effect(() => {
		if (
			breakoutTimeRemaining !== null &&
			breakoutTimeRemaining <= 5000 &&
			breakoutTimeRemaining > 0 &&
			isBreakoutActive &&
			!breakoutEndingNotified
		) {
			pushNotice({
				message: 'Breakout session ending soon. Go back to the plenary room.',
				actionLabel: 'Go back',
				onAction: handleGoBackToPlenary
			});
			breakoutEndingNotified = true;
		}
		if (breakoutTimeRemaining !== null && breakoutTimeRemaining <= 0 && isBreakoutActive) {
			handleGoBackToPlenary();
		}
		if (!isBreakoutActive) {
			breakoutEndingNotified = false;
		}
	});

	// Watch for broadcast messages (participants only — moderator gets toast confirmation)
	$effect(() => {
		if (lastBroadcast) {
			if (!isModerator) {
				pushNotice({ message: lastBroadcast });
			}
			videoCallService.clearLastMessage();
		}
	});

	onDestroy(() => {
		if (countdownInterval) clearInterval(countdownInterval);
		if (toastTimeout) clearTimeout(toastTimeout);
	});

	// Handlers
	function pushNotice(notice: { message: string; actionLabel?: string; onAction?: () => void }) {
		noticeQueue = [...noticeQueue, notice];
	}

	function dismissCurrentNotice() {
		noticeQueue = noticeQueue.slice(1);
	}

	function showToast(message: string) {
		toastMessage = message;
		if (toastTimeout) clearTimeout(toastTimeout);
		toastTimeout = setTimeout(() => {
			toastMessage = null;
		}, 4000);
	}

	// DEV ONLY: reset call state to Waiting
	function devResetCall() {
		videoCallService.changeCallState(eventId, 'Waiting');
		videoCallService.setAgendaItem(eventId, 0);
		videoCallService.endBreakoutSession(eventId);
		hasJoinedCall = false;
		roomContext = 'plenary';
		showCreateBreakout = false;
		console.log('DEV: Call state reset to Waiting');
	}

	function handleStartMeeting() {
		videoCallService.changeCallState(eventId, 'InProgress');
		hasJoinedCall = true;
	}

	function handleJoinMeeting() {
		hasJoinedCall = true;
	}

	function handleSetAgendaItem(index: number) {
		videoCallService.setAgendaItem(eventId, index);
		if (isModerator && agendaItems[index]?.type === 'breakout' && !isBreakoutActive) {
			showCreateBreakout = true;
		}
	}

	function handleNextAgendaItem() {
		const next = currentStep + 1;
		if (next < agendaItems.length) {
			handleSetAgendaItem(next);
		}
	}

	function handleCreateBreakout(config: { maxPerRoom: number; durationMinutes: number }) {
		videoCallService.assignBreakoutRooms(eventId, config.maxPerRoom);
		const ends = new Date(Date.now() + config.durationMinutes * 60 * 1000).toISOString();
		videoCallService.startBreakoutSession(eventId, ends);
		showCreateBreakout = false;

		// Generate mock rooms from mock participants for testing
		const roomCount = Math.ceil(allParticipants.length / config.maxPerRoom);
		mockBreakoutRooms = Array.from({ length: roomCount }, (_, i) => ({
			index: i,
			name: `Room #${i + 1}`,
			participants: allParticipants.slice(i * config.maxPerRoom, (i + 1) * config.maxPerRoom),
			hasAssistanceRequest: false,
			assistanceRequestUser: null
		}));

		// Auto-switch to breakout rooms tab
		activePanel = 'breakoutRooms';
	}

	function handleEnterBreakoutRoom(roomIndex: number) {
		roomContext = {
			type: 'breakout',
			roomIndex,
			roomName: `Breakout room #${roomIndex + 1}`
		};
		if (isModerator) {
			videoCallService.resolveBreakoutRoomAssistanceRequest(eventId, `room-${roomIndex}`);
		}
	}

	function handleLeaveBreakoutRoom() {
		roomContext = 'plenary';
	}

	function handleCallForSupport() {
		const roomId = typeof roomContext !== 'string' ? `room-${roomContext.roomIndex}` : 'room-0'; // Fallback for participants auto-assigned by backend
		videoCallService.requestBreakoutRoomAssistance(eventId, roomId);
		showToast('Support request sent to facilitator');
	}

	function handleBroadcast(message: string) {
		videoCallService.broadcastMessage(eventId, message);
		showToast('Broadcast sent');
	}

	function handleAddTime(minutes: number) {
		const currentEnd = videoCallService.getBreakoutSessionEndTime();
		if (currentEnd) {
			const newEnd = new Date(currentEnd.getTime() + minutes * 60 * 1000).toISOString();
			videoCallService.extendBreakoutSession(eventId, newEnd);
			const msg = `${minutes} minute(s) added to breakout session`;
			videoCallService.broadcastMessage(eventId, msg);
			showToast(`${minutes} minute(s) added`);
		}
	}

	function handleEndBreakoutSession() {
		videoCallService.endBreakoutSession(eventId);
		roomContext = 'plenary';
		mockBreakoutRooms = [];
		activePanel = 'agenda';
	}

	function handleGoBackToPlenary() {
		roomContext = 'plenary';
		if (isModerator) {
			handleEndBreakoutSession();
		}
	}

	function handleApiReady(api: any) {
		jitsiApi = api;
	}
</script>

<svelte:head>
	<title>{event?.name ?? 'Live Event'}</title>
</svelte:head>

{#if meetingPhase === 'loading'}
	<!-- Loading: waiting for call state from server -->
	<div class="bg-background flex min-h-dvh w-full items-center justify-center">
		<div class="flex flex-col items-center gap-8">
			<div class="relative h-16 w-16">
				<div
					class="border-primary/30 border-t-primary absolute inset-0 animate-spin rounded-full border-4"
					style="animation-duration: 1.2s;"
				></div>
			</div>
			<div class="flex flex-col items-center gap-3">
				<h2 class="text-foreground text-2xl font-semibold">
					{event?.name ?? 'Live Event'}
				</h2>
				<p class="text-muted-foreground text-base">Connecting to meeting…</p>
			</div>
		</div>
	</div>
{:else if meetingPhase === 'lobby' || meetingPhase === 'ended'}
	<MeetingLobby
		title={event?.name ?? 'Meeting'}
		scheduledTime={scheduledTimeText}
		endedTime={event ? `${formatDateShort(event.endTime)} ${formatTime(event.endTime)}` : ''}
		participants={otherParticipants}
		{callStatus}
		{isModerator}
		onStartMeeting={handleStartMeeting}
		onJoinMeeting={handleJoinMeeting}
	/>
{:else}
	<!-- In-call: full-width black background, stays in document flow -->
	<div class="bg-sidebar flex h-[calc(100dvh-64px)] w-full flex-col overflow-hidden">
		<div class="flex min-h-0 flex-1">
			<!-- Jitsi area -->
			<div class="relative flex min-h-0 min-w-0 flex-1 flex-col">
				<!-- Header bar -->
				<div
					class="border-sidebar-foreground/20 flex items-end justify-between border-b px-6 pt-4 pb-2"
				>
					<div class="flex items-center gap-3">
						<span class="text-sidebar-foreground text-xl font-medium">
							{event?.name ?? 'Event'}
						</span>
						<div class="flex items-center gap-1.5">
							<span class="bg-destructive h-2.5 w-2.5 rounded-full"></span>
							<span
								class="text-sidebar-foreground text-center text-xs leading-6 font-normal"
							>
								Recording
							</span>
						</div>
					</div>
					<!-- DEV ONLY -->
					<button
						class="bg-destructive text-destructive-foreground hover:bg-destructive/90 rounded px-3 py-1 text-xs font-medium"
						onclick={devResetCall}
					>
						DEV: Reset Call
					</button>
				</div>

				<!-- Jitsi iframe -->
				<div class="relative flex-1 overflow-hidden">
					<JitsiMeet
						roomName={currentJitsiRoom}
						{jwt}
						onApiReady={handleApiReady}
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

					<!-- Room chip overlay -->
					<div class="pointer-events-none absolute inset-x-0 top-0 flex justify-center">
						{#if isBreakoutActive && inBreakoutRoom}
							<div
								class="bg-background pointer-events-auto mt-2 inline-flex items-center justify-between rounded-full px-6 py-2 shadow-md"
							>
								<span class="text-foreground text-sm leading-6 font-medium">
									{roomChipText}
								</span>
								<span
									class="text-muted-foreground ml-3 text-xs leading-6 font-medium"
								>
									Time left {timeLeftFormatted}
								</span>
							</div>
						{:else}
							<div
								class="bg-muted-foreground pointer-events-auto mt-2 inline-flex items-center rounded-full px-4 py-2 shadow-md"
							>
								<span class="text-muted text-sm leading-6 font-medium">
									{roomChipText}
								</span>
							</div>
						{/if}
					</div>
				</div>

				<!-- Host panel toggle tabs (visible when panel closed during breakout) -->
				{#if isModerator && isBreakoutActive && !inBreakoutRoom && !panelOpen}
					<div class="absolute top-12 right-0 z-10 flex flex-col rounded-3xl shadow-lg">
						<button
							class="bg-background h-12 w-48 rounded-t-[20px] px-6 py-3 text-left text-sm font-semibold"
							onclick={() => {
								activePanel = 'agenda';
								panelOpen = true;
							}}
						>
							Agenda
						</button>
						<button
							class="bg-background relative h-12 w-48 rounded-b-[20px] border-t px-6 py-3 text-left text-sm font-semibold"
							onclick={() => {
								activePanel = 'breakoutRooms';
								panelOpen = true;
							}}
						>
							Breakout rooms
							{#if Object.keys(assistanceRequests).length > 0}
								<span
									class="bg-destructive absolute top-3.5 right-4 h-2 w-2 rounded-full"
								></span>
							{/if}
						</button>
					</div>
				{/if}
			</div>

			<!-- Right panel (side-by-side) -->
			{#if panelOpen || !isModerator}
				<div class="h-full w-[360px] shrink-0 p-3">
					<SidePanel
						activeTab={activePanel}
						showTabs={isBreakoutActive && isModerator && !inBreakoutRoom}
						onTabChange={(tab) => (activePanel = tab)}
					>
						{#if isBreakoutActive && (inBreakoutRoom || !isModerator)}
							<BreakoutSessionPanel
								roomName={roomChipText}
								question={currentAgendaItem?.breakoutQuestion}
								description={currentAgendaItem?.breakoutDescription}
								{timeLeftFormatted}
								{isModerator}
								onCallForSupport={handleCallForSupport}
								onLeaveBreakoutRoom={handleLeaveBreakoutRoom}
							/>
						{:else if activePanel === 'breakoutRooms' && breakoutRoomDisplays.length > 0}
							<BreakoutRoomsPanel
								rooms={breakoutRoomDisplays}
								{timeLeftFormatted}
								{isModerator}
								onEnterRoom={handleEnterBreakoutRoom}
								onAddTime={() => (showAddTime = true)}
								onEndSession={handleEndBreakoutSession}
								onBroadcastMessage={() => (showBroadcast = true)}
							/>
						{:else}
							<AgendaPanel
								items={agendaItems}
								{currentStep}
								{isModerator}
								readOnly={isBreakoutActive}
								onSetCurrent={handleSetAgendaItem}
								onNext={handleNextAgendaItem}
							/>
						{/if}
					</SidePanel>
				</div>
			{/if}
		</div>
	</div>

	<!-- Mobile drawer -->
	<Drawer.Root>
		<Drawer.Trigger
			class="bg-primary hover:bg-primary/90 fixed bottom-4 left-1/2 z-50 inline-flex -translate-x-1/2 items-center gap-2 rounded-full px-6 py-3 font-semibold text-white shadow-lg md:hidden"
		>
			<ChevronUp class="h-4 w-4" />
			<span>Agenda</span>
		</Drawer.Trigger>
		<Drawer.Content class="bg-card flex max-h-[80dvh] flex-col rounded-t-3xl">
			<div class="p-4">
				<AgendaPanel
					items={agendaItems}
					{currentStep}
					{isModerator}
					readOnly={isBreakoutActive}
					onSetCurrent={handleSetAgendaItem}
					onNext={handleNextAgendaItem}
				/>
			</div>
		</Drawer.Content>
	</Drawer.Root>
{/if}

<!-- Dialogs -->
<CreateBreakoutDialog
	bind:open={showCreateBreakout}
	participants={allParticipants}
	onClose={() => (showCreateBreakout = false)}
	onCreate={handleCreateBreakout}
/>

<BroadcastMessageDialog
	bind:open={showBroadcast}
	onClose={() => (showBroadcast = false)}
	onSend={handleBroadcast}
/>

<AddTimeDialog
	bind:open={showAddTime}
	{timeLeftFormatted}
	onClose={() => (showAddTime = false)}
	onAddTime={handleAddTime}
/>

{#if noticeQueue.length > 0}
	{@const notice = noticeQueue[0]}
	<NoticeDialog
		open={true}
		message={notice.message}
		actionLabel={notice.actionLabel}
		onAction={notice.onAction}
		onDismiss={dismissCurrentNotice}
	/>
{/if}

<!-- Lightweight toast for confirmations -->
{#if toastMessage}
	<div
		class="animate-in fade-in slide-in-from-top-2 pointer-events-auto fixed top-4 left-1/2 z-50 -translate-x-1/2 duration-300"
	>
		<div
			class="bg-card border-border flex items-center gap-3 rounded-xl border px-4 py-3 shadow-lg"
		>
			<p class="text-foreground text-sm font-medium">{toastMessage}</p>
			<button
				class="text-muted-foreground hover:text-foreground shrink-0 text-sm"
				onclick={() => (toastMessage = null)}
			>
				✕
			</button>
		</div>
	</div>
{/if}
