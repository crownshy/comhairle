<script lang="ts">
	import type { ComhairleDocument, Conversation } from '$lib/api/api.js';
	import FileUpload from '$lib/components/KnowledgeBase/FileUpload.svelte';
	import ParsedFileList from '$lib/components/KnowledgeBase/ParsedFileList.svelte';
	import ParsingFileList from '$lib/components/KnowledgeBase/ParsingFileList.svelte';
	import { Database } from 'lucide-svelte';

	type Props = {
		data: {
			documents: ComhairleDocument[];
			conversation: Conversation;
		};
	};

	let { data }: Props = $props();
	let conversation = $derived(data.conversation);
	let documents = $derived(data.documents);

	const parsingDocuments = $derived(documents?.filter((doc) => doc.parse_progress < 1));
	const parsedDocuments = $derived(documents?.filter((doc) => doc.parse_progress >= 1));
</script>

<h1 class="mb-10 flex flex-row items-center gap-2 text-4xl"><Database /> Knowledge Base</h1>
<p class="mb-10">Use this space to manage your conversation's knowledge base</p>

<section class="mb-4">
	<FileUpload conversation_id={conversation.id} />
</section>
{#if parsingDocuments?.length}
	<section class="mb-8">
		<ParsingFileList documents={parsingDocuments} {conversation} />
	</section>
{/if}
{#if parsedDocuments?.length}
	<section class="mb-8">
		<ParsedFileList documents={parsedDocuments} {conversation} />
	</section>
{/if}
