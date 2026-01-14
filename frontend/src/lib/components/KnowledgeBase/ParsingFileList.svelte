<script lang="ts">
	import type { ComhairleDocument, Conversation } from '$lib/api/api';
	import ParsingFileBadge from './ParsingFileBadge.svelte';

	type Props = {
		conversation: Conversation;
		documents: ComhairleDocument[];
	};

	let { documents, conversation }: Props = $props();
</script>

<h2 class="mb-3 font-bold">Uploading...</h2>
<ul class="flex flex-col gap-3">
	{#if conversation.knowledge_base_id}
		{#each documents as document (document.id)}
			<li>
				<ParsingFileBadge {document} knowledgeBaseId={conversation.knowledge_base_id} />
			</li>
		{/each}
	{/if}
</ul>
