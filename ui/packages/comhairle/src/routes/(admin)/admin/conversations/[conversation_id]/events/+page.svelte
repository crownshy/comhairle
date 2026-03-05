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

<p class="mb-10">Use this space to manage your conversation's events.</p>

<div class="grid w-full grid-cols-1 gap-x-2 gap-y-6 overflow-y-auto mb-8">
	{#each events as event (event.id)}
		<Card.Root class="transition-all px-6">
			<EventCard {event} conversationId={conversation.id} />
		</Card.Root>
	{/each}
</div>

<Button variant="secondary" href={`/admin/conversations/${conversation.id}/events/new`}>
	<Plus /> Add Event
</Button>
