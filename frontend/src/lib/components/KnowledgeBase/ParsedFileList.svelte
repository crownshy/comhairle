<script lang="ts">
	import type { ComhairleDocument, Conversation } from '$lib/api/api';
	import ParsedFileBadge from './ParsedFileBadge.svelte';

	type Props = {
		documents: ComhairleDocument[];
		conversation: Conversation;
	};

	let { documents, conversation }: Props = $props();
</script>

{#if conversation.knowledge_base_id}
	<h2 class="mb-3 font-bold">Uploaded files</h2>
	<ul class="flex flex-col gap-3">
		{#each documents as document (document.id)}
			<li>
				<ParsedFileBadge {document} knowledgeBaseId={conversation.knowledge_base_id} />
			</li>
		{/each}
	</ul>
{/if}
