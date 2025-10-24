<script lang="ts">
	import { Bell } from 'lucide-svelte';
	import NotificationForm from '$lib/components/NotificationForm.svelte';
	import type { PageData } from './$types';
	
	let { data } = $props() as { data: PageData };
	let conversation = $derived(data.conversation);
</script>

<h1 class="mb-10 flex flex-row items-center gap-2 text-4xl"><Bell /> Send Notifications</h1>
<p class="mb-10">Send notifications to all participants in this conversation. Notifications will be delivered to all users who have participated in workflows within this conversation.</p>

<div class="max-w-2xl">
	<NotificationForm conversationId={conversation.id} />
</div>