<script lang="ts">
	import TabStripItem from '$lib/components/TabStripItem.svelte';
	import TabStripShell from '$lib/components/TabStripShell.svelte';
	import { resolve } from '$app/paths';
	import { snakeToSentenceCase } from '$lib/utils/casingUtils';
	import { TabContent } from './external';
	import { Button } from '$lib/components/ui/button';

	const { data, children, params } = $props();
	const { conversation, event } = $derived(data);

	const tabs = [
		'details',
		'structure',
		'facilitators',
		'location',
		'invites',
		'breakout',
		'recordings'
	] as const;

	function labelResolver(tab: (typeof tabs)[number]): string {
		switch (tab) {
			case 'structure':
				return 'Event Structure';
			case 'breakout':
				return 'Breakout Rooms';
			case 'details':
			case 'facilitators':
			case 'location':
			case 'invites':
			case 'recordings':
				return snakeToSentenceCase(tab);
		}
	}
</script>

<TabStripShell>
	{#each tabs as tab (tab)}
		<TabStripItem
			{tab}
			href={resolve(
				`/(admin)/admin/conversations/[conversation_id]/events/[event_id]/${tab}`,
				{ conversation_id: params.conversation_id, event_id: params.event_id }
			)}
			label={labelResolver(tab)}
		/>
	{/each}
</TabStripShell>

<TabContent>
	<div class="mb-6 flex flex-row items-center gap-4">
		<h1 class="text-3xl font-bold">Event: {event?.name}</h1>
		{#if conversation && event}
			<Button href={`/conversations/${conversation.id}/events/${event.id}`}>Event Link</Button
			>
		{/if}
	</div>
	{@render children()}
</TabContent>
