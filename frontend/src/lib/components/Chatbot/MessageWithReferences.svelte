<script lang="ts">
	import { Info } from 'lucide-svelte';
	import * as HoverCard from '$lib/components/ui/hover-card';
	import type { ReferenceChunk, ChatReference } from '$lib/api/chatClient.svelte';
	import ReferencePopoverContent from '$lib/components/Chatbot/ReferencePopoverContent.svelte';

	interface Props {
		content: string;
		reference?: ChatReference | null;
	}

	let { content, reference = null }: Props = $props();

	// Parse content and extract reference markers [ID:X]
	function parseContentWithReferences(text: string): { type: 'text' | 'reference'; value: string }[] {
		const pattern = /\[ID:(\d+)\]/g;
		const parts: { type: 'text' | 'reference'; value: string }[] = [];
		let lastIndex = 0;
		let match;

		while ((match = pattern.exec(text)) !== null) {
			// Add text before the match
			if (match.index > lastIndex) {
				parts.push({ type: 'text', value: text.slice(lastIndex, match.index) });
			}
			// Add the reference marker
			parts.push({ type: 'reference', value: match[1] });
			lastIndex = pattern.lastIndex;
		}

		// Add remaining text
		if (lastIndex < text.length) {
			parts.push({ type: 'text', value: text.slice(lastIndex) });
		}

		return parts;
	}

	function getChunkById(id: string): ReferenceChunk | undefined {
		if (!reference?.chunks) return undefined;
		const index = parseInt(id, 10);
		return reference.chunks[index];
	}

	const parsedContent = $derived(parseContentWithReferences(content));
</script>

<span class="message-with-refs">
	{#each parsedContent as part}
		{#if part.type === 'text'}
			{part.value}
		{:else}
			{@const chunk = getChunkById(part.value)}
			{#if chunk}
				<HoverCard.Root openDelay={200} closeDelay={100}>
					<HoverCard.Trigger class="inline-flex items-center">
						<span class="inline-flex items-center justify-center w-4 h-4 text-[10px] font-medium text-cs-blue-600 bg-cs-blue-100 rounded-full cursor-pointer hover:bg-cs-blue-200 transition-colors mx-0.5">
							<Info class="w-3 h-3" />
						</span>
					</HoverCard.Trigger>
					<HoverCard.Content 
						class="w-96 max-h-96 overflow-y-auto p-0 bg-white border border-cs-grey-200 shadow-lg rounded-lg"
						side="top"
						sideOffset={8}
					>
						<ReferencePopoverContent {chunk} />
					</HoverCard.Content>
				</HoverCard.Root>
			{:else}
				<span class="inline-flex items-center justify-center w-4 h-4 text-[10px] font-medium text-cs-grey-400 bg-cs-grey-100 rounded-full mx-0.5">
					{parseInt(part.value) + 1}
				</span>
			{/if}
		{/if}
	{/each}
</span>

<style>
	.message-with-refs {
		white-space: pre-wrap;
	}
</style>
