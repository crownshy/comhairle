<script lang="ts">
	import type { LayoutProps } from './$types';
	import CollapsibleChat from '$lib/components/Chatbot/CollapsibleChat.svelte';

	let { data, children }: LayoutProps = $props();
	let { conversation, user } = data;
</script>

{@render children()}

{#if conversation?.chatBotId}
	<CollapsibleChat
		chatId={conversation.chatBotId}
		conversationId={conversation.id}
		userId={user?.id}
	/>
{/if}
