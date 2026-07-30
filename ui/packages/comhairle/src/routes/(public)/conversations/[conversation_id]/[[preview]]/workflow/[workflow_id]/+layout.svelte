<script lang="ts">
	import ConversationSupportSidebar from '$lib/components/ConversationSupportSidebar.svelte';
	import type { LayoutProps } from './$types';

	let { data, children }: LayoutProps = $props();
	let { conversation, hasKnowledgeBaseDocs } = $derived(data);
</script>

{@render children()}

<ConversationSupportSidebar {conversation} {hasKnowledgeBaseDocs} />
