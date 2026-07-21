<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import LiveRecorder from '$lib/components/recordings/LiveRecorder.svelte';
	import { Mic } from 'lucide-svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	let conversationId = $derived(data.conversationId);
	let eventId = $derived(data.eventId);
	let event = $derived(data.event);
	let recordings = $derived(data.recordings);
</script>

<svelte:head>
	<title>Record for {event.name}</title>
</svelte:head>

<div class="mx-auto flex w-full max-w-3xl flex-col gap-6 px-6 py-10">
	<div class="flex items-start gap-3">
		<div class="bg-destructive/10 flex h-10 w-10 items-center justify-center rounded-full">
			<Mic class="text-destructive h-5 w-5" />
		</div>
		<div>
			<h1 class="text-3xl font-bold">New live recording</h1>
			<p class="text-muted-foreground text-sm">
				Create a recording for {event.name}.
			</p>
		</div>
	</div>

	<LiveRecorder
		conversation_id={conversationId}
		event_id={eventId}
		{recordings}
		mode="participant"
		onComplete={invalidateAll}
	/>
</div>
