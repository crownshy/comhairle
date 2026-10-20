<script>
	import PageHeader from '$lib/components/PageHeader.svelte';
	import UserConversationPreferencesForm from '$lib/components/UserConversationPreferencesForm/UserConversationPreferencesForm.svelte';

	const { data } = $props();
</script>

<PageHeader
	title="Notifications"
	description="Manage how you would like to be contacted about updates on conversations"
/>
{#each data.participation as conversation (conversation.id)}
	<h2 class="text-2xl font-semibold">{conversation.title}</h2>
	<UserConversationPreferencesForm
		conversationId={conversation.id}
		isGuest={data.user.authType === 'guest'}
	/>
{/each}
