<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { env } from '$env/dynamic/public';

	interface JitsiMeetProps {
		domain?: string;
		roomName: string;
		displayName?: string;
		email?: string;
		jwt: string;
		width?: string | number;
		height?: string | number;
		startWithAudioMuted?: boolean;
		startWithVideoMuted?: boolean;
		configOverwrite?: Record<string, any>;
		interfaceConfigOverwrite?: Record<string, any>;
		loadingMessage?: string;
		onApiReady?: (api: any) => void;
		onReadyToClose?: () => void;
		onParticipantJoined?: (participant: any) => void;
		onParticipantLeft?: (participant: any) => void;
		onVideoConferenceJoined?: (data: any) => void;
		onVideoConferenceLeft?: (data: any) => void;
		onBreakoutRoomsUpdated?: (rooms: Record<string, any>) => void;
	}

	let {
		domain = env.PUBLIC_JITSI_DOMAIN ?? 'jitsi.comhairle.scot',
		roomName,
		displayName = '',
		email = '',
		jwt,
		width = '100%',
		height = '100%',
		startWithAudioMuted = false,
		startWithVideoMuted = false,
		configOverwrite = {},
		interfaceConfigOverwrite = {},
		loadingMessage = 'Connecting to meeting...',
		onApiReady,
		onReadyToClose,
		onParticipantJoined,
		onParticipantLeft,
		onVideoConferenceJoined,
		onVideoConferenceLeft,
		onBreakoutRoomsUpdated
	}: JitsiMeetProps = $props();

	let containerEl: HTMLDivElement;
	let api: any = $state(null);
	let loading = $state(false);
	let error = $state<string | null>(null);

	function loadExternalApi(): Promise<void> {
		return new Promise((resolve, reject) => {
			if ((window as any).JitsiMeetExternalAPI) {
				resolve();
				return;
			}
			const script = document.createElement('script');
			script.src = `https://${domain}/external_api.js`;
			script.async = true;
			script.onload = () => resolve();
			script.onerror = () => reject(new Error(`Failed to load Jitsi API from ${domain}`));
			document.head.appendChild(script);
		});
	}

	function initJitsi() {
		console.log(
			'[JITSI] initJitsi called for room:',
			roomName,
			'| jwt present:',
			!!jwt,
			'| jwt length:',
			jwt?.length
		);
		const JitsiMeetExternalAPI = (window as any).JitsiMeetExternalAPI;
		if (!JitsiMeetExternalAPI) {
			error = 'JitsiMeetExternalAPI not available';
			loading = false;
			return;
		}

		const options: Record<string, any> = {
			roomName,
			width,
			height,
			parentNode: containerEl,
			configOverwrite: {
				startWithAudioMuted,
				startWithVideoMuted,
				prejoinPageEnabled: false,
				...configOverwrite
			},
			interfaceConfigOverwrite: {
				SHOW_JITSI_WATERMARK: false,
				SHOW_WATERMARK_FOR_GUESTS: false,
				...interfaceConfigOverwrite
			}
		};

		if (jwt) options.jwt = jwt;

		if (displayName || email) {
			options.userInfo = {};
			if (displayName) options.userInfo.displayName = displayName;
			if (email) options.userInfo.email = email;
		}

		try {
			console.log('[JITSI] Creating JitsiMeetExternalAPI:', {
				domain,
				roomName,
				jwt: jwt ? jwt.substring(0, 50) + '...' : 'none'
			});
			api = new JitsiMeetExternalAPI(domain, options);

			api.addListener('videoConferenceJoined', (data: any) => {
				console.log('[JITSI] videoConferenceJoined:', data);
				loading = false;
				onVideoConferenceJoined?.(data);
			});

			api.addListener('readyToClose', () => {
				console.log('[JITSI] readyToClose fired');
				onReadyToClose?.();
			});

			api.addListener('participantJoined', (data: any) => {
				console.log('[JITSI] participantJoined:', data);
				onParticipantJoined?.(data);
			});

			api.addListener('participantLeft', (data: any) => {
				console.log('[JITSI] participantLeft:', data);
				onParticipantLeft?.(data);
			});

			api.addListener('videoConferenceLeft', (data: any) => {
				console.log('[JITSI] videoConferenceLeft:', data);
				onVideoConferenceLeft?.(data);
			});

			api.addListener('breakoutRoomsUpdated', (data: any) => {
				console.log('[JITSI] breakoutRoomsUpdated:', data);
				onBreakoutRoomsUpdated?.(data);
			});

			onApiReady?.(api);
		} catch (e) {
			console.error('[JITSI] Failed to initialize:', e);
			error = e instanceof Error ? e.message : 'Failed to initialize Jitsi';
			loading = false;
		}
	}

	onMount(() => {
		if (!browser) return;

		loadExternalApi()
			.then(() => initJitsi())
			.catch((e) => {
				error = e instanceof Error ? e.message : 'Failed to load Jitsi API';
				loading = false;
			});

		return () => {
			console.log('[JITSI] onMount cleanup — disposing API for room:', roomName);
			if (api) {
				api.dispose();
				api = null;
			}
		};
	});

	export function getApi() {
		return api;
	}

	export function executeCommand(command: string, ...args: any[]) {
		if (api) {
			api.executeCommand(command, ...args);
		}
	}

	export function toggleAudio() {
		executeCommand('toggleAudio');
	}

	export function toggleVideo() {
		executeCommand('toggleVideo');
	}

	export function hangup() {
		executeCommand('hangup');
	}

	export function setTileView(enabled: boolean) {
		executeCommand('setTileView', enabled);
	}

	export function toggleScreenShare() {
		executeCommand('toggleShareScreen');
	}
</script>

<div class="relative h-full w-full overflow-hidden rounded-xl">
	{#if loading}
		<div class="bg-muted/50 absolute inset-0 z-10 flex items-center justify-center">
			<div class="flex flex-col items-center gap-3">
				<div
					class="border-primary h-8 w-8 animate-spin rounded-full border-4 border-t-transparent"
				></div>
				<span class="text-muted-foreground text-sm">{loadingMessage}</span>
			</div>
		</div>
	{/if}

	{#if error}
		<div class="bg-destructive/10 absolute inset-0 z-10 flex items-center justify-center">
			<div class="flex flex-col items-center gap-2 text-center">
				<span class="text-destructive text-sm font-medium">{error}</span>
				<button
					class="text-muted-foreground hover:text-foreground text-xs underline"
					onclick={() => {
						error = null;
						loading = true;
						loadExternalApi().then(() => initJitsi());
					}}
				>
					Retry
				</button>
			</div>
		</div>
	{/if}

	<div bind:this={containerEl} class="h-full w-full"></div>
</div>
