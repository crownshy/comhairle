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
		knowledgeBaseId: string;
		document: ComhairleDocument;
	};

	let { document, knowledgeBaseId }: Props = $props();

	async function deleteFile() {
		try {
			await apiClient.DeleteDocument(undefined, {
				params: { document_id: document.id, knowledge_base_id: knowledgeBaseId }
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
				<Button variant="outline">Download</Button>
			{/if}
			{#if document.parse_status === 'FAIL'}
				<Button variant="outline">
					<RefreshCw />
				</Button>
			{/if}
			<Button variant="outline" onclick={deleteFile}>
				<Trash2 />
			</Button>
		</div>
	</div>
</FileContainer>
