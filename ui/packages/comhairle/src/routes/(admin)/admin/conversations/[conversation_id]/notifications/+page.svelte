<script lang="ts">
	import NotificationForm from '$lib/components/NotificationForm.svelte';
	import type { PageData } from './$types';

	let { data } = $props() as { data: PageData };
	let conversation = $derived(data.conversation);
</script>

<svelte:head>
	<title>Manage Notifications - Comhairle Admin</title>
</svelte:head>

<h1 class="mb-4 text-3xl font-bold">Notify</h1>

<p class="mb-10">
	Send notifications to all participants in this conversation. Notifications will be delivered to
	all users who have participated in workflows within this conversation.
</p>

<NotificationForm conversationId={conversation.id} />
