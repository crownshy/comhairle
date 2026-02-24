<script lang="ts">
	import type { ComhairleDocument, ConversationWithTranslations } from '$lib/api/api';
	import ParsedFileBadge from './ParsedFileBadge.svelte';

	type Props = {
		documents: ComhairleDocument[];
		conversation: ConversationWithTranslations;
	};

	let { documents, conversation }: Props = $props();
</script>

{#if conversation.knowledgeBaseId}
	<h2 class="mb-3 font-bold">Uploaded files</h2>
	<ul class="flex flex-col gap-3">
		{#each documents as document (document.id)}
			<li>
				<ParsedFileBadge {document} conversationId={conversation.id} />
			</li>
		{/each}
	</ul>
{/if}
