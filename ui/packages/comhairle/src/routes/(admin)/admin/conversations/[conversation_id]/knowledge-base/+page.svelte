<script lang="ts">
	import type { ComhairleDocument, ConversationWithTranslations } from '@crownshy/api-client/api';
	import FileUpload from '$lib/components/KnowledgeBase/FileUpload.svelte';
	import ParsedFileList from '$lib/components/KnowledgeBase/ParsedFileList.svelte';
	import ParsingFileList from '$lib/components/KnowledgeBase/ParsingFileList.svelte';

	type Props = {
		data: {
			documents: ComhairleDocument[];
			conversation: ConversationWithTranslations;
		};
	};

	let { data }: Props = $props();
	let conversation = $derived(data.conversation);
	let documents = $derived(data.documents);

	const parsingDocuments = $derived(
		documents?.filter((doc) => doc.parse_progress < 1 && doc.parse_progress > 0)
	);
	const parsedDocuments = $derived(
		documents?.filter(
			(doc) =>
				doc.parse_progress >= 1 ||
				(doc.parse_progress === 0 && doc.parse_status === 'CANCEL')
		)
	);
</script>

<svelte:head>
	<title>Knowledge Base - Comhairle Admin</title>
</svelte:head>

<h1 class="mb-4 text-3xl font-bold">Knowledge Base</h1>

<p class="mb-10">Use this space to manage your conversation's knowledge base</p>
<p>
	The knowledge base is a set of documents that you can use to provide users information about the
	topic at hand. They are used for a variety of tasks including influcencing the helper bot and
	the elicitation bot steps
</p>

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
