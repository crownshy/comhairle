<script lang="ts">
	import { page } from '$app/state';
	import JitsiMeet from '$lib/components/JitsiMeet/JitsiMeet.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import { apiClient } from '@crown-shy/api-client/client';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	let conversationId = $derived(data.conversationId);
	let eventId = $derived(data.eventId);
	let event = $derived(data.event);
	let apiAttendances = $derived(data.attendances);
	let user = $derived(data.user);

	let roomName = $derived(`comhairle-event-${eventId}`);

	let jitsiApi: any = $state(null);
	let panelOpen = $state(false);
	let activeTab: 'details' | 'participants' | 'controls' = $state('details');

	// Jitsi-synced state
	let jitsiParticipants = $state<Array<{ id: string; displayName: string }>>([]);
	let conferenceJoined = $state(false);
	let audioMuted = $state(false);
	let videoMuted = $state(false);
	let attendanceRegistered = $state(false);

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
		jitsiParticipants = [...jitsiParticipants, { id: data.id, displayName: data.displayName || 'Guest' }];
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
		{ key: 'details' as const, label: 'Details', icon: '☰' },
		{ key: 'participants' as const, label: 'People', icon: '👥' },
		{ key: 'controls' as const, label: 'Controls', icon: '⚙' }
	];
</script>

<svelte:head>
	<title>{event?.name ?? 'Live Event'}</title>
</svelte:head>

<div class="flex h-dvh flex-col overflow-hidden md:-mx-20 -mb-4">
	<!-- Top bar -->
	<div
		class="flex items-center justify-between border-b border-border bg-card px-3 py-1.5 md:px-4 md:py-2"
	>
		<div class="flex items-center gap-2 md:gap-3 overflow-hidden">
			<a
				href="/conversations/{conversationId}/events/{eventId}"
				class="hidden text-sm text-muted-foreground hover:text-foreground sm:inline"
			>← Event</a>
			<span class="hidden text-sm text-border sm:inline">|</span>
			<h1 class="truncate text-xs font-semibold md:text-sm">
				{event?.name ?? `Event: ${eventId}`}
			</h1>
			{#if conferenceJoined}
				<span class="inline-flex shrink-0 items-center gap-1 rounded-full bg-green-500/10 px-1.5 py-0.5 text-[10px] font-medium text-green-600 md:gap-1.5 md:px-2 md:text-xs">
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
				class="fixed inset-x-0 bottom-0 z-40 flex max-h-[70dvh] flex-col rounded-t-2xl border-t border-border bg-card shadow-lg
					md:relative md:inset-auto md:z-auto md:max-h-none md:w-80 md:shrink-0 md:rounded-none md:border-l md:border-t-0 md:shadow-none"
			>
				<!-- Drag handle (mobile only) -->
				<div class="flex justify-center py-2 md:hidden">
					<div class="h-1 w-8 rounded-full bg-muted-foreground/30"></div>
				</div>

				<!-- Panel tabs -->
				<div class="flex border-b border-border">
					{#each tabs as tab}
						<button
							class="flex-1 px-2 py-2 text-xs font-medium transition-colors md:py-2.5 {activeTab === tab.key
								? 'border-b-2 border-primary text-foreground'
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
					{#if activeTab === 'details'}
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
												{event.currentAttendance}{event.capacity ? ` / ${event.capacity}` : ''}
											</span>
										</div>
										<div class="flex justify-between">
											<span class="text-muted-foreground">Signup</span>
											<span class="capitalize">{event.signupMode}</span>
										</div>
									</div>
								</div>
							{:else}
								<p class="text-xs text-muted-foreground">Event details unavailable.</p>
							{/if}
						</div>

					{:else if activeTab === 'participants'}
						<div class="space-y-2">
							<h3 class="text-sm font-semibold">
								In Call ({jitsiParticipants.length})
							</h3>

							{#if jitsiParticipants.length === 0}
								<p class="py-6 text-center text-xs text-muted-foreground md:py-8">
									{conferenceJoined ? 'No other participants yet' : 'Join the meeting to see participants'}
								</p>
							{:else}
								{#each jitsiParticipants as p (p.id)}
									<div class="flex items-center gap-2 rounded-lg border border-border p-2">
										<div class="flex h-7 w-7 items-center justify-center rounded-full bg-primary/10 text-xs font-medium text-primary">
											{p.displayName.charAt(0).toUpperCase()}
										</div>
										<span class="text-sm">{p.displayName}</span>
									</div>
								{/each}
							{/if}

							{#if apiAttendances.length > 0}
								<hr class="my-3 border-border" />
								<h3 class="text-sm font-semibold">
									Registered ({apiAttendances.length})
								</h3>
								{#each apiAttendances as a (a.id)}
									<div class="flex items-center gap-2 rounded-lg border border-border p-2">
										<div class="flex h-7 w-7 items-center justify-center rounded-full bg-muted text-xs font-medium text-muted-foreground">
											{a.userId.charAt(0).toUpperCase()}
										</div>
										<div class="min-w-0 flex-1">
											<span class="text-sm">{a.userId}</span>
											<span class="ml-1 text-xs text-muted-foreground capitalize">{a.role}</span>
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
								<p class="text-xs font-medium uppercase tracking-wide text-muted-foreground">
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
										onclick={() => jitsiApi?.executeCommand('toggleLobby', true)}
									>
										Lobby
									</Button>
								</div>
							</div>

							<div class="mt-3 rounded-lg border border-dashed border-border p-3 text-center">
								<p class="text-xs text-muted-foreground">
									<code class="text-xs">jitsiApi</code> is available in the browser console.
								</p>
							</div>
						</div>
					{/if}
				</div>
			</div>
		{/if}
	</div>
</div>
