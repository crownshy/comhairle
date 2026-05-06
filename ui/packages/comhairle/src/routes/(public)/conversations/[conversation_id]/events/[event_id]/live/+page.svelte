<script lang="ts">
	import { onDestroy, untrack } from 'svelte';
	import { dev } from '$app/environment';
	import JitsiMeet from '$lib/components/JitsiMeet/JitsiMeet.svelte';
	import * as Drawer from '$lib/components/ui/drawer';
	import { formatDateShort, formatTime } from '$lib/utils';
	import { videoCallService } from '$lib/services/videoCallService.svelte';
	import MeetingLobby from '$lib/components/LiveEvent/MeetingLobby.svelte';
	import AgendaPanel from '$lib/components/LiveEvent/AgendaPanel.svelte';
	import BreakoutSessionPanel from '$lib/components/LiveEvent/BreakoutSessionPanel.svelte';
	import BreakoutRoomsPanel from '$lib/components/LiveEvent/BreakoutRoomsPanel.svelte';
	import BreakoutEndingDialog from '$lib/components/LiveEvent/BreakoutEndingDialog.svelte';
	import CreateBreakoutDialog from '$lib/components/LiveEvent/CreateBreakoutDialog.svelte';
	import BroadcastMessageDialog from '$lib/components/LiveEvent/BroadcastMessageDialog.svelte';
	import AddTimeDialog from '$lib/components/LiveEvent/AddTimeDialog.svelte';
	import NoticeDialog from '$lib/components/LiveEvent/NoticeDialog.svelte';
	import EndMeetingDialog from '$lib/components/LiveEvent/EndMeetingDialog.svelte';
	import MeetingEndedScreen from '$lib/components/LiveEvent/MeetingEndedScreen.svelte';
	import SidePanel from '$lib/components/LiveEvent/SidePanel.svelte';
	import type {
		AgendaItem,
		RoomContext,
		PanelTab,
		BreakoutRoomDisplay
	} from '$lib/components/LiveEvent/types';
	import type { VideoCallParticipant } from '$lib/services/videoCallService.svelte';
	import {
		ChevronUp,
		ChevronLeft,
		ChevronRight,
		MoveUpRight,
		Megaphone,
		CircleStop,
		Check
	} from 'lucide-svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import type { PageProps } from './$types';
	import type { EventAgendaItem } from '@crownshy/api-client/api';

	let { data }: PageProps = $props();

	let conversationId = $derived(data.conversationId);
	let eventId = $derived(data.eventId);
	let event = $derived(data.event);
	let jwt = $derived(data.jwt);
	let user = $derived(data.user);
	let isModerator = $derived(data.isModerator);

	let callStatus = $derived(videoCallService.callStatus);
	let allParticipants = $derived(videoCallService.participants);
	let otherParticipants = $derived(allParticipants.filter((p) => p.user_id !== user?.id));

	/** Participants actually in Jitsi call (excludes current user + lobby-only users) */
	let inCallParticipants = $derived(
		allParticipants.filter((p) => jitsiParticipantMap.has(p.user_id))
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

	// Map Jitsi participant IDs to backend user IDs
	let jitsiParticipantMap = $state<Map<string, string>>(new Map());

	/** Mock breakout rooms for dev testing */
	let mockBreakoutRooms = $state<BreakoutRoomDisplay[]>([]);

	let showCreateBreakout = $state(false);
	let breakoutDialogItem = $state<AgendaItem | null>(null);
	let showBroadcast = $state(false);
	let showAddTime = $state(false);
	let showEndMeeting = $state(false);
	let mobileRoomIndex = $state(0);
	let mobileAgendaViewIndex = $state(0);
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

	/** Get the Jitsi room ID for a given room index (0-based) */
	function getJitsiBreakoutRoomId(roomIndex: number): string | null {
		const nonMainRooms = Object.values(jitsiBreakoutRooms)
			.filter((r: any) => !r.isMainRoom)
			.sort((a: any, b: any) => (a.name ?? '').localeCompare(b.name ?? ''));
		return nonMainRooms[roomIndex]?.jid ?? null;
	}

	/** Lightweight toast for admin confirmations */
	let toastMessage = $state<string | null>(null);
	let toastTimeout: ReturnType<typeof setTimeout> | null = null;

	let breakoutTimeRemaining = $state<number | null>(null);
	let countdownInterval: ReturnType<typeof setInterval> | null = null;

	/** Map API agenda items to live event format */
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
					durationMinutes: item.BreakoutRoom.estimated_time,
					maxPerRoom: item.BreakoutRoom.max_per_room ?? undefined
				};
			}
		});
	}

	let agendaItems = $derived(mapApiAgenda(event?.agenda ?? []));

	let meetingPhase = $derived.by(() => {
		const phase =
			callStatus === null
				? ('loading' as const)
				: callStatus === 'Ended'
					? ('ended' as const)
					: hasJoinedCall
						? ('incall' as const)
						: ('lobby' as const);
		console.log(
			'[BREAKOUT] meetingPhase:',
			phase,
			'{ callStatus:',
			callStatus,
			', hasJoinedCall:',
			hasJoinedCall,
			'}'
		);
		return phase;
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

	let plenaryRoomName = $derived(event?.videoMeetingId ?? '');

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
				participants: videoCallService
					.getBreakoutRoomParticipants(index)
					.filter((p) => p.user_id !== user?.id),
				hasAssistanceRequest: videoCallService.hasAssistanceRequest(`room-${index}`),
				assistanceRequestUser: videoCallService.getAssistanceRequestUser(`room-${index}`)
			}));
		}
		return mockBreakoutRooms;
	});

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
			console.log('[BREAKOUT] Checking rooms ready:', {
				nonMainRoomCount,
				expectedRooms: breakoutRooms.length || mockBreakoutRooms.length,
				breakoutRoomsReady
			});
			// Set ready when Jitsi has created the expected number of rooms
			if (
				nonMainRoomCount > 0 &&
				nonMainRoomCount >= (breakoutRooms.length || mockBreakoutRooms.length)
			) {
				if (!breakoutRoomsReady) {
					console.log('[BREAKOUT] Breakout rooms are now ready!');
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
			console.log('[BREAKOUT] Auto-joining into call, isModerator:', isModerator);
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

	// Auto-enter participants into their assigned breakout room
	$effect(() => {
		console.log('[BREAKOUT] auto-enter effect:', {
			isBreakoutActive,
			isModerator,
			breakoutRoomsReady,
			roomContext: typeof roomContext === 'string' ? roomContext : roomContext,
			userId: user?.id
		});
		if (
			isBreakoutActive &&
			!isModerator &&
			breakoutRoomsReady &&
			typeof roomContext === 'string'
		) {
			const assignedRoom = user ? videoCallService.getUserBreakoutRoom(user.id) : null;
			console.log(
				'[BREAKOUT] Participant auto-entering breakout room:',
				assignedRoom,
				'(fallback 0)'
			);
			// Use assigned room from backend, or default to room 0 for mock testing
			handleEnterBreakoutRoom(assignedRoom ?? 0);
		}
		if (!isBreakoutActive && typeof roomContext !== 'string' && !isModerator) {
			console.log('[BREAKOUT] Breakout ended — returning participant to main room');
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
		console.log('[BREAKOUT] reassignment check:', { assignedRoom, tracked });

		if (tracked === null && assignedRoom !== null) {
			// First assignment — just start tracking
			console.log('[BREAKOUT] First assignment, tracking room:', assignedRoom);
			trackedRoomIndex = assignedRoom;
		} else if (assignedRoom !== null && assignedRoom !== tracked && tracked !== null) {
			// Room changed mid-session — moderator moved us
			console.log('[BREAKOUT] Room CHANGED by moderator:', tracked, '→', assignedRoom);
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
			!isModerator &&
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
				pushNotice({ message: lastBroadcast });
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

	/** DEV ONLY: reset call state to Waiting */
	function devResetCall() {
		if (dev) {
			videoCallService.changeCallState(eventId, 'Waiting');
			videoCallService.setAgendaItem(eventId, 0);
			videoCallService.endBreakoutSession(eventId);
			hasJoinedCall = false;
			roomContext = 'plenary';
			showCreateBreakout = false;
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
				showToast('Cannot create breakout rooms — no participants in the call');
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
		console.log('[BREAKOUT] handleCreateBreakout:', {
			maxPerRoom: config.maxPerRoom,
			durationMinutes: config.durationMinutes,
			roomCount: config.roomAssignments.length
		});
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
		console.log(
			'[BREAKOUT] handleEnterBreakoutRoom:',
			roomIndex,
			'| isModerator:',
			isModerator,
			'| current roomContext:',
			roomContext,
			'| breakoutRoomsReady:',
			breakoutRoomsReady
		);

		if (!breakoutRoomsReady) {
			console.warn(
				'[BREAKOUT] Attempted to enter room before Jitsi breakout rooms are ready'
			);
		}

		console.log('[BREAKOUT] Current jitsiBreakoutRooms state:', jitsiBreakoutRooms);
		console.log('[BREAKOUT] All rooms:', Object.values(jitsiBreakoutRooms));

		roomContext = {
			type: 'breakout',
			roomIndex,
			roomName: `Breakout room #${roomIndex + 1}`
		};
		console.log('[BREAKOUT] roomContext NOW:', roomContext);

		// Use Jitsi's native breakout room API to switch rooms
		const jitsiRoomId = getJitsiBreakoutRoomId(roomIndex);
		if (jitsiRoomId) {
			console.log('[BREAKOUT] joinBreakoutRoom via Jitsi API, roomId:', jitsiRoomId);
			console.log(
				'[BREAKOUT] roomId type:',
				typeof jitsiRoomId,
				'value:',
				JSON.stringify(jitsiRoomId)
			);
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
		console.log('[BREAKOUT] handleLeaveBreakoutRoom → returning to main room');
		roomContext = 'plenary';

		// Use Jitsi API to return to main room (no iframe reload needed)
		try {
			jitsiApi?.executeCommand('joinBreakoutRoom');
			console.log('[BREAKOUT] Sent joinBreakoutRoom command to return to main');
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

	async function handleEndBreakoutSession() {
		console.log('[BREAKOUT] handleEndBreakoutSession - removing breakout rooms');

		// Step 1: Ensure moderator is in main room before removing breakout rooms
		if (typeof roomContext !== 'string') {
			console.log('[BREAKOUT] Moderator in breakout room - returning to main first');
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
			console.log('[BREAKOUT] listBreakoutRooms() returned:', rooms);

			if (rooms) {
				breakoutRoomsList = Object.values(rooms).filter((r: any) => !r.isMainRoom);
			}
		} catch (e) {
			console.error('[BREAKOUT] Error getting breakout rooms:', e);
			// Fallback to state if API call fails
			breakoutRoomsList = Object.values(jitsiBreakoutRooms).filter((r: any) => !r.isMainRoom);
		}

		console.log('[BREAKOUT] Found', breakoutRoomsList.length, 'breakout rooms to remove');

		// Step 3: Remove all breakout rooms - Jitsi should auto-return participants to main room
		console.log('[BREAKOUT] Removing breakout rooms...');
		for (const room of breakoutRoomsList) {
			console.log('[BREAKOUT] Removing room:', {
				id: room.id,
				jid: room.jid,
				name: room.name
			});
			try {
				// Use room.jid for the removeBreakoutRoom command
				jitsiApi?.executeCommand('closeBreakoutRoom', room.id);
				console.log(
					'[BREAKOUT] Successfully sent removeBreakoutRoom command for:',
					room.jid
				);
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
		console.log('[BREAKOUT] handleGoBackToPlenary called, isModerator:', isModerator);
		roomContext = 'plenary';
		if (isModerator) {
			handleEndBreakoutSession();
		} else {
			// Participants return to main room by calling joinBreakoutRoom without arguments
			console.log('[BREAKOUT] Participant returning to main room:', plenaryRoomName);
			try {
				// Calling joinBreakoutRoom without arguments moves user to main room
				jitsiApi?.executeCommand('joinBreakoutRoom');
				console.log(
					'[BREAKOUT] Successfully sent joinBreakoutRoom command to return to main'
				);
			} catch (error) {
				console.error('[BREAKOUT] Error returning to main room:', error);
			}
		}
	}

	function handleApiReady(api: any) {
		jitsiApi = api;
	}

	function handleModeratorStatusChanged(isMod: boolean) {
		console.log('[BREAKOUT] handleModeratorStatusChanged called with:', isMod);
		console.log('[BREAKOUT] Previous jitsiModeratorStatus:', jitsiModeratorStatus);
		jitsiModeratorStatus = isMod;
		console.log('[BREAKOUT] New jitsiModeratorStatus:', jitsiModeratorStatus);
	}

	function handleVideoConferenceJoined(data: any) {
		console.log('[BREAKOUT] Entered Jitsi room:', data.roomName);
		currentJitsiRoomName = data.roomName;
		// Moderator is now in Jitsi — safe to let participants in
		if (isModerator && callStatus === 'Waiting') {
			videoCallService.changeCallState(eventId, 'InProgress');
		}
	}

	function handleVideoConferenceLeft(data: any) {
		console.log('[BREAKOUT] Left Jitsi room:', data.roomName);
	}

	function handleParticipantJoined(participant: any) {
		console.log('[BREAKOUT] Participant joined:', participant);

		// Try to match Jitsi participant to backend user by display name
		const matchingUser = allParticipants.find((p) => p.username === participant.displayName);

		if (matchingUser) {
			console.log(
				'[BREAKOUT] Mapped Jitsi participant',
				participant.id,
				'→ user',
				matchingUser.user_id
			);
			jitsiParticipantMap.set(matchingUser.user_id, participant.id);
		} else {
			console.warn('[BREAKOUT] Could not match participant:', participant.displayName);
		}
	}

	function handleParticipantLeft(participant: any) {
		console.log('[BREAKOUT] Participant left:', participant);

		// Remove from map
		const entry = Array.from(jitsiParticipantMap.entries()).find(
			([_, jitsiId]) => jitsiId === participant.id
		);
		if (entry) {
			jitsiParticipantMap.delete(entry[0]);
			console.log('[BREAKOUT] Removed participant from map:', entry[0]);
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
		onResetCall={() => {
			videoCallService.changeCallState(eventId, 'Waiting');
			videoCallService.setAgendaItem(eventId, 0);
			videoCallService.endBreakoutSession(eventId);
			hasJoinedCall = false;
			roomContext = 'plenary';
		}}
	/>
{:else if meetingPhase === 'lobby'}
	<MeetingLobby
		title={event?.name ?? 'Meeting'}
		scheduledTime={scheduledTimeText}
		endedTime={event ? `${formatDateShort(event.endTime)} ${formatTime(event.endTime)}` : ''}
		participants={otherParticipants}
		{callStatus}
		{isModerator}
		onStartMeeting={handleStartMeeting}
	/>
{:else}
	<!-- In-call: full-width black background, stays in document flow -->
	<div class="bg-sidebar flex h-dvh w-full flex-col overflow-hidden">
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
						<div class="flex items-center gap-2">
							<div class="flex items-center gap-1.5">
								<span
									class="{jitsiModeratorStatus
										? 'bg-green-500'
										: 'bg-yellow-500'} h-2.5 w-2.5 rounded-full"
								></span>
								<span
									class="text-sidebar-foreground text-center text-xs leading-6 font-normal"
								>
									{jitsiModeratorStatus ? 'Moderator' : 'Participant'}
								</span>
							</div>
							{#if currentJitsiRoomName}
								{#if dev}
									<span class="text-sidebar-foreground/60 text-xs">
										Room: {currentJitsiRoomName.slice(0, 8)}...
									</span>
								{/if}
								{#if isBreakoutActive && inBreakoutRoom}
									<div
										class="bg-primary/20 inline-flex items-center gap-2 rounded-full px-3 py-1 text-white"
									>
										<span class="text-xs font-medium">{roomChipText}</span>
										<span class="text-xs opacity-70">· {timeLeftFormatted}</span
										>
									</div>
								{:else}
									<div
										class="bg-sidebar-foreground/20 inline-flex items-center rounded-full px-3 py-1"
									>
										<span class="text-xs font-medium text-white">
											{roomChipText}
										</span>
									</div>
								{/if}
							{/if}
						</div>
					</div>
					{#if dev && isModerator}
						<button
							class="bg-destructive text-destructive-foreground hover:bg-destructive/90 rounded px-3 py-1 text-xs font-medium"
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
						onParticipantJoined={handleParticipantJoined}
						onParticipantLeft={handleParticipantLeft}
						startWithAudioMuted={true}
						configOverwrite={{
							toolbarButtons: [
								'microphone',
								'camera',
								'desktop',
								'participants-pane',
								'chat',
								'raisehand',
								'breakoutrooms',
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
							onAddTime={handleAddTime}
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
							onEndMeeting={() => (showEndMeeting = true)}
						/>
					{/if}
				</SidePanel>
			</div>
		</div>

		<!-- Mobile bottom bar + drawer (in flow, not overlapping iframe) -->
		<Drawer.Root>
			<div
				class="border-sidebar-foreground/20 flex items-center gap-2 border-t px-3 py-2 md:hidden"
			>
				{#if isBreakoutActive}
					<Drawer.Trigger
						class="bg-muted text-muted-foreground flex w-full items-center justify-center rounded-full px-6 py-3 font-semibold"
					>
						Breakout Session
					</Drawer.Trigger>
				{:else}
					<button
						class="text-sidebar-foreground shrink-0 p-1 disabled:opacity-30"
						disabled={mobileAgendaViewIndex <= 0}
						onclick={() => mobileAgendaViewIndex--}
					>
						<ChevronLeft class="h-5 w-5" />
					</button>
					<Drawer.Trigger
						class="flex min-w-0 flex-1 items-center justify-center gap-2 rounded-full px-4 py-2.5 {mobileAgendaViewIndex ===
						currentStep
							? 'bg-primary text-primary-foreground'
							: 'bg-muted text-foreground'}"
					>
						{#if mobileAgendaViewIndex < currentStep}
							<Check class="h-4 w-4 shrink-0" />
						{:else if mobileAgendaViewIndex === currentStep}
							<span class="text-xs font-semibold">Current</span>
						{:else}
							<span class="text-xs font-semibold">{mobileAgendaViewIndex + 1}</span>
						{/if}
						<span class="truncate text-sm font-medium">
							{agendaItems[mobileAgendaViewIndex]?.title ?? 'Agenda'}
						</span>
					</Drawer.Trigger>
					<button
						class="text-sidebar-foreground shrink-0 p-1 disabled:opacity-30"
						disabled={mobileAgendaViewIndex >= agendaItems.length - 1}
						onclick={() => mobileAgendaViewIndex++}
					>
						<ChevronRight class="h-5 w-5" />
					</button>
				{/if}
			</div>
			<Drawer.Content class="bg-muted flex max-h-[80dvh] flex-col rounded-t-3xl">
				{#if isBreakoutActive && isModerator && !inBreakoutRoom}
					<!-- Facilitator: breakout rooms management -->
					<div class="flex flex-col gap-5 p-5">
						<!-- Tabs -->
						<div class="border-b pb-5">
							<div class="flex items-center rounded-2xl p-1.5">
								<button
									class="flex h-9 flex-1 items-center justify-center rounded-2xl px-3 py-2 text-sm transition-all {activePanel ===
									'agenda'
										? 'bg-background text-foreground font-semibold shadow-sm'
										: 'text-muted-foreground font-semibold'}"
									onclick={() => (activePanel = 'agenda')}
								>
									Agenda
								</button>
								<button
									class="flex h-9 flex-1 items-center justify-center rounded-2xl px-3 py-2 text-sm transition-all {activePanel ===
									'breakoutRooms'
										? 'bg-background text-foreground font-semibold shadow-sm'
										: 'text-muted-foreground font-semibold'}"
									onclick={() => (activePanel = 'breakoutRooms')}
								>
									Breakout Session
								</button>
							</div>
						</div>

						{#if activePanel === 'breakoutRooms'}
							<!-- Timer + chips -->
							<div class="flex items-center justify-center gap-2.5 border-b pb-3">
								<span class="text-ring text-sm font-semibold">
									Time left&nbsp;&nbsp;{timeLeftFormatted}
								</span>
								<div class="flex items-center gap-1.5">
									<button
										class="bg-primary/20 text-ring h-6 cursor-pointer rounded-full px-2 text-xs font-medium shadow-sm"
										onclick={() => handleAddTime(-1)}
									>
										-1min
									</button>
									<button
										class="bg-primary/20 text-ring h-6 cursor-pointer rounded-full px-2 text-xs font-medium shadow-sm"
										onclick={() => handleAddTime(1)}
									>
										+1min
									</button>
									<button
										class="bg-primary/20 text-ring h-6 cursor-pointer rounded-full px-2 text-xs font-medium shadow-sm"
										onclick={() => handleAddTime(2)}
									>
										+2min
									</button>
								</div>
							</div>

							<!-- Room carousel -->
							<div class="flex items-center gap-2">
								<button
									class="text-foreground shrink-0 disabled:opacity-30"
									disabled={mobileRoomIndex <= 0}
									onclick={() =>
										(mobileRoomIndex = Math.max(0, mobileRoomIndex - 1))}
								>
									<ChevronLeft class="h-5 w-5" />
								</button>

								{#if breakoutRoomDisplays[mobileRoomIndex]}
									{@const room = breakoutRoomDisplays[mobileRoomIndex]}
									<div
										class="bg-card border-border flex min-w-0 flex-1 flex-col overflow-hidden rounded-[10px] border shadow-sm"
									>
										<div class="flex flex-col gap-2 px-5 py-4">
											<span
												class="text-foreground w-28 truncate text-base leading-6 font-semibold"
											>
												{room.name}
											</span>
											<div class="flex flex-wrap items-start gap-4">
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
															]} flex h-5 w-5 items-center justify-center rounded-full text-xs font-medium text-white uppercase"
														>
															{(p.username ?? p.user_id)
																.charAt(0)
																.toUpperCase()}
														</div>
														<span
															class="text-foreground text-sm font-medium"
														>
															{p.username ?? p.user_id.slice(0, 8)}
														</span>
													</div>
												{/each}
											</div>
										</div>
										<div class="border-border flex items-center border-t p-2">
											<button
												class="text-foreground hover:bg-muted flex flex-1 items-center justify-center gap-2 rounded-lg px-3 py-2 text-xs font-medium"
												onclick={() => handleEnterBreakoutRoom(room.index)}
											>
												<MoveUpRight class="h-4 w-4" />
												Enter
												{#if room.hasAssistanceRequest}
													<span
														class="bg-destructive h-2 w-2 shrink-0 rounded-full"
													></span>
												{/if}
											</button>
										</div>
									</div>
								{/if}

								<button
									class="text-foreground shrink-0 disabled:opacity-30"
									disabled={mobileRoomIndex >= breakoutRoomDisplays.length - 1}
									onclick={() =>
										(mobileRoomIndex = Math.min(
											breakoutRoomDisplays.length - 1,
											mobileRoomIndex + 1
										))}
								>
									<ChevronRight class="h-5 w-5" />
								</button>
							</div>

							<!-- Broadcast + End -->
							<div class="flex flex-col gap-1.5">
								<Button
									variant="primaryDark"
									class="h-10 w-full text-sm font-medium"
									onclick={() => (showBroadcast = true)}
								>
									<Megaphone class="mr-1.5 h-4 w-4" />
									Broadcast message
								</Button>
								<Button
									variant="outline"
									class="border-input text-destructive hover:bg-destructive/5 hover:text-destructive h-10 w-full"
									onclick={handleEndBreakoutSession}
								>
									<CircleStop class="mr-1.5 h-4 w-4" />
									End breakout session
								</Button>
							</div>
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
					</div>
				{:else if isBreakoutActive && inBreakoutRoom}
					<!-- In breakout room (facilitator or participant) -->
					<div class="p-3">
						<BreakoutSessionPanel
							roomName={roomChipText}
							question={currentAgendaItem?.breakoutQuestion}
							description={currentAgendaItem?.breakoutDescription}
							{timeLeftFormatted}
							{isModerator}
							onCallForSupport={handleCallForSupport}
							onLeaveBreakoutRoom={handleLeaveBreakoutRoom}
						/>
					</div>
				{:else}
					<div class="p-4">
						<AgendaPanel
							items={agendaItems}
							{currentStep}
							{isModerator}
							readOnly={isBreakoutActive}
							onSetCurrent={handleSetAgendaItem}
							onNext={handleNextAgendaItem}
							onEndMeeting={() => (showEndMeeting = true)}
						/>
					</div>
				{/if}
			</Drawer.Content>
		</Drawer.Root>
	</div>
{/if}

<!-- Dialogs -->
<CreateBreakoutDialog
	bind:open={showCreateBreakout}
	participants={inCallParticipants}
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

<AddTimeDialog
	bind:open={showAddTime}
	{timeLeftFormatted}
	onClose={() => (showAddTime = false)}
	onAddTime={handleAddTime}
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
