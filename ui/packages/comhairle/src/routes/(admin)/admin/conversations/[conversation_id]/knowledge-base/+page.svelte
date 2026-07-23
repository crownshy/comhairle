<script lang="ts">
	import type { ComhairleDocument, ConversationWithTranslations } from '@crownshy/api-client/api';
	import { apiClient } from '@crownshy/api-client/client';
	import { invalidate } from '$app/navigation';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import FileUpload from '$lib/components/KnowledgeBase/FileUpload.svelte';
	import ParsedFileList from '$lib/components/KnowledgeBase/ParsedFileList.svelte';
	import ParsingFileList from '$lib/components/KnowledgeBase/ParsingFileList.svelte';
	import { Switch } from '$lib/components/ui/switch';
	import { Label } from '$lib/components/ui/label';
	import { notifications } from '$lib/notifications.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';

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

	// Matches the participant-side gate exactly (parse_status === 'DONE'): the assistant only
	// answers from fully parsed documents, so this is what decides whether enabling actually
	// shows anything to participants.
	const hasParsedDocs = $derived(documents?.some((doc) => doc.parse_status === 'DONE') ?? false);

	// Writable derived: tracks the saved value but can be optimistically flipped on toggle and
	// reverted if the save fails.
	let enabled = $derived(conversation.enableQaChatBot);

	async function saveEnabled(value: boolean) {
		enabled = value;
		const res = await tryCatchAsync(() =>
			apiClient.UpdateConversation(
				{ enable_qa_chat_bot: value },
				{ params: { conversation_id: conversation.id } }
			)
		);
		if (res.err !== null) {
			console.error(res.err);
			enabled = !value;
			notifications.send({ message: 'Failed to update setting', priority: 'ERROR' });
			return;
		}
		notifications.send({ message: 'Setting updated', priority: 'INFO' });
		await invalidate('conversation:meta');
	}
</script>

<svelte:head>
	<title>Knowledge Base - Comhairle Admin</title>
</svelte:head>

<PageHeader
	title="Knowledge Base"
	description="Enable the Learning Assistant and manage the documents it answers from"
/>

<section class="mb-8 flex flex-col gap-2">
	<div class="flex items-center justify-between gap-4">
		<div class="flex flex-col gap-1">
			<Label for="enable-learning-assistant" class="font-medium"
				>Show Learning Assistant</Label
			>
			<p class="text-muted-foreground text-base">
				Display a Q&amp;A Learning Assistant that answers participants' questions from the
				documents below.
			</p>
		</div>
		<Switch id="enable-learning-assistant" checked={enabled} onCheckedChange={saveEnabled} />
	</div>
	{#if enabled && !hasParsedDocs}
		<p class="text-muted-foreground text-base">
			The Learning Assistant won't appear to participants until at least one document has been
			uploaded and parsed.
		</p>
	{/if}
</section>

<p class="text-base">
	The knowledge base is a set of documents that you can use to provide users information about the
	topic at hand. They are used for a variety of tasks including influencing the helper bot and the
	elicitation bot steps.
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
