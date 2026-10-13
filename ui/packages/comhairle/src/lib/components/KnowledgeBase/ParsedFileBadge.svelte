<script lang="ts">
	import type { ComhairleDocument } from '@crownshy/api-client/api';
	import FileContainer from './FileContainer.svelte';
	import { File, Trash2, RefreshCw } from 'lucide-svelte';
	import formatFileSize from '$lib/utils/formatFileSize';
	import Button from '../ui/button/button.svelte';
	import { notifications } from '$lib/notifications.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import { invalidate } from '$app/navigation';
	import { key } from '$lib/utils/invalidationKey';

	type Props = {
		conversationId: string;
		document: ComhairleDocument;
		editable?: boolean;
	};

	let { document, conversationId, editable = true }: Props = $props();

	async function deleteFile() {
		if (!editable) return;
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
			await invalidate(key('admin/knowledge-base/documents'));
		}
	}

	async function restartParsingFile() {
		if (!editable) return;
		try {
			await apiClient.ParseDocument(undefined, {
				params: { document_id: document.id, conversation_id: conversationId }
			});

			notifications.send({
				message: 'Document processing restarted',
				priority: 'INFO'
			});
		} catch (e) {
			notifications.send({
				message: 'Failed to begin processing file',
				priority: 'ERROR'
			});
			console.error(e);
		} finally {
			await invalidate(key('admin/knowledge-base/documents'));
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
			{#if editable && document.parse_status !== 'DONE'}
				<Button variant="outline" onclick={restartParsingFile}>
					<RefreshCw />
				</Button>
			{/if}
			{#if editable}
				<Button variant="outline" onclick={deleteFile}>
					<Trash2 />
				</Button>
			{/if}
		</div>
	</div>
	{#if document.parse_status !== 'DONE'}
		<span class="text-red-600">Processing stopped or failed</span>
	{/if}
</FileContainer>
