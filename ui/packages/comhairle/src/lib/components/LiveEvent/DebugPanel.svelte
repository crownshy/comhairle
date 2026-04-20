<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import { X } from 'lucide-svelte';

	interface Props {
		jitsiApi: any;
		audioMuted: boolean;
		videoMuted: boolean;
		participantCount: number;
		conferenceJoined: boolean;
		onAutoBreakout?: (maxPerRoom: number) => void;
		onCloseBreakout?: () => void;
		onNotify?: (msg: string) => void;
		onClose: () => void;
	}

	let {
		jitsiApi,
		audioMuted,
		videoMuted,
		participantCount,
		conferenceJoined,
		onAutoBreakout,
		onCloseBreakout,
		onNotify,
		onClose
	}: Props = $props();

	let announcementText = $state('');
</script>

<div class="flex h-full flex-col">
	<div class="border-border flex items-center justify-between border-b px-4 py-3">
		<h2 class="text-lg font-semibold">Debug</h2>
		<button
			class="text-muted-foreground hover:text-foreground rounded-md p-1"
			onclick={onClose}
		>
			<X class="h-4 w-4" />
		</button>
	</div>

	<div class="flex-1 space-y-4 overflow-y-auto p-4">
		<!-- Status -->
		<div class="space-y-1">
			<p class="text-muted-foreground text-xs font-medium tracking-wide uppercase">Status</p>
			<div class="grid grid-cols-2 gap-1.5 text-xs">
				<span class="text-muted-foreground">Conference</span>
				<span class="font-medium">{conferenceJoined ? 'Joined' : 'Not joined'}</span>
				<span class="text-muted-foreground">Audio</span>
				<span class="font-medium">{audioMuted ? 'Muted' : 'On'}</span>
				<span class="text-muted-foreground">Video</span>
				<span class="font-medium">{videoMuted ? 'Off' : 'On'}</span>
				<span class="text-muted-foreground">Participants</span>
				<span class="font-medium">{participantCount}</span>
				<span class="text-muted-foreground">Jitsi API</span>
				<span class="font-medium">{jitsiApi ? 'Ready' : 'Not loaded'}</span>
			</div>
		</div>

		<hr class="border-border" />

		<!-- Meeting Controls -->
		<div class="space-y-2">
			<p class="text-muted-foreground text-xs font-medium tracking-wide uppercase">
				Meeting Controls
			</p>
			<div class="grid grid-cols-2 gap-2">
				<Button
					variant={audioMuted ? 'default' : 'outline'}
					size="sm"
					class="w-full text-xs"
					onclick={() => jitsiApi?.executeCommand('toggleAudio')}
				>
					{audioMuted ? 'Unmute' : 'Mute'}
				</Button>
				<Button
					variant={videoMuted ? 'default' : 'outline'}
					size="sm"
					class="w-full text-xs"
					onclick={() => jitsiApi?.executeCommand('toggleVideo')}
				>
					{videoMuted ? 'Video On' : 'Video Off'}
				</Button>
				<Button
					variant="outline"
					size="sm"
					class="w-full text-xs"
					onclick={() => jitsiApi?.executeCommand('toggleShareScreen')}
				>
					Share Screen
				</Button>
				<Button
					variant="outline"
					size="sm"
					class="w-full text-xs"
					onclick={() => jitsiApi?.executeCommand('toggleTileView')}
				>
					Tile View
				</Button>
				<Button
					variant="outline"
					size="sm"
					class="w-full text-xs"
					onclick={() => jitsiApi?.executeCommand('toggleRaiseHand')}
				>
					Raise Hand
				</Button>
				<Button
					variant="outline"
					size="sm"
					class="w-full text-xs"
					onclick={() => jitsiApi?.executeCommand('muteEveryone')}
				>
					Mute All
				</Button>
			</div>
		</div>

		<hr class="border-border" />

		<!-- Announcement -->
		<div class="space-y-2">
			<p class="text-muted-foreground text-xs font-medium tracking-wide uppercase">
				Announcement
			</p>
			<div class="flex gap-2">
				<input
					type="text"
					placeholder="Message for all..."
					bind:value={announcementText}
					onkeydown={(e) => {
						if (e.key === 'Enter' && announcementText.trim()) {
							onNotify?.(announcementText.trim());
							announcementText = '';
						}
					}}
					class="border-border bg-background focus:ring-primary flex-1 rounded-lg border px-2.5 py-1.5 text-xs focus:ring-1 focus:outline-none"
				/>
				<Button
					variant="default"
					size="sm"
					class="shrink-0 text-xs"
					disabled={!announcementText.trim()}
					onclick={() => {
						onNotify?.(announcementText.trim());
						announcementText = '';
					}}
				>
					Send
				</Button>
			</div>
		</div>

		<hr class="border-border" />

		<!-- Breakout Rooms (raw) -->
		<div class="space-y-2">
			<p class="text-muted-foreground text-xs font-medium tracking-wide uppercase">
				Breakout Rooms (Jitsi)
			</p>
			<div class="grid grid-cols-2 gap-2">
				<Button
					variant="default"
					size="sm"
					class="w-full text-xs"
					onclick={() => onAutoBreakout?.(6)}
				>
					Auto-assign (6/room)
				</Button>
				<Button
					variant="outline"
					size="sm"
					class="w-full text-xs"
					onclick={() => onCloseBreakout?.()}
				>
					Close Breakouts
				</Button>
				<Button
					variant="outline"
					size="sm"
					class="w-full text-xs"
					onclick={async () => {
						if (!jitsiApi) return;
						const rooms = await jitsiApi.getRoomsInfo();
						console.log('Breakout rooms:', rooms);
						alert(JSON.stringify(rooms, null, 2));
					}}
				>
					Inspect Rooms
				</Button>
				<Button
					variant="outline"
					size="sm"
					class="w-full text-xs"
					onclick={() => jitsiApi?.executeCommand('toggleLobby', true)}
				>
					Toggle Lobby
				</Button>
			</div>
		</div>

		<hr class="border-border" />

		<!-- API Explorer -->
		<div class="space-y-2">
			<p class="text-muted-foreground text-xs font-medium tracking-wide uppercase">
				API Explorer
			</p>
			<div class="grid grid-cols-2 gap-2">
				<Button
					variant="outline"
					size="sm"
					class="w-full text-xs"
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
					class="w-full text-xs"
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
					class="w-full text-xs"
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
					class="w-full text-xs"
					onclick={async () => {
						if (!jitsiApi) return;
						const info = await jitsiApi.getParticipantsInfo();
						console.log('Participants info:', info);
						alert(JSON.stringify(info, null, 2));
					}}
				>
					Participants
				</Button>
			</div>
		</div>
	</div>
</div>
