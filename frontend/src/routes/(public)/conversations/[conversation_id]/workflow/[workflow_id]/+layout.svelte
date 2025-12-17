<script lang="ts">
	import type { LayoutProps } from './$types';
	import CollapsibleChat from '$lib/components/Chatbot/CollapsibleChat.svelte';

	let { data, children }: LayoutProps = $props();
	let { conversation } = data;
</script>

{@render children()}

<CollapsibleChat
    chatId="08e6c2dcda6d11f09b4f22424bee5982"
    knowledgeBaseIds={["21372960d1dc11f095fbceda8b23386f"]}
 />
