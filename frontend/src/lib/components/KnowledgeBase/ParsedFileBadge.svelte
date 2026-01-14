<script lang="ts">
	import type { ComhairleDocument } from '$lib/api/api';
	import FileContainer from './FileContainer.svelte';
	import { File, RefreshCw, Trash2 } from 'lucide-svelte';
	import formatFileSize from '$lib/utils/formatFileSize';
	import Button from '../ui/button/button.svelte';

	type Props = {
		document: ComhairleDocument;
	};

	let { document }: Props = $props();
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
				<Button variant="outline"><RefreshCw /></Button>
			{/if}
			<Button variant="outline"><Trash2 /></Button>
		</div>
	</div>
</FileContainer>
