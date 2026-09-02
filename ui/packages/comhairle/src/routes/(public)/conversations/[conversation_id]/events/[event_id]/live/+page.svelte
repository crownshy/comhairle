<script lang="ts">
	import { onDestroy, untrack } from 'svelte';
	import { slide } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { dev } from '$app/environment';
	import JitsiMeet from '$lib/components/JitsiMeet/JitsiMeet.svelte';
	import { formatDateShort, formatTime } from '$lib/utils';
	import { formatCountdown } from '$lib/utils/formatCountdown';
	import { getJitsiBreakoutRoomId } from '$lib/utils/jitsiBreakoutRooms';
	import { groupParticipantsByRoom } from '$lib/utils/breakoutRoomAssignments';
	import { mapApiAgenda } from '$lib/utils/liveEventAgenda';
	import { videoCallService } from '$lib/services/videoCallService.svelte';
	import MeetingLobby from '$lib/components/LiveEvent/MeetingLobby.svelte';
	import AgendaPanel from '$lib/components/LiveEvent/AgendaPanel.svelte';
	import BreakoutSessionPanel from '$lib/components/LiveEvent/BreakoutSessionPanel.svelte';
	import BreakoutRoomsPanel from '$lib/components/LiveEvent/BreakoutRoomsPanel.svelte';
	import BreakoutEndingDialog from '$lib/components/LiveEvent/BreakoutEndingDialog.svelte';
	import CreateBreakoutDialog from '$lib/components/LiveEvent/CreateBreakoutDialog.svelte';
	import BroadcastMessageDialog from '$lib/components/LiveEvent/BroadcastMessageDialog.svelte';
	import NoticeDialog from '$lib/components/LiveEvent/NoticeDialog.svelte';
	import EndMeetingDialog from '$lib/components/LiveEvent/EndMeetingDialog.svelte';
	import MeetingEndedScreen from '$lib/components/LiveEvent/MeetingEndedScreen.svelte';
	import SidePanel from '$lib/components/LiveEvent/SidePanel.svelte';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import type {
		AgendaItem,
		RoomContext,
		PanelTab,
		BreakoutRoomDisplay
	} from '$lib/components/LiveEvent/types';
	import type { VideoCallParticipant } from '$lib/services/videoCallService.svelte';
	import {
		ChevronLeft,
		ChevronRight,
		MoveUpRight,
		Megaphone,
		CircleStop,
		Check,
		Hand,
		ArrowRight
	} from 'lucide-svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	let conversationId = $derived(data.conversationId);
	let eventId = $derived(data.eventId);
	let event = $derived(data.event);
	let jwt = $derived(data.jwt);
	let user = $derived(data.user);
	let isModerator = $derived(data.isModerator);

	let callStatus = $derived(videoCallService.callStatus);
	let allParticipants = $derived(videoCallService.participants);
	let otherParticipants = $derived(
		allParticipants.filter((p) => {
			if (p.user_id === user?.id) return false;
			if (!isModerator && (p.role === 'moderator' || p.role === 'facilitator')) return false;
			return true;
		})
	);

	/** Participants actually in the Jitsi video (excludes current user + lobby-only users).
	 *  Membership comes from the backend in-video set (reported by each client on join),
	 *  so it is unaffected by users renaming themselves in the Jitsi interface. */
	let inCallParticipants = $derived(
		allParticipants
			.filter(
				(p) =>
					p.user_id !== user?.id && videoCallService.inVideoParticipantIds.has(p.user_id)
			)
			// Show the Jitsi name the user chose in the call, falling back to their username.
			.map((p) => ({
				...p,
				username: videoCallService.jitsiDisplayNameFor(p.user_id) ?? p.username
			}))
	);
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

	let hasJoinedCall = $state(false);
	let jitsiApi: any = $state(null);
	let roomContext = $state<RoomContext>('plenary');
	let activePanel = $state<PanelTab>('agenda');
	let jitsiModeratorStatus = $state<boolean>(false);
	let currentJitsiRoomName = $state<string>('');
	/** This client's own Jitsi participant id, used to detect our own rename events. */
	let localParticipantId = $state<string | null>(null);

	/** Mock breakout rooms for dev testing */
	let mockBreakoutRooms = $state<BreakoutRoomDisplay[]>([]);

	let showCreateBreakout = $state(false);
	let breakoutDialogItem = $state<AgendaItem | null>(null);
	let showBroadcast = $state(false);
	let showEndMeeting = $state(false);
	let mobileRoomIndex = $state(0);
	let mobileAgendaViewIndex = $state(0);
	/** Whether the mobile bottom sheet is collapsed (peek) or expanded */
	let mobileSheetCollapsed = $state(false);
	let seenAssistanceRequests = $state<Set<string>>(new Set());

	/** Notice queue for assistance requests, broadcasts, time warnings */
	let noticeQueue = $state<{ message: string; actionLabel?: string; onAction?: () => void }[]>(
		[]
	);
	let showBreakoutEnding = $state(false);
	let breakoutEndingDismissed = $state(false);
	let breakoutAutoEnded = $state(false);

	/** Tracks assigned room index to detect mid-session reassignment by moderator */
	let trackedRoomIndex = $state<number | null>(null);

	/** Jitsi's native breakout room data from breakoutRoomsUpdated event */
	let jitsiBreakoutRooms = $state<Record<string, any>>({});

	/** Flag to track if Jitsi breakout rooms have been created and are ready */
	let breakoutRoomsReady = $state(false);

	/** Lightweight toast for admin confirmations */
	let toastMessage = $state<string | null>(null);
	let toastTimeout: ReturnType<typeof setTimeout> | null = null;

	let breakoutTimeRemaining = $state<number | null>(null);
	let countdownInterval: ReturnType<typeof setInterval> | null = null;

	// Prefer the agenda pushed over WS (live edits) over the SSR-loaded one, which can go stale.
	let agendaItems = $derived(mapApiAgenda(videoCallService.agenda ?? event?.agenda ?? []));

	let meetingPhase = $derived.by(() => {
		let phase: 'loading' | 'ended' | 'incall' | 'lobby';
		if (callStatus === null) {
			phase = 'loading';
		} else if (hasJoinedCall && callStatus === 'Ended') {
			phase = 'ended';
		} else if (hasJoinedCall) {
			phase = 'incall';
		} else {
			phase = 'lobby';
		}
		return phase;
	});

	/** True if any moderator/facilitator (other than current user) is in the lobby. */
	let hostPresent = $derived(
		allParticipants.some(
			(p) => p.user_id !== user?.id && (p.role === 'moderator' || p.role === 'facilitator')
		)
	);

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

	let plenaryRoomName = $derived(event?.videoMeetingId ?? '');

	let roomChipText = $derived.by(() => {
		if (typeof roomContext === 'string') return 'Plenary room';
		return roomContext.roomName;
	});

	let timeLeftFormatted = $derived(formatCountdown(breakoutTimeRemaining));

	let breakoutRoomDisplays = $derived.by((): BreakoutRoomDisplay[] => {
		// Use real rooms from backend if available, otherwise use mock rooms
		if (breakoutRooms.length > 0) {
			return breakoutRooms.map((_, index) => ({
				index,
				name: `Room #${index + 1}`,
				participants: videoCallService
					.getBreakoutRoomParticipants(index)
					.filter((p) => p.user_id !== user?.id),
				hasAssistanceRequest: videoCallService.hasAssistanceRequest(`room-${index}`),
				assistanceRequestUser: videoCallService.getAssistanceRequestUser(`room-${index}`)
			}));
		}
		return mockBreakoutRooms;
	});

	let preassignedBreakoutRooms = $derived(
		groupParticipantsByRoom(allParticipants, videoCallService.breakoutRooms)
	);

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

	// Watch for Jitsi breakout rooms to become ready
	$effect(() => {
		if (isBreakoutActive) {
			const nonMainRoomCount = Object.values(jitsiBreakoutRooms).filter(
				(r: any) => !r.isMainRoom
			).length;
			// Set ready when Jitsi has created the expected number of rooms
			if (
				nonMainRoomCount > 0 &&
				nonMainRoomCount >= (breakoutRooms.length || mockBreakoutRooms.length)
			) {
				if (!breakoutRoomsReady) {
					breakoutRoomsReady = true;
				}
			}
		} else {
			// Reset when breakout session ends
			breakoutRoomsReady = false;
		}
	});

	// Watch for new assistance requests (host only)
	$effect(() => {
		if (!isModerator) return;
		const reqs = assistanceRequests;
		for (const rn of Object.keys(reqs)) {
			const userId = reqs[rn].made_by_user;
			const key = `${rn}:${userId}`;
			if (seenAssistanceRequests.has(key)) continue;
			const match = rn.match(/room-(\d+)/);
			const ri = match ? parseInt(match[1]) : 0;
			const roomName = `Breakout room #${ri + 1}`;
			const participant = allParticipants.find((p) => p.user_id === userId);
			const displayName = participant?.username ?? userId.slice(0, 8);
			pushNotice({
				message: `${displayName} from ${roomName} requested help.`,
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

	// Auto-join when call is in progress (moderator started it)
	$effect(() => {
		if (callStatus === 'InProgress' && !hasJoinedCall) {
			hasJoinedCall = true;
		}
	});

	$effect(() => {
		if (isBreakoutActive && isModerator) {
			activePanel = 'breakoutRooms';
		}
		if (!isBreakoutActive && activePanel === 'breakoutRooms') {
			activePanel = 'agenda';
		}
	});

	// Sync mobile agenda view with current step
	$effect(() => {
		mobileAgendaViewIndex = currentStep;
	});

	// Reset mobile sheet collapse state when a participant enters a breakout room (peek)
	$effect(() => {
		if (inBreakoutRoom && !isModerator) {
			mobileSheetCollapsed = true;
		}
	});

	// Auto-enter participants into their assigned breakout room
	$effect(() => {
		if (
			isBreakoutActive &&
			!isModerator &&
			breakoutRoomsReady &&
			typeof roomContext === 'string'
		) {
			const assignedRoom = user ? videoCallService.getUserBreakoutRoom(user.id) : null;
			// Use assigned room from backend, or default to room 0 for mock testing
			handleEnterBreakoutRoom(assignedRoom ?? 0);
		}
		if (!isBreakoutActive && typeof roomContext !== 'string' && !isModerator) {
			roomContext = 'plenary';
			// Jitsi automatically moves participants back when rooms are removed
		}
	});

	// Detect mid-session room reassignment (moderator moved a participant)
	$effect(() => {
		if (!isBreakoutActive || isModerator || !user || !breakoutRoomsReady) {
			trackedRoomIndex = null;
			return;
		}

		const assignedRoom = videoCallService.getUserBreakoutRoom(user.id);
		const tracked = untrack(() => trackedRoomIndex);

		if (tracked === null && assignedRoom !== null) {
			// First assignment — just start tracking
			trackedRoomIndex = assignedRoom;
		} else if (assignedRoom !== null && assignedRoom !== tracked && tracked !== null) {
			// Room changed mid-session — moderator moved us
			handleEnterBreakoutRoom(assignedRoom);
			pushNotice({ message: `You've been moved to Breakout room #${assignedRoom + 1}` });
			trackedRoomIndex = assignedRoom;
		}
	});

	// Breakout ending: show countdown dialog for participants, silently auto-end for facilitator
	let breakoutSecondsLeft = $derived(
		breakoutTimeRemaining !== null ? Math.ceil(breakoutTimeRemaining / 1000) : 0
	);

	$effect(() => {
		// Show countdown dialog to participants at 5 seconds
		if (
			breakoutTimeRemaining !== null &&
			breakoutTimeRemaining <= 5000 &&
			breakoutTimeRemaining > 0 &&
			isBreakoutActive &&
			!breakoutEndingDismissed
		) {
			showBreakoutEnding = true;
		}
		// Auto-end at 0
		if (
			breakoutTimeRemaining !== null &&
			breakoutTimeRemaining <= 0 &&
			isBreakoutActive &&
			!breakoutAutoEnded
		) {
			breakoutAutoEnded = true;
			handleGoBackToPlenary();
		}
		// Dismiss dialog once back in plenary / breakout ended
		if (!isBreakoutActive) {
			showBreakoutEnding = false;
			breakoutEndingDismissed = false;
			breakoutAutoEnded = false;
		}
	});

	// When callStatus becomes 'Ended', hang up Jitsi for ALL participants
	$effect(() => {
		if (callStatus === 'Ended' && jitsiApi) {
			jitsiApi.executeCommand('hangup');
			hasJoinedCall = false;
		}
	});

	// Watch for broadcast messages (participants only — moderator gets toast confirmation)
	$effect(() => {
		if (lastBroadcast) {
			if (!isModerator) {
				showToast(lastBroadcast);
			}
			videoCallService.clearLastMessage();
		}
	});

	onDestroy(() => {
		if (countdownInterval) clearInterval(countdownInterval);
		if (toastTimeout) clearTimeout(toastTimeout);
	});

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

	/** clears all call and UI state */
	function resetCall() {
		videoCallService.changeCallState(eventId, 'Waiting');
		videoCallService.setAgendaItem(eventId, 0);
		videoCallService.endBreakoutSession(eventId);
		hasJoinedCall = false;
		roomContext = 'plenary';
		showCreateBreakout = false;
		breakoutDialogItem = null;
		showBroadcast = false;
		showEndMeeting = false;
		activePanel = 'agenda';
		noticeQueue = [];
		showBreakoutEnding = false;
		breakoutEndingDismissed = false;
		breakoutAutoEnded = false;
		trackedRoomIndex = null;
		jitsiBreakoutRooms = {};
		breakoutRoomsReady = false;
		mockBreakoutRooms = [];
		mobileRoomIndex = 0;
		mobileAgendaViewIndex = 0;
		mobileSheetCollapsed = false;
		seenAssistanceRequests = new Set();
	}

	/** DEV ONLY: reset call state to Waiting */
	function devResetCall() {
		if (dev) {
			resetCall();
			console.log('DEV: Call state reset to Waiting');
		}
	}

	function handleStartMeeting() {
		// Only render Jitsi — don't broadcast InProgress until moderator is actually in the call
		hasJoinedCall = true;
	}

	function handleSetAgendaItem(index: number) {
		videoCallService.setAgendaItem(eventId, index);
		if (isModerator && agendaItems[index]?.type === 'breakout' && !isBreakoutActive) {
			if (inCallParticipants.length === 0) {
				showToast('Cannot create breakout rooms. No participants in the call');
				return;
			}
			breakoutDialogItem = agendaItems[index];
			showCreateBreakout = true;
		}
	}

	function handleNextAgendaItem() {
		const next = currentStep + 1;
		if (next < agendaItems.length) {
			handleSetAgendaItem(next);
		}
	}

	/** Moderator ends the meeting for everyone */
	async function handleEndMeeting() {
		showEndMeeting = false;

		// Close breakout rooms first if a session is active
		if (isBreakoutActive) {
			await handleEndBreakoutSession();
		}

		// Broadcast 'Ended' state to all participants via WS
		videoCallService.changeCallState(eventId, 'Ended');
	}

	function handleCreateBreakout(config: {
		maxPerRoom: number;
		durationMinutes: number;
		roomAssignments: VideoCallParticipant[][];
	}) {
		// Send explicit room assignments so backend uses the dialog's distribution
		const roomAssignments = config.roomAssignments.map((room) => room.map((p) => p.user_id));
		videoCallService.assignBreakoutRooms(eventId, config.maxPerRoom, roomAssignments);
		const ends = new Date(Date.now() + config.durationMinutes * 60 * 1000).toISOString();
		videoCallService.startBreakoutSession(eventId, ends);
		showCreateBreakout = false;

		// Create breakout rooms in Jitsi via native API
		for (let i = 0; i < config.roomAssignments.length; i++) {
			jitsiApi?.executeCommand('addBreakoutRoom', `Breakout room #${i + 1}`);
		}

		// Use dialog's room assignments for local display until backend confirms
		mockBreakoutRooms = config.roomAssignments.map((participants, i) => ({
			index: i,
			name: `Room #${i + 1}`,
			participants,
			hasAssistanceRequest: false,
			assistanceRequestUser: null
		}));

		// Auto-switch to breakout rooms tab
		activePanel = 'breakoutRooms';
	}

	function handleEnterBreakoutRoom(roomIndex: number) {
		if (!breakoutRoomsReady) {
			console.warn(
				'[BREAKOUT] Attempted to enter room before Jitsi breakout rooms are ready'
			);
		}

		roomContext = {
			type: 'breakout',
			roomIndex,
			roomName: `Breakout room #${roomIndex + 1}`
		};

		// Use Jitsi's native breakout room API to switch rooms
		const jitsiRoomId = getJitsiBreakoutRoomId(jitsiBreakoutRooms, roomIndex);
		if (jitsiRoomId) {
			jitsiApi?.executeCommand('joinBreakoutRoom', jitsiRoomId);
		} else {
			console.warn(
				'[BREAKOUT] No Jitsi room ID found for index:',
				roomIndex,
				'— available rooms:',
				Object.values(jitsiBreakoutRooms).map((r: any) => ({
					id: r.id,
					name: r.name,
					isMain: r.isMainRoom
				}))
			);
		}

		if (isModerator) {
			videoCallService.resolveBreakoutRoomAssistanceRequest(eventId, `room-${roomIndex}`);
		}
	}

	function handleLeaveBreakoutRoom() {
		roomContext = 'plenary';

		// Use Jitsi API to return to main room (no iframe reload needed)
		try {
			jitsiApi?.executeCommand('joinBreakoutRoom');
		} catch (error) {
			console.error('[BREAKOUT] Error returning to main room:', error);
		}
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

	function handleUpdateTime(minutes: number) {
		const currentEnd = videoCallService.getBreakoutSessionEndTime();
		if (currentEnd) {
			const newEnd = new Date(currentEnd.getTime() + minutes * 60 * 1000).toISOString();
			videoCallService.extendBreakoutSession(eventId, newEnd);
			const abs = Math.abs(minutes);
			const verb = minutes >= 0 ? 'added' : 'removed';
			const preposition = minutes >= 0 ? 'to' : 'from';
			const msg = `${abs} minute(s) ${verb} ${preposition} breakout session`;
			videoCallService.broadcastMessage(eventId, msg);
			showToast(`${abs} minute(s) ${verb}`);
		}
	}

	function handleEndBreakoutSessionCountdown() {
		const currentEnd = videoCallService.getBreakoutSessionEndTime();
		if (!currentEnd || !isBreakoutActive) {
			showToast('No active breakout session to end');
			return;
		}
		const newEnd = new Date(Date.now() + 60 * 1000).toISOString();
		const message = 'Breakout rooms will finish in 1 min';
		videoCallService.extendBreakoutSession(eventId, newEnd);
		videoCallService.broadcastMessage(eventId, message);
		showToast(message);
	}

	async function handleEndBreakoutSession() {
		// Step 1: Ensure moderator is in main room before removing breakout rooms
		if (typeof roomContext !== 'string') {
			try {
				jitsiApi?.executeCommand('joinBreakoutRoom');
				// Wait a moment for the transition
				await new Promise((resolve) => setTimeout(resolve, 500));
			} catch (error) {
				console.error('[BREAKOUT] Error returning to main room:', error);
			}
		}

		// Step 2: Get fresh breakout rooms list from Jitsi API
		let breakoutRoomsList: any[] = [];
		try {
			const rooms = await jitsiApi?.listBreakoutRooms?.();

			if (rooms) {
				breakoutRoomsList = Object.values(rooms).filter((r: any) => !r.isMainRoom);
			}
		} catch (e) {
			console.error('[BREAKOUT] Error getting breakout rooms:', e);
			// Fallback to state if API call fails
			breakoutRoomsList = Object.values(jitsiBreakoutRooms).filter((r: any) => !r.isMainRoom);
		}

		// Step 3: Remove all breakout rooms - Jitsi should auto-return participants to main room
		for (const room of breakoutRoomsList) {
			try {
				// Use room.jid for the removeBreakoutRoom command
				jitsiApi?.executeCommand('closeBreakoutRoom', room.id);
			} catch (error) {
				console.error('[BREAKOUT] Error removing breakout room:', error);
			}
		}

		// Step 4: Wait a moment for rooms to be removed
		await new Promise((resolve) => setTimeout(resolve, 500));

		// Step 5: End the session on backend (broadcasts to all participants)
		videoCallService.endBreakoutSession(eventId);

		// Step 6: Update local state
		roomContext = 'plenary';
		activePanel = 'agenda';

		// Clean up local state
		mockBreakoutRooms = [];
		jitsiBreakoutRooms = {};
		breakoutRoomsReady = false;
	}

	function handleGoBackToPlenary() {
		roomContext = 'plenary';
		if (isModerator) {
			handleEndBreakoutSession();
		} else {
			// Participants return to main room by calling joinBreakoutRoom without arguments
			try {
				// Calling joinBreakoutRoom without arguments moves user to main room
				jitsiApi?.executeCommand('joinBreakoutRoom');
			} catch (error) {
				console.error('[BREAKOUT] Error returning to main room:', error);
			}
		}
	}

	function handleApiReady(api: any) {
		jitsiApi = api;
	}

	function handleModeratorStatusChanged(isMod: boolean) {
		jitsiModeratorStatus = isMod;
	}

	function handleVideoConferenceJoined(data: any) {
		currentJitsiRoomName = data.roomName;
		localParticipantId = data.id ?? null;
		// Tell the backend we're really in the video (rename-proof call presence) and
		// report the Jitsi name we chose so the UI can show it instead of the username.
		videoCallService.reportVideoJoined(eventId, data.displayName ?? data.displayname);
		// Moderator is now in Jitsi — safe to let participants in
		if (isModerator && callStatus === 'Waiting') {
			videoCallService.changeCallState(eventId, 'InProgress');
		}
	}

	function handleVideoConferenceLeft(data: any) {
		videoCallService.reportVideoLeft(eventId);
	}

	function handleDisplayNameChange(data: any) {
		// Only re-report when OUR own display name changed.
		if (data?.id && data.id === localParticipantId) {
			videoCallService.reportVideoJoined(eventId, data.displayname ?? data.displayName);
		}
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
{:else if meetingPhase === 'ended'}
	<MeetingEndedScreen
		title={event?.name ?? 'Meeting'}
		conversationUrl={`/conversations/${conversationId}`}
		{isModerator}
		onResetCall={resetCall}
	/>
{:else if meetingPhase === 'lobby'}
	<MeetingLobby
		title={event?.name ?? 'Meeting'}
		scheduledTime={scheduledTimeText}
		endedTime={event ? `${formatDateShort(event.endTime)} ${formatTime(event.endTime)}` : ''}
		endTimeIso={event?.endTime}
		participants={otherParticipants}
		{callStatus}
		{isModerator}
		{hostPresent}
		onStartMeeting={handleStartMeeting}
		onResetCall={resetCall}
	/>
{:else}
	<!-- In-call: full-width black background, stays in document flow -->
	<div class="bg-sidebar relative flex h-dvh w-full flex-col overflow-hidden">
		<div class="flex min-h-0 flex-1">
			<!-- Jitsi area -->
			<div class="relative flex min-h-0 min-w-0 flex-1 flex-col">
				<!-- Header bar -->
				<div
					class="border-sidebar-foreground/20 flex flex-col gap-2 border-b px-4 pt-4 pb-3 md:flex-row md:items-center md:justify-between md:px-6 md:pb-3"
				>
					<!-- Top row: event name + recording + chip (desktop inline) -->
					<div
						class="flex items-center justify-between gap-3 md:flex-wrap md:justify-start md:gap-4"
					>
						<span class="text-sidebar-foreground text-lg font-medium md:text-xl">
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
						<!-- Desktop-only: dev info -->
						{#if currentJitsiRoomName && dev}
							<div class="hidden items-center gap-2 md:flex">
								<span class="text-sidebar-foreground/60 text-xs">
									Room: {currentJitsiRoomName.slice(0, 8)}...
								</span>
							</div>
						{/if}
						<!-- Desktop chip inline -->
						{#if currentJitsiRoomName}
							<div class="hidden md:block">
								{@render roomChip()}
							</div>
						{/if}
						{#if dev && isModerator}
							<button
								class="bg-destructive text-destructive-foreground hover:bg-destructive/90 rounded px-3 py-1 text-xs font-medium md:hidden"
								onclick={devResetCall}
							>
								Reset
							</button>
						{/if}
					</div>

					<!-- Mobile full-width chip below title row -->
					{#if currentJitsiRoomName}
						<div class="md:hidden">
							{@render roomChip()}
						</div>
					{/if}

					{#if dev && isModerator}
						<button
							class="bg-destructive text-destructive-foreground hover:bg-destructive/90 hidden shrink-0 rounded px-3 py-1 text-xs font-medium md:block"
							onclick={devResetCall}
						>
							DEV: Reset Call
						</button>
					{/if}
				</div>

				<!-- Jitsi iframe -->
				<div class="relative flex-1 overflow-hidden">
					<JitsiMeet
						roomName={plenaryRoomName}
						{jwt}
						onApiReady={handleApiReady}
						onBreakoutRoomsUpdated={(rooms) => {
							jitsiBreakoutRooms = rooms;
						}}
						onModeratorStatusChanged={handleModeratorStatusChanged}
						onVideoConferenceJoined={handleVideoConferenceJoined}
						onVideoConferenceLeft={handleVideoConferenceLeft}
						onDisplayNameChange={handleDisplayNameChange}
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
							prejoinPageEnabled: false,
							disableDeepLinking: true,
							hideConferenceSubject: true
						}}
					/>
				</div>
			</div>

			<!-- Right panel (side-by-side, hidden on mobile) -->
			<div class="relative hidden h-full w-[360px] shrink-0 p-3 md:block">
				<SidePanel
					activeTab={activePanel}
					showTabs={isBreakoutActive && isModerator && !inBreakoutRoom}
					onTabChange={(tab) => (activePanel = tab)}
				>
					{#if isBreakoutActive && inBreakoutRoom}
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
							onUpdateTime={handleUpdateTime}
							onEndSession={handleEndBreakoutSessionCountdown}
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
							onEndMeeting={() => (showEndMeeting = true)}
						/>
					{/if}
				</SidePanel>
			</div>
		</div>

		<!-- Mobile drawers: peek is in-flow (pushes iframe), expansion overlays -->
		{#if isBreakoutActive && inBreakoutRoom}
			<!-- Inline spacer reserves peek height so iframe is pushed up -->
			<div class="h-16 shrink-0 md:hidden" aria-hidden="true"></div>
			<!-- In breakout room (participant or moderator): peek/expand drawer -->
			<div
				class="bg-muted absolute inset-x-0 bottom-0 z-30 flex max-h-[75dvh] flex-col overflow-hidden rounded-t-3xl rounded-b-none shadow-[0_-4px_20px_rgba(0,0,0,0.25)] md:hidden"
			>
				<button
					type="button"
					class="flex h-16 w-full shrink-0 cursor-pointer flex-col items-center justify-center gap-2"
					onclick={() => (mobileSheetCollapsed = !mobileSheetCollapsed)}
					aria-expanded={!mobileSheetCollapsed}
					aria-label={mobileSheetCollapsed
						? 'Expand breakout session'
						: 'Collapse breakout session'}
				>
					<div class="bg-foreground/20 h-1 w-10 rounded-full"></div>
					<span class="text-foreground text-sm font-medium">Breakout Session</span>
				</button>

				{#if !mobileSheetCollapsed}
					<div
						class="flex min-h-0 flex-col gap-5 overflow-y-auto px-4 pb-5"
						transition:slide={{ duration: 280, easing: cubicOut }}
					>
						{#if currentAgendaItem?.breakoutQuestion}
							<div
								class="bg-background flex shrink-0 flex-col items-center gap-4 rounded-2xl px-5 py-5"
							>
								<div class="flex flex-col items-center gap-2">
									<span class="text-primary text-xs font-medium uppercase">
										Question
									</span>
									<p
										class="text-foreground text-center text-lg leading-tight font-semibold break-words"
									>
										{currentAgendaItem.breakoutQuestion}
									</p>
								</div>
								{#if currentAgendaItem.breakoutDescription}
									<div class="bg-border h-px w-full"></div>
									<div class="text-muted-foreground w-full text-base">
										<ContentRenderer
											content={currentAgendaItem.breakoutDescription}
										/>
									</div>
								{/if}
							</div>
						{/if}

						{#if isModerator}
							<Button
								variant="destructive"
								class="h-11 w-full text-sm font-medium"
								onclick={handleLeaveBreakoutRoom}
							>
								Leave {roomChipText}
							</Button>
						{:else}
							<Button
								variant="primaryDark"
								class="h-11 w-full text-sm font-medium"
								onclick={handleCallForSupport}
							>
								<Hand class="mr-1.5 h-4 w-4" />
								Call for support
							</Button>
						{/if}
					</div>
				{/if}
			</div>
		{:else if isModerator}
			<!-- Inline spacer reserves peek height so iframe is pushed up -->
			<div class="h-16 shrink-0 md:hidden" aria-hidden="true"></div>
			<!-- Facilitator (not in breakout): collapsible sheet with tabs (when breakout active) -->
			<div
				class="bg-muted absolute inset-x-0 bottom-0 z-30 flex max-h-[75dvh] flex-col overflow-hidden rounded-t-3xl rounded-b-none shadow-[0_-4px_20px_rgba(0,0,0,0.25)] md:hidden"
			>
				<button
					type="button"
					class="flex h-16 w-full shrink-0 cursor-pointer flex-col items-center justify-center gap-2"
					onclick={() => (mobileSheetCollapsed = !mobileSheetCollapsed)}
					aria-expanded={!mobileSheetCollapsed}
					aria-label={mobileSheetCollapsed ? 'Expand controls' : 'Collapse controls'}
				>
					<div class="bg-foreground/20 h-1 w-10 rounded-full"></div>
					<span class="text-foreground text-sm font-medium">
						{isBreakoutActive ? 'Breakout Session' : 'Agenda'}
					</span>
				</button>

				{#if !mobileSheetCollapsed}
					<div
						class="flex min-h-0 flex-col overflow-hidden"
						transition:slide={{ duration: 280, easing: cubicOut }}
					>
						<!-- Tabs only shown while breakout session is active -->
						{#if isBreakoutActive}
							<div class="shrink-0 px-4 pt-1 pb-3">
								<div class="flex items-center gap-1 rounded-2xl">
									<button
										type="button"
										class="h-9 flex-1 rounded-xl px-3 text-sm transition-all {activePanel ===
										'agenda'
											? 'bg-background text-foreground font-semibold shadow-sm'
											: 'text-muted-foreground font-semibold'}"
										onclick={() => (activePanel = 'agenda')}
									>
										Agenda
									</button>
									<button
										type="button"
										class="h-9 flex-1 rounded-xl px-3 text-sm transition-all {activePanel ===
										'breakoutRooms'
											? 'bg-background text-foreground font-semibold shadow-sm'
											: 'text-muted-foreground font-semibold'}"
										onclick={() => (activePanel = 'breakoutRooms')}
									>
										Breakout Session
									</button>
								</div>
							</div>
						{/if}

						{#if isBreakoutActive && activePanel === 'breakoutRooms'}
							{@render facilitatorBreakoutPanel()}
						{:else}
							{@render facilitatorAgendaPanel()}
						{/if}
					</div>
				{/if}
			</div>
		{:else}
			<!-- Participant (not in breakout): chip navigation row, no drawer -->
			<div
				class="border-sidebar-foreground/20 bg-sidebar flex shrink-0 items-center gap-2 border-t px-3 py-4 md:hidden"
			>
				<button
					type="button"
					class="text-sidebar-foreground shrink-0 p-1 disabled:opacity-30"
					disabled={mobileAgendaViewIndex <= 0}
					onclick={() => mobileAgendaViewIndex--}
					aria-label="Previous agenda item"
				>
					<ChevronLeft class="h-5 w-5" />
				</button>
				{@render agendaChip(mobileAgendaViewIndex)}
				<button
					type="button"
					class="text-sidebar-foreground shrink-0 p-1 disabled:opacity-30"
					disabled={mobileAgendaViewIndex >= agendaItems.length - 1}
					onclick={() => mobileAgendaViewIndex++}
					aria-label="Next agenda item"
				>
					<ChevronRight class="h-5 w-5" />
				</button>
			</div>
		{/if}
	</div>
{/if}

<!-- Dialogs -->
<CreateBreakoutDialog
	bind:open={showCreateBreakout}
	participants={allParticipants}
	initialAssignments={preassignedBreakoutRooms}
	defaultDuration={breakoutDialogItem?.durationMinutes}
	defaultMaxPerRoom={breakoutDialogItem?.maxPerRoom}
	onClose={() => (showCreateBreakout = false)}
	onCreate={handleCreateBreakout}
/>

<BroadcastMessageDialog
	bind:open={showBroadcast}
	onClose={() => (showBroadcast = false)}
	onSend={handleBroadcast}
/>

<EndMeetingDialog
	bind:open={showEndMeeting}
	onConfirm={handleEndMeeting}
	onCancel={() => (showEndMeeting = false)}
/>

<BreakoutEndingDialog
	bind:open={showBreakoutEnding}
	secondsLeft={breakoutSecondsLeft}
	onGoBack={() => {
		showBreakoutEnding = false;
		breakoutEndingDismissed = true;
		handleGoBackToPlenary();
	}}
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

{#snippet roomChip()}
	{#if isBreakoutActive && inBreakoutRoom}
		<div
			class="bg-sidebar-foreground/20 flex items-center justify-center gap-2 rounded-full px-4 py-2 md:px-3 md:py-1"
		>
			<span class="text-sidebar-foreground text-sm font-medium md:text-xs">
				{roomChipText}
			</span>
			<span class="text-primary text-sm font-medium md:text-xs">
				<span class="md:hidden">Time left </span>
				<span class="hidden md:inline">·</span>
				{timeLeftFormatted}
			</span>
		</div>
	{:else}
		<div
			class="bg-sidebar-foreground/20 flex items-center justify-center rounded-full px-4 py-2 md:px-3 md:py-1"
		>
			<span class="text-sidebar-foreground text-sm font-medium md:text-xs">
				{roomChipText}
			</span>
		</div>
	{/if}
{/snippet}

{#snippet agendaChip(index: number)}
	{@const item = agendaItems[index]}
	{@const status = index < currentStep ? 'done' : index === currentStep ? 'current' : 'upcoming'}
	<div
		class="flex min-w-0 flex-1 items-center justify-center gap-2 rounded-full px-4 py-2.5 {status ===
		'current'
			? 'bg-primary/30 text-foreground'
			: status === 'done'
				? 'bg-muted-foreground/10 text-muted-foreground'
				: 'bg-background text-card-foreground'}"
	>
		{#if status === 'done'}
			<div
				class="bg-muted-foreground/20 flex h-5 w-5 shrink-0 items-center justify-center rounded-full"
			>
				<Check class="text-muted-foreground h-3 w-3" />
			</div>
		{:else if status === 'current'}
			<span
				class="text-primary bg-background flex h-5 shrink-0 items-center rounded-full border px-2 text-[10px] font-semibold"
			>
				Current
			</span>
		{:else}
			<div
				class="bg-primary/10 flex h-5 w-5 shrink-0 items-center justify-center rounded-full"
			>
				<span class="text-primary text-[10px] font-semibold">{index + 1}</span>
			</div>
		{/if}
		<span class="truncate text-sm font-medium">
			{item?.title ?? 'Agenda'}
		</span>
	</div>
{/snippet}

{#snippet facilitatorAgendaPanel()}
	{@const hasNext = currentStep < agendaItems.length - 1}
	{@const isLastItem = currentStep === agendaItems.length - 1 && agendaItems.length > 0}
	<div class="flex flex-col gap-3 px-4 pb-5">
		<div class="flex items-center gap-2">
			<button
				type="button"
				class="text-foreground shrink-0 p-1 disabled:opacity-30"
				disabled={mobileAgendaViewIndex <= 0}
				onclick={() => mobileAgendaViewIndex--}
				aria-label="Previous agenda item"
			>
				<ChevronLeft class="h-5 w-5" />
			</button>
			<button
				type="button"
				class="flex min-w-0 flex-1 cursor-pointer"
				onclick={() => !isBreakoutActive && handleSetAgendaItem(mobileAgendaViewIndex)}
				disabled={isBreakoutActive || mobileAgendaViewIndex === currentStep}
			>
				{@render agendaChip(mobileAgendaViewIndex)}
			</button>
			<button
				type="button"
				class="text-foreground shrink-0 p-1 disabled:opacity-30"
				disabled={mobileAgendaViewIndex >= agendaItems.length - 1}
				onclick={() => mobileAgendaViewIndex++}
				aria-label="Next agenda item"
			>
				<ChevronRight class="h-5 w-5" />
			</button>
		</div>

		{#if isBreakoutActive}
			<p class="text-muted-foreground px-2 text-center text-xs leading-relaxed">
				End the breakout session to continue with the agenda.
			</p>
		{:else if isLastItem}
			<Button
				variant="destructive"
				class="h-11 w-full text-sm font-medium"
				onclick={() => (showEndMeeting = true)}
			>
				End meeting
			</Button>
		{:else}
			<Button
				variant="primaryDark"
				class="h-11 w-full text-sm font-medium"
				onclick={handleNextAgendaItem}
				disabled={!hasNext}
			>
				Start next step
				<ArrowRight class="ml-1.5 h-4 w-4" />
			</Button>
		{/if}
	</div>
{/snippet}

{#snippet facilitatorBreakoutPanel()}
	<div class="flex min-h-0 flex-col gap-4 overflow-y-auto px-4 pb-5">
		<!-- Timer + chips -->
		<div class="flex items-center justify-center gap-2.5">
			<span class="text-ring text-sm font-semibold">
				Time left&nbsp;&nbsp;{timeLeftFormatted}
			</span>
			<div class="flex items-center gap-1.5">
				<button
					type="button"
					class="bg-primary/20 text-ring h-6 cursor-pointer rounded-full px-2 text-xs font-medium shadow-sm"
					onclick={() => handleUpdateTime(-1)}
				>
					-1min
				</button>
				<button
					type="button"
					class="bg-primary/20 text-ring h-6 cursor-pointer rounded-full px-2 text-xs font-medium shadow-sm"
					onclick={() => handleUpdateTime(1)}
				>
					+1min
				</button>
				<button
					type="button"
					class="bg-primary/20 text-ring h-6 cursor-pointer rounded-full px-2 text-xs font-medium shadow-sm"
					onclick={() => handleUpdateTime(2)}
				>
					+2min
				</button>
			</div>
		</div>

		<!-- Room carousel -->
		<div class="flex items-center gap-2">
			<button
				type="button"
				class="text-foreground shrink-0 p-1 disabled:opacity-30"
				disabled={mobileRoomIndex <= 0}
				onclick={() => (mobileRoomIndex = Math.max(0, mobileRoomIndex - 1))}
				aria-label="Previous room"
			>
				<ChevronLeft class="h-5 w-5" />
			</button>

			{#if breakoutRoomDisplays[mobileRoomIndex]}
				{@const room = breakoutRoomDisplays[mobileRoomIndex]}
				<div
					class="bg-card border-border flex min-w-0 flex-1 flex-col overflow-hidden rounded-xl border shadow-sm"
				>
					<div class="flex flex-col gap-3 px-5 py-4">
						<span class="text-foreground truncate text-base font-semibold">
							{room.name}
						</span>
						<div class="flex flex-wrap items-start gap-3">
							{#each room.participants as p, i (p.user_id)}
								<div class="flex items-center gap-1.5">
									<div
										class="{[
											'bg-emerald-500',
											'bg-primary',
											'bg-indigo-500',
											'bg-orange-500',
											'bg-rose-500',
											'bg-cyan-500'
										][
											i % 6
										]} flex h-6 w-6 items-center justify-center rounded-full text-xs font-semibold text-white uppercase"
									>
										{(p.username ?? p.user_id).charAt(0).toUpperCase()}
									</div>
									<span class="text-foreground text-sm font-medium">
										{p.username ?? p.user_id.slice(0, 8)}
									</span>
								</div>
							{/each}
						</div>
					</div>
					<div class="border-border flex items-center border-t">
						<button
							type="button"
							class="text-foreground hover:bg-muted relative flex flex-1 items-center justify-center gap-2 px-3 py-3 text-xs font-medium"
							onclick={() => handleEnterBreakoutRoom(room.index)}
						>
							<MoveUpRight class="h-4 w-4" />
							Enter
							{#if room.hasAssistanceRequest}
								<span class="bg-destructive h-2 w-2 shrink-0 rounded-full"></span>
							{/if}
						</button>
					</div>
				</div>
			{/if}

			<button
				type="button"
				class="text-foreground shrink-0 p-1 disabled:opacity-30"
				disabled={mobileRoomIndex >= breakoutRoomDisplays.length - 1}
				onclick={() =>
					(mobileRoomIndex = Math.min(
						breakoutRoomDisplays.length - 1,
						mobileRoomIndex + 1
					))}
				aria-label="Next room"
			>
				<ChevronRight class="h-5 w-5" />
			</button>
		</div>

		<!-- Broadcast + End -->
		<div class="flex flex-col gap-2">
			<Button
				variant="primaryDark"
				class="h-11 w-full text-sm font-medium"
				onclick={() => (showBroadcast = true)}
			>
				<Megaphone class="mr-1.5 h-4 w-4" />
				Broadcast message
			</Button>
			<Button
				variant="outline"
				class="border-input text-destructive hover:bg-destructive/5 hover:text-destructive h-11 w-full text-sm font-medium"
				onclick={handleEndBreakoutSessionCountdown}
			>
				<CircleStop class="mr-1.5 h-4 w-4" />
				End breakout session
			</Button>
		</div>
	</div>
{/snippet}

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
