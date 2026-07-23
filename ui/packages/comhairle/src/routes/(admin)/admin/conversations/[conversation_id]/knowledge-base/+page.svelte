<script lang="ts">
	import type { ComhairleDocument, ConversationWithTranslations } from '@crownshy/api-client/api';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import FileUpload from '$lib/components/FileUpload.svelte';
	import ParsedFileList from '$lib/components/KnowledgeBase/ParsedFileList.svelte';
	import ParsingFileList from '$lib/components/KnowledgeBase/ParsingFileList.svelte';
	import { notifications } from '$lib/notifications.svelte';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import { invalidateAll } from '$app/navigation';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import Media from '$lib/interfaces/Media';

	const MAX_SIZE = 50;

	type Props = {
		data: {
			documents: ComhairleDocument[];
			conversation: ConversationWithTranslations;
		};
	};

	let { data }: Props = $props();
	let conversation = $derived(data.conversation);
	let documents = $derived(data.documents);

	let isUploading = $state<boolean>(false);
	let urlInput = $state<string>('');

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

	async function uploadFile(file: File) {
		const media = new Media();

		isUploading = true;
		const response = await tryCatchAsync(() =>
			media.upload(`/api/conversation/${conversation.id}/upload_document`, [file], {
				maxSizeMB: MAX_SIZE
			})
		);
		isUploading = false;

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
		await invalidateAll();
	}

	async function uploadFromUrl() {
		if (!urlInput.trim()) {
			notifications.send({
				message: 'Please enter a valid URL',
				priority: 'ERROR'
			});
			return;
		}

		isUploading = true;
		const response = await tryCatchAsync(() =>
			fetch(`/api/conversation/${conversation.id}/upload_document`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({ url: urlInput }),
				credentials: 'include'
			})
		);
		isUploading = false;

		if (response.err !== null) {
			notifications.send({
				message: 'Failed to upload from URL',
				priority: 'ERROR'
			});
			return;
		}

		notifications.send({
			message: 'File uploaded from URL successfully',
			priority: 'INFO'
		});
		urlInput = '';
		await invalidateAll();
	}
</script>

<svelte:head>
	<title>Knowledge Base - Comhairle Admin</title>
</svelte:head>

<PageHeader
	title="Knowledge Base"
	description="Use this space to manage your conversation's knowledge base"
/>
<p>
	The knowledge base is a set of documents that you can use to provide users information about the
	topic at hand. They are used for a variety of tasks including influcencing the helper bot and
	the elicitation bot steps
</p>

<section class="mb-4">
	<div class="flex w-full flex-col gap-2 border-t py-5 lg:flex-row lg:justify-between">
		<div class="flex grow flex-col gap-4">
			<FileUpload
				accept=".jpeg,.jpg,.png,.pdf,.mp4,.txt"
				maxSizeMB={MAX_SIZE}
				onfile={uploadFile}
			/>
			<div class="flex flex-col gap-2">
				<div class="text-muted-foreground text-sm">or upload from URL</div>
				<div class="flex gap-2">
					<Input
						class="flex-1"
						type="text"
						placeholder="Add file URL"
						bind:value={urlInput}
						disabled={isUploading}
					/>
					<Button
						variant="outline"
						onclick={uploadFromUrl}
						disabled={isUploading || !urlInput.trim()}
					>
						Upload
					</Button>
				</div>
			</div>
		</div>
	</div>
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
