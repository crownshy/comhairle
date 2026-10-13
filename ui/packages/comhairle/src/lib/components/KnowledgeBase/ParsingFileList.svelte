<script lang="ts">
	import type { ComhairleDocument, ConversationWithTranslations } from '@crownshy/api-client/api';
	import ParsingFileBadge from './ParsingFileBadge.svelte';

	type Props = {
		conversation: ConversationWithTranslations;
		documents: ComhairleDocument[];
		editable?: boolean;
	};

	let { documents, conversation, editable = true }: Props = $props();
</script>

<h2 class="mb-3 font-bold">Processing...</h2>
<ul class="flex flex-col gap-3">
	{#if conversation.knowledgeBaseId}
		{#each documents as document (document.id)}
			<li>
				<ParsingFileBadge {document} conversationId={conversation.id} {editable} />
			</li>
		{/each}
	{/if}
</ul>
