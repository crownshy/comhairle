<script lang="ts">
	import type { LayoutProps } from './$types';
	import CollapsibleChat from '$lib/components/Chatbot/CollapsibleChat.svelte';

	let { data, children }: LayoutProps = $props();
	let { conversation, user } = data;
</script>

{@render children()}

{#if conversation?.chat_bot_id}
	<CollapsibleChat
		chatId={conversation.chat_bot_id}
		conversationId={conversation.id}
		userId={user?.id}
	/>
{/if}
