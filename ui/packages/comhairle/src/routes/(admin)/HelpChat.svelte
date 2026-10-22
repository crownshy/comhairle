<script lang="ts">
	import { MessageCircle, X } from 'lucide-svelte';

	/** RAGFlow widget URL in `mode=window`, which renders only the chat panel. */
	let { src }: { src: string } = $props();

	let open = $state(false);
	// Mount the chat on first open, then keep it so the thread survives closing.
	let opened = $state(false);

	function toggle() {
		open = !open;
		opened = true;
	}
</script>

{#if opened}
	<!-- color-scheme matches RAGFlow's light page. A mismatch with our dark mode makes the
	     browser paint an opaque white backdrop behind the iframe. -->
	<iframe
		{src}
		title="Help chat"
		allow="microphone;camera"
		class="fixed end-6 bottom-24 z-40 h-[500px] w-[380px] max-w-[calc(100vw-3rem)] border-none bg-transparent [color-scheme:light]"
		class:hidden={!open}
	></iframe>
{/if}

<button
	type="button"
	onclick={toggle}
	aria-label={open ? 'Close help chat' : 'Open help chat'}
	aria-expanded={open}
	class="bg-primary text-primary-foreground hover:bg-primary/90 focus-visible:ring-ring/50 inline-flex size-14 cursor-pointer items-center justify-center rounded-full shadow-lg transition-colors focus-visible:ring-[3px] focus-visible:outline-none"
>
	{#if open}
		<X class="size-6" aria-hidden="true" />
	{:else}
		<MessageCircle class="size-6" aria-hidden="true" />
	{/if}
</button>
