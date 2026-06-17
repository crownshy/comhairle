<script lang="ts">
	import { getContext } from 'svelte';
	import EventStrip from '$lib/components/EventStrip.svelte';
	import {
		CONVERSATION_TAB_EXTRAS_CTX,
		type ConversationTabExtras
	} from '$lib/conversationTabExtras';

	let { data, children } = $props();

	let conversation = $derived(data.conversation);
	let events = $derived(data.events?.records ?? []);

	const tabExtras = getContext<ConversationTabExtras>(CONVERSATION_TAB_EXTRAS_CTX);

	$effect(() => {
		if (!tabExtras) return;
		tabExtras.primary = eventStripSnippet;
		return () => {
			tabExtras.primary = null;
		};
	});
</script>

{#snippet eventStripSnippet()}
	<EventStrip conversationId={conversation.id} {events} />
{/snippet}

{@render children()}
