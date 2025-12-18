<script lang="ts">
	import type { LayoutProps } from './$types';
	import CollapsibleChat from '$lib/components/Chatbot/CollapsibleChat.svelte';

	let { data, children }: LayoutProps = $props();
	let { conversation } = data;
</script>

{@render children()}

<CollapsibleChat
    chatId="b20c102adc0111f09b4f22424bee5982"
    knowledgeBaseIds={["21372960d1dc11f095fbceda8b23386f"]}
 />
