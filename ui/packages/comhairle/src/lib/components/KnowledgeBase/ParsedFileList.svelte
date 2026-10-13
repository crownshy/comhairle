<script lang="ts">
	import type { ComhairleDocument, ConversationWithTranslations } from '@crownshy/api-client/api';
	import ParsedFileBadge from './ParsedFileBadge.svelte';

	type Props = {
		documents: ComhairleDocument[];
		conversation: ConversationWithTranslations;
		editable?: boolean;
	};

	let { documents, conversation, editable = true }: Props = $props();
</script>

{#if conversation.knowledgeBaseId}
	<h2 class="mb-3 font-bold">Processed files</h2>
	<ul class="flex flex-col gap-3">
		{#each documents as document (document.id)}
			<li>
				<ParsedFileBadge {document} conversationId={conversation.id} {editable} />
			</li>
		{/each}
	</ul>
{/if}
