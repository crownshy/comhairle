<script lang="ts">
	import type { LayoutProps } from './$types';
	import CollapsibleChat from '$lib/components/Chatbot/CollapsibleChat.svelte';

	let { data, children }: LayoutProps = $props();
	let { conversation } = data;
</script>

{@render children()}

<CollapsibleChat chatId={conversation?.chat_id} />
