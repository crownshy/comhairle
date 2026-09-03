<script lang="ts">
	import NotificationForm from '$lib/components/NotificationForm.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import TabContent from '../TabContent.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Manage Notifications - Comhairle Admin</title>
</svelte:head>

<TabContent>
	<PageHeader
		title="Notify"
		description="Send notifications to all participants in this conversation. Notifications will be delivered to all users who have participated in workflows within this conversation."
	/>

	<NotificationForm conversationId={data.conversation.id} />
</TabContent>
