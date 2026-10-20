<script>
	import ErrorMessage from '$lib/components/ErrorMessage.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import { Spinner } from '$lib/components/ui/spinner';
	import UserConversationPreferencesForm from '$lib/components/UserConversationPreferencesForm/UserConversationPreferencesForm.svelte';

	const { data } = $props();
</script>

<PageHeader
	title="Notifications"
	description="Manage how you would like to be contacted about updates on conversations"
/>

{#await data.streamedParticipation}
	<div class="text-l flex items-center gap-2">
		<Spinner />Loading conversations
	</div>
{:then conversations}
	{#if conversations.err !== null}
		{console.error(conversations.err)}
		<ErrorMessage>Could not load conversations please try again</ErrorMessage>
	{:else}
		{#each conversations.ok as conversation (conversation.id)}
			<h2 class="text-2xl font-semibold">{conversation.title}</h2>
			<UserConversationPreferencesForm
				conversationId={conversation.id}
				isGuest={data.user.authType === 'guest'}
			/>
		{:else}
			<span class="text-muted-foreground italic">No conversations to show</span>
		{/each}
	{/if}
{/await}
