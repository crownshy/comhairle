<script lang="ts">
	import { ChevronDown, ChevronUp } from 'lucide-svelte';
	import type { ReferenceChunk } from '$lib/api/chatClient.svelte';

	interface Props {
		chunk: ReferenceChunk;
	}

	let { chunk }: Props = $props();
	let isExpanded = $state(false);

	function stripHtml(text: string): string {
		return text.replace(/<[^>]*>/g, ' ').replace(/\s+/g, ' ').trim();
	}

	const strippedContent = $derived(stripHtml(chunk.content));
	const isTruncatable = $derived(strippedContent.length > 300);
	const displayContent = $derived(
		isExpanded || !isTruncatable 
			? strippedContent 
			: strippedContent.slice(0, 300) + '...'
	);
</script>

<div class="p-4">
	<!-- Document source header -->
	<div class="flex items-start gap-2 mb-3 pb-3 border-b border-cs-grey-100">
		<div class="flex-shrink-0 w-8 h-8 bg-cs-blue-100 rounded flex items-center justify-center">
			<svg class="w-4 h-4 text-cs-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
			</svg>
		</div>
		<div class="flex-1 min-w-0">
			<p class="text-sm font-medium text-cs-grey-900 truncate">
				{chunk.document_name}
			</p>
			{#if chunk.similarity}
				<p class="text-xs text-cs-grey-500">
					Relevance: {(chunk.similarity * 100).toFixed(1)}%
				</p>
			{/if}
		</div>
	</div>
	
	<!-- Content preview -->
	<div class="text-sm text-cs-grey-700 leading-relaxed">
		{displayContent}
	</div>

	<!-- See more/less button -->
	{#if isTruncatable}
		<button
			onclick={() => isExpanded = !isExpanded}
			class="mt-2 inline-flex items-center gap-1 text-xs font-medium text-cs-blue-600 hover:text-cs-blue-700 transition-colors"
		>
			{#if isExpanded}
				<ChevronUp class="w-3 h-3" />
				See less
			{:else}
				<ChevronDown class="w-3 h-3" />
				See more
			{/if}
		</button>
	{/if}

	<!-- Image indicator if applicable -->
	{#if chunk.doc_type?.includes('image')}
		<div class="mt-3 pt-3 border-t border-cs-grey-100">
			<span class="inline-flex items-center gap-1 text-xs text-cs-grey-500">
				<svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
				</svg>
				Contains image/table
			</span>
		</div>
	{/if}
</div>
