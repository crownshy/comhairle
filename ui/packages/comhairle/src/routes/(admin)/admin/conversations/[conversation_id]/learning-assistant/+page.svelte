<script lang="ts">
	import type { ComhairleDocument, ConversationWithTranslations } from '@crownshy/api-client/api';
	import { apiClient } from '@crownshy/api-client/client';
	import { invalidate } from '$app/navigation';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import FileInput from '$lib/components/FileInput.svelte';
	import ParsedFileList from '$lib/components/KnowledgeBase/ParsedFileList.svelte';
	import ParsingFileList from '$lib/components/KnowledgeBase/ParsingFileList.svelte';
	import { Switch } from '$lib/components/ui/switch';
	import { Button } from '$lib/components/ui/button';
	import * as Alert from '$lib/components/ui/alert';
	import { TriangleAlert, RefreshCw } from 'lucide-svelte';
	import { notifications } from '$lib/notifications.svelte';
	import { invalidateAll } from '$app/navigation';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import Media from '$lib/interfaces/Media';
	import { MB } from '$lib/utils/units';

	const MAX_SIZE = 50 * MB;

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

	let isSyncing = $state(false);

	// Rebuilds the reserved learn-content document in the knowledge base from the current
	// learn-step content (see ADR-0010). Content is not synced automatically, so this is the
	// admin's manual trigger after editing learn steps.
	async function syncLearnContent() {
		isSyncing = true;
		const res = await tryCatchAsync(() =>
			fetch(`/api/conversation/${conversation.id}/documents/sync_learning_content`, {
				method: 'POST',
				credentials: 'include'
			})
		);
		isSyncing = false;

		if (res.err !== null) {
			console.error(res.err);
			notifications.send({ message: 'Failed to sync learn content', priority: 'ERROR' });
			return;
		}
		if (!res.ok.ok) {
			console.error(res.ok.statusText);
			notifications.send({ message: 'Failed to sync learn content', priority: 'ERROR' });
			return;
		}

		notifications.send({
			message: 'Learn content sync started. It will be ready once parsing finishes.',
			priority: 'INFO'
		});
		await invalidateAll();
	}

	async function uploadFile(file: File) {
		const media = new Media();
		const formData = new FormData();
		formData.append('file', file);

		const response = await tryCatchAsync(() =>
			media.upload(`/api/conversation/${conversation.id}/documents`, formData, {
				maxSize: MAX_SIZE
			})
		);

		if (response.err !== null) {
			notifications.send({
				message: 'Failed to upload file',
				priority: 'ERROR'
			});
			return;
		}

		notifications.send({
			message: 'File uploaded successfully',
			priority: 'INFO'
		});
		await invalidate('knowledge-base:documents');
	}

	// FIX: Upload from Url functionality
	// async function uploadFromUrl() {
	// 	if (!urlInput.trim()) {
	// 		notifications.send({
	// 			message: 'Please enter a valid URL',
	// 			priority: 'ERROR'
	// 		});
	// 		return;
	// 	}
	//
	// 	isUploading = true;
	// 	const response = await tryFetch(`/api/conversation/${conversation.id}/upload_document`, {
	// 		method: 'POST',
	// 		headers: {
	// 			'Content-Type': 'application/json'
	// 		},
	// 		body: JSON.stringify({ url: urlInput }),
	// 		credentials: 'include'
	// 	});
	// 	isUploading = false;
	//
	// 	if (response.err !== null) {
	// 		notifications.send({
	// 			message: 'Failed to upload from URL',
	// 			priority: 'ERROR'
	// 		});
	// 		return;
	// 	}
	//
	// 	notifications.send({
	// 		message: 'File uploaded from URL successfully',
	// 		priority: 'INFO'
	// 	});
	// 	urlInput = '';
	// 	await invalidate('knowledge-base:documents');
	// }
</script>

<svelte:head>
	<title>Learning Assistant - Comhairle Admin</title>
</svelte:head>

<PageHeader
	title="Learning Assistant"
	description="Enable the Learning Assistant and manage the documents it answers from"
/>

<div class="flex flex-col">
	<!-- Enable toggle. Mirrors the Configure page's two-column row (label left, control right,
		divider border) so this page reads like the rest of the admin form pages. -->
	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<div class="lg:w-50 lg:shrink-0 lg:pt-1">
			<label for="enable-learning-assistant" class="text-sm font-semibold">
				Show Learning Assistant
			</label>
		</div>
		<div class="flex-1 space-y-2">
			<div class="flex items-start gap-3">
				<Switch
					id="enable-learning-assistant"
					class="mt-0.5"
					checked={enabled}
					onCheckedChange={saveEnabled}
				/>
				<p class="text-muted-foreground text-base">
					Display a Q&amp;A Learning Assistant that answers participants' questions from
					the documents below.
				</p>
			</div>
			{#if enabled && !hasParsedDocs}
				<Alert.Root>
					<TriangleAlert />
					<Alert.Description>
						Upload and parse at least one document below, otherwise the Learning
						Assistant won't appear to participants.
					</Alert.Description>
				</Alert.Root>
			{/if}
		</div>
	</div>

	<!-- Learn content sync. Pushes the authored learn-step text into the knowledge base so the
		assistant can answer against what participants read. Manual by design (ADR-0010). -->
	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<div class="lg:w-50 lg:shrink-0 lg:pt-1">
			<p class="text-sm font-semibold">Learn content</p>
		</div>
		<div class="flex-1 space-y-4">
			<p class="text-muted-foreground text-base">
				Sync your learn-step pages into the knowledge base so the assistant can answer
				questions about them. Changes to learn steps are not picked up automatically, so
				re-sync after you finish editing.
			</p>
			<Button variant="outline" onclick={syncLearnContent} disabled={isSyncing}>
				<RefreshCw class={isSyncing ? 'animate-spin' : ''} />
				{isSyncing ? 'Syncing...' : 'Sync learn content'}
			</Button>
		</div>
	</div>

	<!-- Documents (the knowledge base itself). -->
	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<div class="lg:w-50 lg:shrink-0 lg:pt-1">
			<p class="text-sm font-semibold">Documents</p>
		</div>
		<div class="flex-1 space-y-4">
			<p class="text-muted-foreground text-base">
				The knowledge base is a set of documents you can use to provide participants
				information about the topic at hand. They also inform the helper bot and elicitation
				bot steps.
			</p>
			<section class="mt-4 flex w-full flex-col gap-4 border-t pt-6">
				<FileInput
					name="files"
					accept=".jpeg,.jpg,.png,.pdf,.mp4,.txt"
					maxSize={MAX_SIZE}
					onfile={uploadFile}
					multiple
				/>
				<!-- FIX: Upload from Url functionality -->
				<!-- <div> -->
				<!-- 	<div class="text-muted-foreground my-2 text-sm">or upload from URL</div> -->
				<!-- 	<div class="flex gap-2"> -->
				<!-- 		<Input -->
				<!-- 			class="flex-1" -->
				<!-- 			type="text" -->
				<!-- 			placeholder="Add file URL" -->
				<!-- 			bind:value={urlInput} -->
				<!-- 			disabled={isUploading} -->
				<!-- 		/> -->
				<!-- 		<Button -->
				<!-- 			variant="outline" -->
				<!-- 			onclick={uploadFromUrl} -->
				<!-- 			disabled={isUploading || !urlInput.trim()} -->
				<!-- 		> -->
				<!-- 			Upload -->
				<!-- 		</Button> -->
				<!-- 	</div> -->
				<!-- </div> -->
			</section>
			{#if parsingDocuments?.length}
				<ParsingFileList documents={parsingDocuments} {conversation} />
			{/if}
			{#if parsedDocuments?.length}
				<ParsedFileList documents={parsedDocuments} {conversation} />
			{/if}
		</div>
	</div>
</div>
