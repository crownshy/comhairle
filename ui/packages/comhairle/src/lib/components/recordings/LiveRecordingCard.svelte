<script lang="ts">
	import { Loader2, Mic, Pause, Play, Square, Trash2 } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';

	import { LiveRecorderController } from './liveRecorderController.svelte';
	import { type LiveAudioRecordingDto } from './liveRecorderShared';

	type Props = {
		controller: LiveRecorderController;
		liveRecording: LiveAudioRecordingDto;
	};

	let { controller, liveRecording }: Props = $props();

	const isActiveRow = $derived(controller.activeLiveRecordingId === liveRecording.id);
	const uploadedBytes = $derived(controller.liveRecordingUploadedBytes(liveRecording));

	function formatSize(bytes: number): string {
		if (bytes < 1024) return `${bytes} B`;
		if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
		return `${(bytes / (1024 * 1024)).toFixed(2)} MB`;
	}
</script>

<div class="border-border rounded-lg border p-3">
	<div class="mb-2 text-sm font-semibold">Live recording</div>
	<div
		class="bg-muted/40 flex items-center justify-between gap-3 rounded-md p-2"
		class:opacity-50={controller.hasActiveLiveRecording && !isActiveRow}
	>
		<div class="min-w-0">
			<div class="truncate text-sm font-medium">
				{controller.liveRecordingName(liveRecording)}
			</div>
		</div>
		<div class="flex items-center gap-2">
			{#if isActiveRow}
				{#if controller.phase === 'starting'}
					<div class="text-muted-foreground flex items-center gap-2 text-sm">
						<Loader2 class="h-4 w-4 animate-spin" />
						Starting…
					</div>
				{:else if controller.phase === 'recording'}
					<div
						class="bg-destructive/10 flex h-8 w-8 items-center justify-center rounded-full transition-transform duration-75"
						style="transform: scale({1 + controller.audioVolume * 0.35})"
						aria-hidden="true"
					>
						<Mic class="text-destructive h-4 w-4" />
					</div>
					<Button variant="outline" size="sm" onclick={() => controller.pauseRecording()}>
						<Pause class="mr-2 h-4 w-4" />
						Pause
					</Button>
					<Button
						variant="destructive"
						size="sm"
						onclick={() => controller.stopRecording()}
					>
						<Square class="mr-2 h-4 w-4" />
						Stop
					</Button>
				{:else if controller.phase === 'stopping'}
					<div class="text-muted-foreground flex items-center gap-2 text-sm">
						<Loader2 class="h-4 w-4 animate-spin" />
						Saving…
					</div>
				{/if}
			{:else}
				<Button
					variant="outline"
					size="sm"
					disabled={controller.hasActiveLiveRecording ||
						controller.finalisingLiveRecordingId !== null}
					onclick={() => controller.resumeLiveRecording(liveRecording.id)}
				>
					<Play class="mr-2 h-4 w-4" />
					Resume
				</Button>
				{#if controller.isParticipantMode}
					<Button
						variant="destructive"
						size="sm"
						disabled={controller.hasActiveLiveRecording ||
							controller.finalisingLiveRecordingId !== null}
						onclick={() => controller.finaliseExistingRecording(liveRecording.id)}
					>
						<Square class="mr-2 h-4 w-4" />
						{controller.finalisingLiveRecordingId === liveRecording.id
							? 'Saving…'
							: 'Stop'}
					</Button>
					<Button
						variant="ghost"
						size="sm"
						disabled={controller.hasActiveLiveRecording ||
							controller.finalisingLiveRecordingId !== null}
						onclick={() => controller.discardLiveRecording(liveRecording.id)}
					>
						<Trash2 class="h-4 w-4" />
						<span class="sr-only">Delete recording</span>
					</Button>
				{:else}
					<Button
						variant="ghost"
						size="sm"
						disabled={controller.hasActiveLiveRecording ||
							controller.finalisingLiveRecordingId !== null}
						onclick={() => controller.discardLiveRecording(liveRecording.id)}
					>
						<Trash2 class="h-4 w-4" />
					</Button>
				{/if}
			{/if}
		</div>
	</div>
</div>
