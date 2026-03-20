<script lang="ts">
	import ConversationSupportSidebar from '$lib/components/ConversationSupportSidebar.svelte';
	import type { LayoutProps } from './$types';

	let { data, children }: LayoutProps = $props();
	let { conversation, user } = data;
</script>

{@render children()}

<ConversationSupportSidebar {conversation} />
