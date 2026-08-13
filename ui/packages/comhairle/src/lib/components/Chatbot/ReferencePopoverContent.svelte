<script lang="ts">
	import { ChevronDown, ChevronUp, FileText } from 'lucide-svelte';
	import type { ReferenceChunk } from '$lib/api/chatClient.svelte';

	interface Props {
		chunk: ReferenceChunk;
		/**
		 * When provided, the footer action opens this chunk in the document viewer
		 * instead of expanding the (raw, often messy) extracted text inline.
		 */
		onOpenSource?: (chunk: ReferenceChunk) => void;
	}

	let { chunk, onOpenSource }: Props = $props();
	let isExpanded = $state(false);

	function stripHtml(text: string): string {
		return text
			.replace(/<[^>]*>/g, ' ')
			.replace(/\s+/g, ' ')
			.trim();
	}

	const strippedContent = $derived(stripHtml(chunk.content));
	const isTruncatable = $derived(strippedContent.length > 300);
	// With a viewer available we always keep the preview short; "Open in document"
	// is the way to read the full passage, so the inline expand is only a fallback.
	const displayContent = $derived(
		(isExpanded && !onOpenSource) || !isTruncatable
			? strippedContent
			: strippedContent.slice(0, 300) + '...'
	);
</script>

<div class="p-4">
	<!-- Document source header -->
	<div class="border-chat-border mb-3 flex items-start gap-2 border-b pb-3">
		<div
			class="bg-chat-primary-lighter flex h-8 w-8 shrink-0 items-center justify-center rounded"
		>
			<svg
				class="text-chat-primary h-4 w-4"
				fill="none"
				stroke="currentColor"
				viewBox="0 0 24 24"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
				/>
			</svg>
		</div>
		<div class="flex min-w-0 flex-1 items-center justify-center">
			<p class="text-chat-text truncate text-sm font-medium">
				{chunk.document_name}
			</p>
		</div>
	</div>

	<!-- Content preview -->
	<div class="text-chat-neutral text-sm leading-relaxed">
		{displayContent}
	</div>

	<!-- Footer action: open the real document, or (fallback) expand the text inline. -->
	{#if onOpenSource}
		<button
			type="button"
			onclick={() => onOpenSource?.(chunk)}
			class="text-chat-primary hover:text-chat-primary-dark mt-2 inline-flex items-center gap-1 text-xs font-medium transition-colors"
		>
			<FileText class="h-3 w-3" />
			Open in document
		</button>
	{:else if isTruncatable}
		<button
			type="button"
			onclick={() => (isExpanded = !isExpanded)}
			class="text-chat-primary hover:text-chat-primary-dark mt-2 inline-flex items-center gap-1 text-xs font-medium transition-colors"
		>
			{#if isExpanded}
				<ChevronUp class="h-3 w-3" />
				See less
			{:else}
				<ChevronDown class="h-3 w-3" />
				See more
			{/if}
		</button>
	{/if}

	<!-- Image indicator if applicable -->
	{#if chunk.doc_type?.includes('image')}
		<div class="border-chat-border mt-3 border-t pt-3">
			<span class="text-chat-text-muted inline-flex items-center gap-1 text-xs">
				<svg class="h-3 w-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
					/>
				</svg>
				Contains image/table
			</span>
		</div>
	{/if}
</div>
