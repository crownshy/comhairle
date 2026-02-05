<script lang="ts">
	import type { ComhairleDocument, ConversationWithTranslations } from '$lib/api/api';
	import ParsingFileBadge from './ParsingFileBadge.svelte';

	type Props = {
		conversation: ConversationWithTranslations;
		documents: ComhairleDocument[];
	};

	let { documents, conversation }: Props = $props();
</script>

<h2 class="mb-3 font-bold">Uploading...</h2>
<ul class="flex flex-col gap-3">
	{#if conversation.knowledgeBaseId}
		{#each documents as document (document.id)}
			<li>
				<ParsingFileBadge {document} conversationId={conversation.id} />
			</li>
		{/each}
	{/if}
</ul>
