<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import { Plus } from 'lucide-svelte';
	import * as Card from '$lib/components/ui/card';
	import EventCard from '$lib/components/EventCard.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';

	let { data } = $props();
	let conversation = $derived(data.conversation);
	let events = $derived(data.events.records);

	let pageTitle = $derived(`Manage Events - ${conversation.title}`);
</script>

<svelte:head>
	<title>{pageTitle} - Comhairle Admin</title>
</svelte:head>

<PageHeader title="Events" description="Use this space to manage your conversation's events." />

<div class="flex max-w-[700px] flex-col gap-6">
	{#if events.length === 0}
		<div
			class="border-border bg-card text-muted-foreground flex flex-col items-center justify-center gap-3 rounded-xl border border-dashed p-12 text-center"
		>
			<p class="text-base">No events yet. Add your first event to get started.</p>
			<Button variant="default" href={`/admin/conversations/${conversation.id}/events/new`}>
				<Plus class="h-4 w-4" /> Add event
			</Button>
		</div>
	{:else}
		{#each events as event (event.id)}
			<Card.Root class="overflow-hidden rounded-3xl pb-0 shadow-sm">
				<EventCard {event} conversationId={conversation.id} />
			</Card.Root>
		{/each}

		<div class="flex justify-center pt-6">
			<Button variant="default" href={`/admin/conversations/${conversation.id}/events/new`}>
				<Plus class="h-4 w-4" /> Add event
			</Button>
		</div>
	{/if}
</div>
