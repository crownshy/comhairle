<script lang="ts">
	import AdminPrevNextControls from '$lib/components/AdminPrevNextControls.svelte';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb';
	import Button from '$lib/components/ui/button/button.svelte';
	import { Plus } from 'lucide-svelte';
	import { useAdminLayoutSlots } from '../useAdminLayoutSlots.svelte';
	import * as Card from '$lib/components/ui/card';
	import EventCard from '$lib/components/EventCard.svelte';

	let { data } = $props();
	let conversation = $derived(data.conversation);
	let events = $derived(data.events.records);

	useAdminLayoutSlots({
		title: titleSnippet,
		breadcrumbs: breadcrumbSnippet
	});
	let pageTitle = $derived(`Manage Events - ${conversation.title}`);
</script>

<svelte:head>
	<title>{pageTitle} - Comhairle Admin</title>
</svelte:head>

{#snippet titleSnippet()}
	<h1 class="text-4xl font-bold">Events</h1>
	<AdminPrevNextControls
		prev={{
			name: 'Knowledge base',
			url: `/admin/conversations/${conversation.id}/knowledge-base`
		}}
	/>
{/snippet}

{#snippet breadcrumbSnippet()}
	<Breadcrumb.Item>Events</Breadcrumb.Item>
{/snippet}

<p class="text-muted-foreground mb-10 text-base font-medium">
	Use this space to manage your conversation's events.
</p>

<div class="mx-auto flex max-w-[700px] flex-col gap-6">
	{#each events as event (event.id)}
		<Card.Root class="overflow-hidden rounded-3xl shadow-sm">
			<EventCard {event} conversationId={conversation.id} />
		</Card.Root>
	{/each}

	<div class="flex justify-center pt-6">
		<Button variant="default" href={`/admin/conversations/${conversation.id}/events/new`}>
			<Plus class="h-4 w-4" /> Add event
		</Button>
	</div>
</div>
