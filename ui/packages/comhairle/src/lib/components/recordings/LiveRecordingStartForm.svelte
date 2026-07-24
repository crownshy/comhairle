<script lang="ts">
	import { Mic } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';

	import { LiveRecorderController } from './liveRecorderController.svelte';

	type Props = {
		controller: LiveRecorderController;
	};

	let { controller }: Props = $props();
</script>

<div class="flex items-center gap-4" class:opacity-50={controller.hasActiveLiveRecording}>
	<div class="bg-muted flex h-10 w-10 shrink-0 items-center justify-center rounded-full">
		<Mic class="h-5 w-5" />
	</div>
	<Input
		class="max-w-xs"
		placeholder="Recording name"
		disabled={controller.hasActiveLiveRecording || !controller.canStartNewRecording}
		bind:value={controller.recordingName}
		onkeydown={(event) => {
			if (event.key === 'Enter') void controller.startRecording();
		}}
	/>
	<Button
		onclick={() => controller.startRecording()}
		disabled={!controller.canStartNewRecording || !controller.recordingName.trim()}
	>
		Start recording
	</Button>
</div>
