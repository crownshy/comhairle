<script lang="ts">
	import type {
		ComhairleChat,
		ComhairleDocument,
		ConversationWithTranslations
	} from '@crownshy/api-client/api';
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
	import { LEARN_CONTENT_DOCUMENT_NAME } from '$lib/utils/constants';
	import {
		sectionsToPdfDefinition,
		collectImageSources,
		type LearnContentSection,
		type ImageMap
	} from '$lib/learn/tiptapToPdf';
	import LearnSyncStatus from './LearnSyncStatus.svelte';
	import { allLanguages } from '$lib/config/languages';
	import MultiSelect from '$lib/components/ui/mutli-select/multi-select.svelte';
	import type { Option } from '$lib/components/ui/mutli-select/multi-select.svelte';

	const MAX_SIZE = 50 * MB;

	type Props = {
		data: {
			documents: ComhairleDocument[];
			conversation: ConversationWithTranslations;
			chat: ComhairleChat;
		};
	};

	let { data }: Props = $props();
	let conversation = $derived(data.conversation);
	let chat = $derived(data.chat);
	let documents = $derived(data.documents);

	// The synced learn-step content is a knowledge-base document like any other, but it is
	// managed via Sync (not the uploader), so keep it out of the uploaded-files lists and
	// surface its own status instead.
	const learnDoc = $derived(documents?.find((doc) => doc.name === LEARN_CONTENT_DOCUMENT_NAME));
	const uploadedDocuments = $derived(
		documents?.filter((doc) => doc.name !== LEARN_CONTENT_DOCUMENT_NAME) ?? []
	);

	const parsingDocuments = $derived(
		uploadedDocuments.filter((doc) => doc.parse_progress < 1 && doc.parse_progress > 0)
	);
	const parsedDocuments = $derived(
		uploadedDocuments.filter(
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

	// Largest dimension we rasterise learn-content images to. Caps the embedded PNG size so the
	// generated PDF stays reasonable while staying sharp at the ~500pt content width.
	const MAX_IMAGE_DIMENSION = 1200;

	// Fetch an image URL and rasterise it to a PNG data URL via a canvas. Going through a canvas
	// (rather than embedding the bytes directly) normalises any source format - including webp /
	// avif that pdfmake can't embed - to PNG, and lets us downscale huge images. Returns null on
	// any failure (network, or a CORS-tainted canvas): a missing image is skipped, never fatal.
	function imageUrlToPngDataUrl(url: string): Promise<string | null> {
		return new Promise((resolve) => {
			const img = new Image();
			img.crossOrigin = 'anonymous';
			img.onload = () => {
				try {
					const scale = Math.min(
						1,
						MAX_IMAGE_DIMENSION / Math.max(img.naturalWidth, img.naturalHeight)
					);
					const canvas = document.createElement('canvas');
					canvas.width = Math.max(1, Math.round(img.naturalWidth * scale));
					canvas.height = Math.max(1, Math.round(img.naturalHeight * scale));
					const ctx = canvas.getContext('2d');
					if (!ctx) return resolve(null);
					ctx.drawImage(img, 0, 0, canvas.width, canvas.height);
					resolve(canvas.toDataURL('image/png'));
				} catch (error) {
					console.error('Could not embed learn image', url, error);
					resolve(null);
				}
			};
			img.onerror = () => resolve(null);
			img.src = url;
		});
	}

	// Resolve every referenced image to an embeddable data URL. Fetched concurrently; failures
	// drop out of the map (their nodes are then skipped in the PDF).
	async function resolveImages(sources: string[]): Promise<ImageMap> {
		const entries = await Promise.all(
			sources.map(async (src) => [src, await imageUrlToPngDataUrl(src)] as const)
		);
		const map: ImageMap = {};
		for (const [src, data] of entries) {
			if (data) map[src] = data;
		}
		return map;
	}

	// Build a text-bearing PDF from the learn steps in the browser and upload it. pdfmake is
	// heavy (~1MB) and browser-only, so it (and its bundled fonts) is loaded lazily here rather
	// than in the SSR/initial bundle. Vite's CJS interop may surface the module as `default`
	// or spread its named exports, so accept either shape.
	async function generateLearnPdf(sections: LearnContentSection[]): Promise<Blob> {
		const pdfmakeModule =
			(await import('pdfmake/build/pdfmake')) as typeof import('pdfmake/build/pdfmake') & {
				default?: typeof import('pdfmake/build/pdfmake');
			};
		const pdfMake = pdfmakeModule.default ?? pdfmakeModule;
		const fontsModule = await import('pdfmake/build/vfs_fonts');
		pdfMake.addVirtualFileSystem(fontsModule.default);

		const images = await resolveImages(collectImageSources(sections));
		const definition = sectionsToPdfDefinition(sections, images);
		return pdfMake.createPdf(definition).getBlob();
	}

	// Rebuilds the reserved learn-content document in the knowledge base from the current
	// learn-step content (see ADR-0010). Content is not synced automatically, so this is the
	// admin's manual trigger after editing learn steps. The frontend fetches the raw learn
	// content, renders it to a text-bearing PDF, and uploads that; the backend owns the
	// RAGFlow dedup + parse dance.
	async function syncLearnContent() {
		isSyncing = true;

		const res = await tryCatchAsync(async () => {
			const { sections } = await apiClient.GetLearnContent({
				params: { conversation_id: conversation.id }
			});

			const blob = await generateLearnPdf(sections);

			const media = new Media();
			const formData = new FormData();
			formData.append('file', blob, LEARN_CONTENT_DOCUMENT_NAME);
			const uploadRes = await media.upload(
				`/api/conversation/${conversation.id}/documents/sync_learning_content`,
				formData,
				{ maxSize: MAX_SIZE }
			);
			if (uploadRes.err !== null) throw new Error('Failed to upload learn content');
			return uploadRes.ok;
		});

		isSyncing = false;

		if (res.err !== null) {
			console.error(res.err);
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

	let allLanguageOptions = $derived<Option[]>(
		allLanguages.map((lang) => ({ value: lang.name, label: lang.name }))
	);

	let selectedCrossLanguages = $derived<Option[]>(
		chat?.prompt?.cross_languages && chat.prompt.cross_languages.length
			? allLanguageOptions.filter(
					(langOption) =>
						!!chat?.prompt?.cross_languages?.find((lang) => lang === langOption.value)
				)
			: []
	);

	async function handleCrossLanguagesChange(options: Option[]) {
		const result = await tryCatchAsync(() =>
			apiClient.UpdateChat(
				{ prompt: { cross_languages: [...options.map((o) => o.value)] } },
				{ params: { conversation_id: conversation.id } }
			)
		);

		if (result.err !== null) {
			return notifications.send({
				priority: 'ERROR',
				message: 'Failed to update learning assistant cross languages'
			});
		}

		notifications.send({
			priority: 'INFO',
			message: 'Successfully updated learning assistant cross languages'
		});
		invalidate('knowledge-base:documents');
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
			<p class="text-muted-foreground text-base">
				Re-syncing replaces the learning-material document, so sources cited in earlier
				assistant answers will no longer open for participants.
			</p>
			<div class="flex flex-col gap-3">
				<Button
					variant="outline"
					class="self-start"
					onclick={syncLearnContent}
					disabled={isSyncing}
				>
					<RefreshCw class={isSyncing ? 'animate-spin' : ''} />
					{isSyncing ? 'Syncing...' : 'Sync learn content'}
				</Button>
				<LearnSyncStatus document={learnDoc} conversationId={conversation.id} />
			</div>
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

	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<div class="lg:w-50 lg:shrink-0 lg:pt-1">
			<p class="text-sm font-semibold">Cross-language Search</p>
		</div>
		<div class="flex-1 space-y-4">
			<p class="text-muted-foreground text-base">
				Lets users ask questions in one language and still get answers from documents
				written in another. Before searching, your question is translated into the
				document's language(s), so nothing gets missed just because of a language mismatch.
			</p>
			<div class="flex max-w-md flex-col gap-3">
				<MultiSelect
					defaultOptions={allLanguageOptions}
					selected={selectedCrossLanguages}
					onSelectedChange={handleCrossLanguagesChange}
					placeholder="Select languages..."
					ariaLabel="Supported languages"
					emptyMessage="No languages found"
					class="w-full"
				/>
			</div>
		</div>
	</div>
</div>
