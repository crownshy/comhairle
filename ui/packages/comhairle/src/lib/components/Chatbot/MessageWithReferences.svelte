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
	function parseContentWithReferences(
		text: string
	): { type: 'text' | 'reference'; value: string }[] {
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
						<span
							class="text-chat-primary bg-chat-primary-lighter hover:bg-chat-primary-light mx-0.5 inline-flex h-4 w-4 cursor-pointer items-center justify-center rounded-full text-[10px] font-medium transition-colors"
						>
							<Info class="h-3 w-3" />
						</span>
					</HoverCard.Trigger>
					<HoverCard.Content
						class="bg-chat-bubble border-chat-border max-h-96 w-96 overflow-y-auto rounded-lg border p-0 shadow-lg"
						side="top"
						sideOffset={8}
					>
						<ReferencePopoverContent {chunk} />
					</HoverCard.Content>
				</HoverCard.Root>
			{:else}
				<span
					class="text-chat-text-muted bg-chat-bg mx-0.5 inline-flex h-4 w-4 items-center justify-center rounded-full text-[10px] font-medium"
				>
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
