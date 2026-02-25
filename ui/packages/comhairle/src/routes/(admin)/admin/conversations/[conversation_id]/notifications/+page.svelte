<script lang="ts">
	import NotificationForm from '$lib/components/NotificationForm.svelte';
	import type { PageData } from './$types';
	import { BreadcrumbItem } from '$lib/components/ui/breadcrumb';
	import { useAdminLayoutSlots } from '../useAdminLayoutSlots.svelte';

	let { data } = $props() as { data: PageData };
	let conversation = $derived(data.conversation);

	useAdminLayoutSlots({
		title: titleSnippet,
		breadcrumbs: breadcrumbSnippet
	});
</script>

{#snippet titleSnippet()}
	<h1 class="text-4xl font-bold">Notifications</h1>
{/snippet}

{#snippet breadcrumbSnippet()}
	<BreadcrumbItem>Notifications</BreadcrumbItem>
{/snippet}

<p class="mb-10">
	Send notifications to all participants in this conversation. Notifications will be delivered to
	all users who have participated in workflows within this conversation.
</p>

<div class="max-w-2xl">
	<NotificationForm conversationId={conversation.id} />
</div>
