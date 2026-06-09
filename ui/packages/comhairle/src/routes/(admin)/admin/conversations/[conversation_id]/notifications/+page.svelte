<script lang="ts">
	import NotificationForm from '$lib/components/NotificationForm.svelte';
	import type { PageData } from './$types';
	import { useAdminLayoutSlots } from '../useAdminLayoutSlots.svelte';

	let { data } = $props() as { data: PageData };
	let conversation = $derived(data.conversation);

	useAdminLayoutSlots({
		title: titleSnippet,
		breadcrumbs: [{ label: 'Notifications' }]
	});
</script>

<svelte:head>
	<title>Manage Notifications - Comhairle Admin</title>
</svelte:head>

{#snippet titleSnippet()}
	<h1 class="text-4xl font-bold">Notifications</h1>
{/snippet}

<p class="mb-10">
	Send notifications to all participants in this conversation. Notifications will be delivered to
	all users who have participated in workflows within this conversation.
</p>

<NotificationForm conversationId={conversation.id} />
