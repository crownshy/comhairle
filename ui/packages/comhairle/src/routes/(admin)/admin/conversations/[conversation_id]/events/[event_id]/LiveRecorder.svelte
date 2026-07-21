<script lang="ts">
	import SharedLiveRecorder from '$lib/components/recordings/LiveRecorder.svelte';
	import type { AudioRecordingDto } from '@crownshy/api-client/api';

	type Props = {
		conversation_id: string;
		event_id: string;
		recordings: AudioRecordingDto[];
		onComplete?: () => void | Promise<void>;
	};

	let { conversation_id, event_id, recordings, onComplete }: Props = $props();
</script>

<SharedLiveRecorder {conversation_id} {event_id} {recordings} {onComplete} mode="admin" />
