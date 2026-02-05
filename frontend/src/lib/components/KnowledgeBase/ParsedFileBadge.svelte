<script lang="ts">
	import type { ComhairleDocument } from '$lib/api/api';
	import FileContainer from './FileContainer.svelte';
	import { File, Trash2, RefreshCw } from 'lucide-svelte';
	import formatFileSize from '$lib/utils/formatFileSize';
	import Button from '../ui/button/button.svelte';
	import { notifications } from '$lib/notifications.svelte';
	import { apiClient } from '$lib/api/client';
	import { invalidateAll } from '$app/navigation';

	type Props = {
		conversationId: string;
		document: ComhairleDocument;
	};

	let { document, conversationId }: Props = $props();

	async function deleteFile() {
		try {
			await apiClient.DeleteDocument(undefined, {
				params: { document_id: document.id, conversation_id: conversationId }
			});

			notifications.send({
				message: 'Document deleted',
				priority: 'INFO'
			});
		} catch (e) {
			notifications.send({
				message: 'Failed to delete file',
				priority: 'ERROR'
			});
			console.error(e);
		} finally {
			await invalidateAll();
		}
	}

	async function restartParsingFile() {
		try {
			await apiClient.ParseDocument(undefined, {
				params: { document_id: document.id, conversation_id: conversationId }
			});

			notifications.send({
				message: 'Document parsing restarted',
				priority: 'INFO'
			});
		} catch (e) {
			notifications.send({
				message: 'Failed to begin parsing file',
				priority: 'ERROR'
			});
			console.error(e);
		} finally {
			await invalidateAll();
		}
	}
</script>

<FileContainer>
	<div class="flex justify-between">
		<div class="flex items-center gap-3">
			<File class="h-5 w-5" />
			<p class="font-bold">
				{document.name}
			</p>
			<span class="text-base-muted-foreground">{formatFileSize(document.size)}</span>
		</div>
		<div class="flex gap-2">
			{#if document.parse_status === 'DONE'}
				<Button
					variant="outline"
					href={`/api/conversation/${conversationId}/documents/${document.id}/download`}
					download
				>
					Download
				</Button>
			{/if}
			{#if document.parse_status !== 'DONE'}
				<Button variant="outline" onclick={restartParsingFile}>
					<RefreshCw />
				</Button>
			{/if}
			<Button variant="outline" onclick={deleteFile}>
				<Trash2 />
			</Button>
		</div>
	</div>
	{#if document.parse_status !== 'DONE'}
		<span class="text-red-600">Parsing stopped or failed</span>
	{/if}
</FileContainer>
