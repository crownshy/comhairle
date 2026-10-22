<script lang="ts">
	type Props = {
		/** RAGFlow widget URL in `mode=window`, which renders only the chat panel. */
		src: string;
		open: boolean;
		/** True once the chat has been opened, so the thread survives closing. */
		mounted: boolean;
	};

	let { src, open, mounted }: Props = $props();
</script>

{#if mounted}
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
