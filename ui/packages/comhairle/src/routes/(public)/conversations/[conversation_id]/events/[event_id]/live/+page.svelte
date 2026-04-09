<script lang="ts">
	import { page } from '$app/state';
	import JitsiMeet from '$lib/components/JitsiMeet/JitsiMeet.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import * as Drawer from '$lib/components/ui/drawer';
	import { apiClient } from '@crownshy/api-client/client';
	import {
		List,
		Info,
		Users,
		Settings,
		CircleCheck,
		ChevronRight,
		ChevronUp
	} from 'lucide-svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	let conversationId = $derived(data.conversationId);
	let eventId = $derived(data.eventId);
	let event = $derived(data.event);
	let jwt = $derived(data.jwt);
	let apiAttendances = $derived(data.attendances);
	let user = $derived(data.user);

	let roomName = $derived(event?.videoMeetingId);

	let jitsiApi: any = $state(null);
	let panelOpen = $state(true);
	let activeTab: 'agenda' | 'details' | 'participants' | 'controls' = $state('agenda');

	// Jitsi-synced state
	let jitsiParticipants = $state<Array<{ id: string; displayName: string }>>([]);
	let conferenceJoined = $state(false);
	let audioMuted = $state(false);
	let videoMuted = $state(false);
	let attendanceRegistered = $state(false);

	// Prototype agenda items
	let agendaItems = $state([
		{
			id: '1',
			title: 'Welcome & Introductions',
			duration: '5 min',
			status: 'current' as const
		},
		{ id: '2', title: 'Topic Discussion', duration: '20 min', status: 'upcoming' as const },
		{ id: '3', title: 'Q&A Session', duration: '10 min', status: 'upcoming' as const },
		{ id: '4', title: 'Wrap-up & Next Steps', duration: '5 min', status: 'upcoming' as const }
	]);

	function advanceAgenda() {
		const currentIdx = agendaItems.findIndex((item) => item.status === 'current');
		if (currentIdx === -1) return;

		agendaItems = agendaItems.map((item, i) => {
			if (i === currentIdx) return { ...item, status: 'done' as const };
			if (i === currentIdx + 1) return { ...item, status: 'current' as const };
			return item;
		});
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

		// Auto-register attendance when joining the live call
		if (user && !attendanceRegistered) {
			try {
				await apiClient.CreateEventAttendance({
					params: {
						conversation_id: conversationId,
						event_id: eventId
					},
					body: { role: 'attendee' }
				});
				attendanceRegistered = true;
			} catch (e) {
				// May already be registered — that's fine
				console.warn('Attendance registration:', e);
				attendanceRegistered = true;
			}
		}
	}

	function handleConferenceLeft() {
		conferenceJoined = false;
		jitsiParticipants = [];
	}

	function formatDate(iso: string) {
		return new Date(iso).toLocaleDateString(undefined, {
			weekday: 'short',
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	const tabs = [
		{ key: 'agenda' as const, label: 'Agenda', icon: List },
		{ key: 'details' as const, label: 'Details', icon: Info },
		{ key: 'participants' as const, label: 'People', icon: Users },
		{ key: 'controls' as const, label: 'Controls', icon: Settings }
	];
</script>

{#snippet panelTabs()}
	<div class="border-border flex border-b-[3px]">
		{#each tabs as tab}
			<button
				class="flex-1 py-1.5 transition-colors {activeTab === tab.key
					? 'border-primary text-foreground border-b-[3px] font-semibold'
					: 'text-muted-foreground hover:text-foreground font-medium'}"
				onclick={() => (activeTab = tab.key)}
			>
				<span class="inline-flex items-center gap-2 rounded-lg px-3 py-2">
					<tab.icon class="h-4 w-4" />
					<span class="text-base">{tab.label}</span>
				</span>
			</button>
		{/each}
	</div>
{/snippet}

{#snippet panelContent()}
	<div class="flex-1 overflow-y-auto p-6">
		{#if activeTab === 'agenda'}
			<div class="flex flex-col gap-6">
				<div class="flex items-center justify-between">
					<h3 class="text-2xl font-semibold">Meeting Agenda</h3>
					<Button variant="outline" onclick={advanceAgenda}>
						Next
						<ChevronRight class="h-4 w-4" />
					</Button>
				</div>

				<div class="flex flex-col gap-3">
					{#each agendaItems as item (item.id)}
						<div
							class="flex h-14 items-center justify-between overflow-hidden rounded-full px-5 py-2 shadow-sm transition-colors
								{item.status === 'current'
								? 'bg-primary/10 ring-primary ring-2'
								: item.status === 'done'
									? 'bg-muted ring-muted ring-1'
									: 'bg-background ring-border ring-1'}"
						>
							<div class="flex items-center gap-2">
								{#if item.status === 'done'}
									<CircleCheck class="h-5 w-5 text-emerald-500" />
								{:else if item.status === 'current'}
									<span class="bg-primary h-3 w-3 animate-pulse rounded-full"
									></span>
								{:else}
									<span class="bg-border h-3 w-3 rounded-full"></span>
								{/if}
								<span
									class="line-clamp-1 text-base {item.status === 'current'
										? 'font-semibold'
										: 'font-medium'}">{item.title}</span
								>
							</div>
							<span class="text-primary line-clamp-1 text-sm font-medium"
								>{item.duration}</span
							>
						</div>
					{/each}
				</div>
			</div>
		{:else if activeTab === 'details'}
			<div class="space-y-3">
				<h3 class="text-sm font-semibold">Event Details</h3>

				{#if event}
					<div class="space-y-2 text-sm">
						{#if event.description}
							<p class="text-muted-foreground">{event.description}</p>
						{/if}
						<div class="grid gap-2 text-xs">
							<div class="flex justify-between">
								<span class="text-muted-foreground">Starts</span>
								<span>{formatDate(event.startTime)}</span>
							</div>
							<div class="flex justify-between">
								<span class="text-muted-foreground">Ends</span>
								<span>{formatDate(event.endTime)}</span>
							</div>
							<div class="flex justify-between">
								<span class="text-muted-foreground">Attendance</span>
								<span>
									{event.currentAttendance}{event.capacity
										? ` / ${event.capacity}`
										: ''}
								</span>
							</div>
							<div class="flex justify-between">
								<span class="text-muted-foreground">Signup</span>
								<span class="capitalize">{event.signupMode}</span>
							</div>
						</div>
					</div>
				{:else}
					<p class="text-muted-foreground text-xs">Event details unavailable.</p>
				{/if}
			</div>
		{:else if activeTab === 'participants'}
			<div class="space-y-2">
				<h3 class="text-sm font-semibold">
					In Call ({jitsiParticipants.length})
				</h3>

				{#if jitsiParticipants.length === 0}
					<p class="text-muted-foreground py-6 text-center text-xs">
						{conferenceJoined
							? 'No other participants yet'
							: 'Join the meeting to see participants'}
					</p>
				{:else}
					{#each jitsiParticipants as p (p.id)}
						<div class="border-border flex items-center gap-2 rounded-lg border p-2">
							<div
								class="bg-primary/10 text-primary flex h-7 w-7 items-center justify-center rounded-full text-xs font-medium"
							>
								{p.displayName.charAt(0).toUpperCase()}
							</div>
							<span class="text-sm">{p.displayName}</span>
						</div>
					{/each}
				{/if}

				{#if apiAttendances.length > 0}
					<hr class="border-border my-3" />
					<h3 class="text-sm font-semibold">
						Registered ({apiAttendances.length})
					</h3>
					{#each apiAttendances as a (a.id)}
						<div class="border-border flex items-center gap-2 rounded-lg border p-2">
							<div
								class="bg-muted text-muted-foreground flex h-7 w-7 items-center justify-center rounded-full text-xs font-medium"
							>
								{a.userId.charAt(0).toUpperCase()}
							</div>
							<div class="min-w-0 flex-1">
								<span class="text-sm">{a.userId}</span>
								<span class="text-muted-foreground ml-1 text-xs capitalize"
									>{a.role}</span
								>
							</div>
						</div>
					{/each}
				{/if}
			</div>
		{:else if activeTab === 'controls'}
			<div class="space-y-3">
				<h3 class="text-sm font-semibold">Meeting Controls</h3>

				<div class="grid grid-cols-2 gap-2">
					<Button
						variant={audioMuted ? 'default' : 'outline'}
						size="sm"
						class="w-full justify-start text-xs"
						onclick={() => jitsiApi?.executeCommand('toggleAudio')}
					>
						{audioMuted ? 'Unmute' : 'Mute'}
					</Button>

					<Button
						variant={videoMuted ? 'default' : 'outline'}
						size="sm"
						class="w-full justify-start text-xs"
						onclick={() => jitsiApi?.executeCommand('toggleVideo')}
					>
						{videoMuted ? 'Video On' : 'Video Off'}
					</Button>

					<Button
						variant="outline"
						size="sm"
						class="w-full justify-start text-xs"
						onclick={() => jitsiApi?.executeCommand('toggleShareScreen')}
					>
						Share
					</Button>

					<Button
						variant="outline"
						size="sm"
						class="w-full justify-start text-xs"
						onclick={() => jitsiApi?.executeCommand('toggleTileView')}
					>
						Tiles
					</Button>

					<Button
						variant="outline"
						size="sm"
						class="w-full justify-start text-xs"
						onclick={() => jitsiApi?.executeCommand('toggleRaiseHand')}
					>
						Hand
					</Button>

					<Button
						variant="outline"
						size="sm"
						class="w-full justify-start text-xs"
						onclick={() => jitsiApi?.executeCommand('muteEveryone')}
					>
						Mute All
					</Button>
				</div>

				<hr class="border-border" />

				<div class="space-y-2">
					<p class="text-muted-foreground text-xs font-medium tracking-wide uppercase">
						API Explorer
					</p>

					<div class="grid grid-cols-2 gap-2">
						<Button
							variant="outline"
							size="sm"
							class="w-full justify-start text-xs"
							onclick={async () => {
								if (!jitsiApi) return;
								const rooms = await jitsiApi.getRoomsInfo();
								console.log('Rooms info:', rooms);
								alert(JSON.stringify(rooms, null, 2));
							}}
						>
							Rooms
						</Button>

						<Button
							variant="outline"
							size="sm"
							class="w-full justify-start text-xs"
							onclick={async () => {
								if (!jitsiApi) return;
								const devices = await jitsiApi.getAvailableDevices();
								console.log('Available devices:', devices);
								alert(JSON.stringify(devices, null, 2));
							}}
						>
							Devices
						</Button>

						<Button
							variant="outline"
							size="sm"
							class="w-full justify-start text-xs"
							onclick={() => {
								if (!jitsiApi) return;
								const n = jitsiApi.getNumberOfParticipants();
								alert(`Participants: ${n}`);
							}}
						>
							Count
						</Button>

						<Button
							variant="outline"
							size="sm"
							class="w-full justify-start text-xs"
							onclick={() => jitsiApi?.executeCommand('toggleLobby', true)}
						>
							Lobby
						</Button>
					</div>
				</div>
			</div>
		{/if}
	</div>
{/snippet}

<svelte:head>
	<title>{event?.name ?? 'Live Event'}</title>
</svelte:head>

<!-- ===== mobile (< md) ===== -->
<div class="-mb-4 flex h-dvh flex-col overflow-hidden md:hidden">
	<!-- Back bar -->
	<div class="bg-card flex items-center gap-3 px-6 py-3">
		<a
			href="/conversations/{conversationId}/events/{eventId}"
			class="text-muted-foreground hover:text-foreground inline-flex items-center gap-2 text-sm font-medium"
		>
			<span
				class="border-input flex h-9 w-9 items-center justify-center rounded-full border bg-white shadow-sm"
				>←</span
			>
			Back to conversation
		</a>
	</div>

	<!-- Jitsi fills the rest -->
	<div class="relative mx-4 min-h-0 flex-1 overflow-hidden rounded-3xl">
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

	<!-- Mobile Drawer trigger + bottom sheet -->
	<Drawer.Root>
		<Drawer.Trigger
			class="bg-primary hover:bg-primary/90 fixed bottom-4 left-1/2 z-50 inline-flex -translate-x-1/2 items-center gap-2 rounded-full px-6 py-3 font-semibold text-white shadow-lg transition-colors"
		>
			<ChevronUp class="h-4 w-4" />
			<span>Agenda</span>
		</Drawer.Trigger>
		<Drawer.Content class="bg-card flex max-h-[80dvh] flex-col rounded-t-3xl">
			{@render panelTabs()}
			{@render panelContent()}
		</Drawer.Content>
	</Drawer.Root>
</div>

<!-- ===== desktop (md+) ===== -->
<div class="bg-muted -mb-4 hidden min-h-dvh flex-col overflow-hidden md:-mx-20 md:flex">
	<!-- Top bar -->
	<div class="mx-auto w-full max-w-[1440px] px-6 pt-12 pb-4">
		<div class="flex items-center gap-3">
			<a
				href="/conversations/{conversationId}/events/{eventId}"
				class="text-muted-foreground hover:text-foreground inline-flex items-center gap-2 text-sm font-medium"
			>
				<span
					class="border-input flex h-9 w-9 items-center justify-center rounded-full border bg-white shadow-sm"
					>←</span
				>
				Back to conversation
			</a>
			<span class="text-muted-foreground text-sm">|</span>
			<h1 class="text-foreground text-lg font-semibold">
				{event?.name ?? `Event: ${eventId}`}
			</h1>
			{#if conferenceJoined}
				<span
					class="inline-flex shrink-0 items-center gap-1.5 rounded-full bg-green-500/10 px-2 py-0.5 text-xs font-medium text-green-600"
				>
					<span class="h-1.5 w-1.5 rounded-full bg-green-500"></span>
					Live
				</span>
			{/if}
		</div>
	</div>

	<!-- Half-and-half -->
	<div class="mx-auto flex w-full max-w-[1440px] flex-1 gap-16 px-6 pb-24">
		<!-- Jitsi -->
		<div class="relative min-h-[600px] min-w-0 flex-1 overflow-hidden rounded-3xl">
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

		<!-- Panel card -->
		<div
			class="bg-card flex min-w-0 flex-1 flex-col overflow-hidden rounded-3xl shadow-[0px_2px_4px_0px_rgba(0,0,0,0.12)]"
		>
			{@render panelTabs()}
			{@render panelContent()}
		</div>
	</div>
</div>
