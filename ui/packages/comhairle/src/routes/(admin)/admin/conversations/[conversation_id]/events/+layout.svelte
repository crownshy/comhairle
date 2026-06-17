<script lang="ts">
	import EventStrip from '$lib/components/EventStrip.svelte';

	let { data, children } = $props();

	let conversation = $derived(data.conversation);
	let events = $derived(data.events?.records ?? []);
</script>

<div class="-mx-4 -mt-8 sm:-mx-8 sm:-mt-10 lg:-mx-16">
	<EventStrip conversationId={conversation.id} {events} />
</div>

{@render children()}
