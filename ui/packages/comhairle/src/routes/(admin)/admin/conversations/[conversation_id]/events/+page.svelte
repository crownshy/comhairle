<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import { Plus } from 'lucide-svelte';
	import * as Card from '$lib/components/ui/card';
	import EventCard from '$lib/components/EventCard.svelte';
	import { permissions } from '$lib/permissions.svelte';
	import { useTour } from '$lib/tours/useTour.svelte';
	import { eventsListTour } from './tour';
	import TabContent from '$lib/components/TabContent.svelte';
	import ErrorMessage from '$lib/components/ErrorMessage.svelte';
	import LoadingMessage from '$lib/components/LoadingMessage.svelte';
	import { resolve } from '$app/paths';

	useTour(() => eventsListTour);

	let { data } = $props();

	const canEdit = $derived(
		permissions.can('conversation', 'conversation_update', data.conversation.id)
	);
</script>

<svelte:head>
	<title>Manage Events - {data.conversation.title} - Comhairle Admin</title>
</svelte:head>

{#snippet AddButton()}
	{#if canEdit}
		<Button
			href={resolve(`/(admin)/admin/conversations/[conversation_id]/events/new`, {
				conversation_id: data.conversation.id
			})}
		>
			<Plus class="h-4 w-4" /> Add event
		</Button>
	{/if}
{/snippet}

<TabContent>
	<div data-tour="events-list" class="mx-auto flex max-w-175 flex-col gap-6">
		{#await data.streamedEvents}
			<LoadingMessage items="events" />
		{:then events}
			{#if events.err !== null}
				<ErrorMessage>Could not load events, please try again</ErrorMessage>
			{:else}
				{#each events.ok as event (event.id)}
					<Card.Root class="overflow-hidden rounded-3xl pb-0 shadow-sm">
						<EventCard
							{event}
							conversationId={data.conversation.id}
							editable={canEdit}
						/>
					</Card.Root>
				{:else}
					<div
						class="border border-border bg-card text-muted-foreground flex flex-col items-center justify-center gap-y-4 rounded-xl border-dashed p-12"
					>
						No events yet.
						{@render AddButton()}
					</div>
				{/each}
				{#if events.ok.length > 0}
					<div class="self-center">
						{@render AddButton()}
					</div>
				{/if}
			{/if}
		{/await}
	</div>
</TabContent>
