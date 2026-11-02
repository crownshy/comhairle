<script lang="ts">
	import NotificationForm from '$lib/components/NotificationForm.svelte';
	import type { PageData } from './$types';
	import { permissions } from '$lib/permissions.svelte';
	import { useTour } from '$lib/tours/useTour.svelte';
	import { notificationsTour } from './tour';

	useTour(() => notificationsTour);

	let { data } = $props() as { data: PageData };
	let conversation = $derived(data.conversation);
	const canAdmin = $derived(
		permissions.can('conversation', 'conversation_admin', conversation.id)
	);
</script>

<svelte:head>
	<title>Manage Notifications - Comhairle Admin</title>
</svelte:head>

{#if canAdmin}
	<NotificationForm conversationId={conversation.id} />
{/if}
