<script lang="ts">
	import NotificationForm from '$lib/components/NotificationForm.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import type { PageData } from './$types';

	let { data } = $props() as { data: PageData };
	let conversation = $derived(data.conversation);
</script>

<svelte:head>
	<title>Manage Notifications - Comhairle Admin</title>
</svelte:head>

<PageHeader
	title="Notify"
	description="Send notifications to all participants in this conversation. Notifications will be delivered to all users who have participated in workflows within this conversation."
/>

<NotificationForm conversationId={conversation.id} />
