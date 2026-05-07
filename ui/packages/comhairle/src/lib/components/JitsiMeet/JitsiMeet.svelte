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
		onModeratorStatusChanged?: (isModerator: boolean) => void;
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
		onBreakoutRoomsUpdated,
		onModeratorStatusChanged
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

	function decodeJwt(token: string) {
		try {
			const parts = token.split('.');
			if (parts.length !== 3) return null;
			// Decode the payload (second part)
			const payload = parts[1];
			const decoded = atob(payload.replace(/-/g, '+').replace(/_/g, '/'));
			return JSON.parse(decoded);
		} catch (e) {
			console.error('[JITSI] Failed to decode JWT:', e);
			return null;
		}
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

		// Decode and log JWT claims
		if (jwt) {
			const claims = decodeJwt(jwt);
			console.log('[JITSI] JWT Claims:', claims);
			console.log('[JITSI] Moderator status in JWT:', claims?.context?.user?.moderator);
		}

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
				SHOW_BRAND_WATERMARK: false,
				HIDE_DEEP_LINKING_LOGO: true,
				JITSI_WATERMARK_LINK: '',
				DEFAULT_LOGO_URL: '',
				DEFAULT_WELCOME_PAGE_LOGO_URL: '',
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

			let localParticipantId: string | null = null;

			api.addListener('videoConferenceJoined', (data: any) => {
				console.log('[JITSI] ===== JOINED CONFERENCE =====');
				console.log('[JITSI] videoConferenceJoined:', data);
				console.log('[JITSI] Room name:', data.roomName);
				console.log('[JITSI] Display name:', data.displayName);
				console.log('[JITSI] Participant ID:', data.id);

				// Store local participant ID
				localParticipantId = data.id;

				// Log the user's role/moderator status
				const isModerator = api.isModeratorEnabled?.() ?? false;
				const participantInfo = api.getParticipantsInfo?.();
				console.log('[JITSI] Current user role:', {
					isModerator,
					displayName: data.displayName,
					participantId: data.id,
					participantInfo
				});
				console.log('[JITSI] ==============================');

				// Notify parent of moderator status
				console.log('[JITSI] Setting initial moderator status:', isModerator);
				onModeratorStatusChanged?.(isModerator);

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
				console.log('[JITSI] ===== LEFT CONFERENCE =====');
				console.log('[JITSI] videoConferenceLeft:', data);
				console.log('[JITSI] Room name:', data.roomName);
				console.log('[JITSI] ==========================');
				onVideoConferenceLeft?.(data);
			});

			api.addListener('breakoutRoomsUpdated', (data: any) => {
				console.log('[JITSI] breakoutRoomsUpdated event received');
				console.log('[JITSI] Data structure:', data);
				const rooms = data?.rooms || data || {};
				console.log('[JITSI] Parsed rooms:', rooms);
				console.log('[JITSI] Room count:', Object.keys(rooms).length);
				console.log(
					'[JITSI] Room details:',
					Object.values(rooms).map((r: any) => ({
						id: r.id,
						jid: r.jid,
						name: r.name,
						isMainRoom: r.isMainRoom
					}))
				);
				onBreakoutRoomsUpdated?.(rooms);
			});

			api.addListener('participantRoleChanged', (data: any) => {
				const isModeratorFromRole = data.role === 'moderator';
				const isModeratorFromAPI = api.isModeratorEnabled?.() ?? false;
				console.log('[JITSI] participantRoleChanged:', {
					participantId: data.id,
					role: data.role,
					isLocalUser: data.id === localParticipantId,
					localParticipantId,
					isModeratorFromRole,
					isModeratorFromAPI
				});

				// Only update moderator status if this is the local user's role changing
				if (data.id === localParticipantId) {
					console.log('[JITSI] Local user role changed to:', data.role);
					// Use the role from the event, not isModeratorEnabled()
					onModeratorStatusChanged?.(isModeratorFromRole);
				}
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
