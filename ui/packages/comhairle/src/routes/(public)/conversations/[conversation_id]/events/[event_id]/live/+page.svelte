<script lang="ts">
	import { page } from '$app/state';
	import JitsiMeet from '$lib/components/JitsiMeet/JitsiMeet.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import { apiClient } from '@crownshy/api-client/client';
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
	let panelOpen = $state(false);
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
		{ key: 'agenda' as const, label: 'Agenda', icon: '☰' },
		{ key: 'details' as const, label: 'Details', icon: 'ℹ' },
		{ key: 'participants' as const, label: 'People', icon: '👥' },
		{ key: 'controls' as const, label: 'Controls', icon: '⚙' }
	];
</script>

<svelte:head>
	<title>{event?.name ?? 'Live Event'}</title>
</svelte:head>

<div class="-mb-4 flex h-dvh flex-col overflow-hidden md:-mx-20">
	<!-- Top bar -->
	<div
		class="border-border bg-card flex items-center justify-between border-b px-3 py-1.5 md:px-4 md:py-2"
	>
		<div class="flex items-center gap-2 overflow-hidden md:gap-3">
			<a
				href="/conversations/{conversationId}/events/{eventId}"
				class="text-muted-foreground hover:text-foreground hidden text-sm sm:inline"
				>← Event</a
			>
			<span class="text-border hidden text-sm sm:inline">|</span>
			<h1 class="truncate text-xs font-semibold md:text-sm">
				{event?.name ?? `Event: ${eventId}`}
			</h1>
			{#if conferenceJoined}
				<span
					class="inline-flex shrink-0 items-center gap-1 rounded-full bg-green-500/10 px-1.5 py-0.5 text-[10px] font-medium text-green-600 md:gap-1.5 md:px-2 md:text-xs"
				>
					<span class="h-1.5 w-1.5 rounded-full bg-green-500"></span>
					Live
				</span>
			{/if}
		</div>

		<Button
			variant="outline"
			size="sm"
			class="shrink-0 text-xs md:text-sm"
			onclick={() => (panelOpen = !panelOpen)}
		>
			<span class="md:hidden">{panelOpen ? '✕' : 'Show Panel'}</span>
			<span class="hidden md:inline">{panelOpen ? 'Hide Panel' : 'Show Panel'}</span>
		</Button>
	</div>

	<!-- Main content area: row on desktop, column on mobile -->
	<div class="relative flex min-h-0 flex-1 flex-col md:flex-row">
		<!-- Jitsi iframe area -->
		<div class="relative min-h-0 min-w-0 flex-1">
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

		<!-- Panel: bottom drawer on mobile, right sidebar on desktop -->
		{#if panelOpen}
			<!-- Mobile: backdrop -->
			<button
				class="fixed inset-0 z-30 bg-black/40 md:hidden"
				onclick={() => (panelOpen = false)}
				aria-label="Close panel"
			></button>

			<div
				class="border-border bg-card fixed inset-x-0 bottom-0 z-40 flex max-h-[70dvh] flex-col rounded-t-2xl border-t shadow-lg
					md:relative md:inset-auto md:z-auto md:max-h-none md:w-80 md:shrink-0 md:rounded-none md:border-t-0 md:border-l md:shadow-none"
			>
				<!-- Drag handle (mobile only) -->
				<div class="flex justify-center py-2 md:hidden">
					<div class="bg-muted-foreground/30 h-1 w-8 rounded-full"></div>
				</div>

				<!-- Panel tabs -->
				<div class="border-border flex border-b">
					{#each tabs as tab}
						<button
							class="flex-1 px-2 py-2 text-xs font-medium transition-colors md:py-2.5 {activeTab ===
							tab.key
								? 'border-primary text-foreground border-b-2'
								: 'text-muted-foreground hover:text-foreground'}"
							onclick={() => (activeTab = tab.key)}
						>
							<span class="mr-1">{tab.icon}</span>
							{tab.label}
						</button>
					{/each}
				</div>

				<!-- Tab content -->
				<div class="flex-1 overflow-y-auto p-3">
					{#if activeTab === 'agenda'}
						<div class="space-y-2">
							<div class="flex items-center justify-between">
								<h3 class="text-sm font-semibold">Meeting Agenda</h3>
								<Button variant="outline" size="sm" onclick={advanceAgenda}>
									Next →
								</Button>
							</div>

							{#each agendaItems as item (item.id)}
								<div
									class="rounded-lg border p-2.5 transition-colors md:p-3 {item.status ===
									'current'
										? 'border-primary bg-primary/5'
										: item.status === 'done'
											? 'border-border bg-muted/30 opacity-60'
											: 'border-border'}"
								>
									<div class="flex items-start justify-between gap-2">
										<div class="flex items-center gap-2">
											{#if item.status === 'done'}
												<span class="text-green-500">✓</span>
											{:else if item.status === 'current'}
												<span
													class="bg-primary h-2 w-2 animate-pulse rounded-full"
												></span>
											{:else}
												<span
													class="bg-muted-foreground/30 h-2 w-2 rounded-full"
												></span>
											{/if}
											<span class="text-sm font-medium">{item.title}</span>
										</div>
										<span
											class="text-muted-foreground text-xs whitespace-nowrap"
											>{item.duration}</span
										>
									</div>
								</div>
							{/each}

							<div
								class="border-border mt-4 rounded-lg border border-dashed p-3 text-center md:p-4"
							>
								<p class="text-muted-foreground text-xs">
									Prototype agenda panel Can we change anything here? thoughts?
								</p>
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
								<p class="text-muted-foreground text-xs">
									Event details unavailable.
								</p>
							{/if}
						</div>
					{:else if activeTab === 'participants'}
						<div class="space-y-2">
							<h3 class="text-sm font-semibold">
								In Call ({jitsiParticipants.length})
							</h3>

							{#if jitsiParticipants.length === 0}
								<p class="text-muted-foreground py-6 text-center text-xs md:py-8">
									{conferenceJoined
										? 'No other participants yet'
										: 'Join the meeting to see participants'}
								</p>
							{:else}
								{#each jitsiParticipants as p (p.id)}
									<div
										class="border-border flex items-center gap-2 rounded-lg border p-2"
									>
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
									<div
										class="border-border flex items-center gap-2 rounded-lg border p-2"
									>
										<div
											class="bg-muted text-muted-foreground flex h-7 w-7 items-center justify-center rounded-full text-xs font-medium"
										>
											{a.userId.charAt(0).toUpperCase()}
										</div>
										<div class="min-w-0 flex-1">
											<span class="text-sm">{a.userId}</span>
											<span
												class="text-muted-foreground ml-1 text-xs capitalize"
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

							<div class="grid grid-cols-2 gap-2 md:grid-cols-1">
								<Button
									variant={audioMuted ? 'default' : 'outline'}
									size="sm"
									class="w-full justify-start text-xs md:text-sm"
									onclick={() => jitsiApi?.executeCommand('toggleAudio')}
								>
									{audioMuted ? 'Unmute' : 'Mute'}
								</Button>

								<Button
									variant={videoMuted ? 'default' : 'outline'}
									size="sm"
									class="w-full justify-start text-xs md:text-sm"
									onclick={() => jitsiApi?.executeCommand('toggleVideo')}
								>
									{videoMuted ? 'Video On' : 'Video Off'}
								</Button>

								<Button
									variant="outline"
									size="sm"
									class="w-full justify-start text-xs md:text-sm"
									onclick={() => jitsiApi?.executeCommand('toggleShareScreen')}
								>
									Share
								</Button>

								<Button
									variant="outline"
									size="sm"
									class="w-full justify-start text-xs md:text-sm"
									onclick={() => jitsiApi?.executeCommand('toggleTileView')}
								>
									Tiles
								</Button>

								<Button
									variant="outline"
									size="sm"
									class="w-full justify-start text-xs md:text-sm"
									onclick={() => jitsiApi?.executeCommand('toggleRaiseHand')}
								>
									Hand
								</Button>

								<Button
									variant="outline"
									size="sm"
									class="w-full justify-start text-xs md:text-sm"
									onclick={() => jitsiApi?.executeCommand('muteEveryone')}
								>
									Mute All
								</Button>
							</div>

							<hr class="border-border" />

							<div class="space-y-2">
								<p
									class="text-muted-foreground text-xs font-medium tracking-wide uppercase"
								>
									API Explorer
								</p>

								<div class="grid grid-cols-2 gap-2 md:grid-cols-1">
									<Button
										variant="outline"
										size="sm"
										class="w-full justify-start text-xs md:text-sm"
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
										class="w-full justify-start text-xs md:text-sm"
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
										class="w-full justify-start text-xs md:text-sm"
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
										class="w-full justify-start text-xs md:text-sm"
										onclick={() =>
											jitsiApi?.executeCommand('toggleLobby', true)}
									>
										Lobby
									</Button>
								</div>
							</div>
						</div>
					{/if}
				</div>
			</div>
		{/if}
	</div>
</div>
