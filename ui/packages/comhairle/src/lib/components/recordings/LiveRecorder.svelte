<script lang="ts">
	import { onDestroy, onMount } from 'svelte';

	import LiveRecordingCard from './LiveRecordingCard.svelte';
	import LiveRecordingStartForm from './LiveRecordingStartForm.svelte';
	import { LiveRecorderController } from './liveRecorderController.svelte';
	import type { LiveRecorderProps } from './liveRecorderShared';

	let {
		conversation_id,
		event_id,
		recordings,
		onComplete,
		mode = 'admin'
	}: LiveRecorderProps = $props();

	let controller = $state<LiveRecorderController | null>(null);

	function currentContext() {
		return {
			conversationId: conversation_id,
			eventId: event_id,
			recordings,
			onComplete,
			mode
		};
	}

	$effect(() => {
		controller?.updateContext(currentContext());
	});

	onMount(() => {
		controller = new LiveRecorderController(currentContext());
		void controller.loadLiveRecordings();
	});

	onDestroy(() => {
		controller?.destroy();
	});
</script>

<div class="flex flex-col gap-4">
	{#if controller?.currentLiveRecording}
		<LiveRecordingCard {controller} liveRecording={controller.currentLiveRecording} />
	{/if}

	{#if controller && controller.isParticipantMode && controller.liveRecordings.length > 0 && !controller.hasActiveLiveRecording}
		<p class="text-muted-foreground text-sm">
			Resume or finalise your recording before starting a new one.
		</p>
	{/if}

	{#if controller && controller.liveRecordings.length === 0}
		<LiveRecordingStartForm {controller} />
	{/if}
</div>
