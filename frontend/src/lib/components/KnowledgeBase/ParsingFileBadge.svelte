<script lang="ts">
	import type { ComhairleDocument } from '$lib/api/api';
	import FileContainer from './FileContainer.svelte';
	import { File, X } from 'lucide-svelte';
	import formatFileSize from '$lib/utils/formatFileSize';
	import { Progress } from 'bits-ui';
	import { cubicInOut } from 'svelte/easing';
	import { Tween } from 'svelte/motion';
	import { apiClient } from '$lib/api/client';
	import { onDestroy, onMount } from 'svelte';
	import { invalidateAll } from '$app/navigation';
	import { notifications } from '$lib/notifications.svelte';

	type Props = {
		document: ComhairleDocument;
		knowledgeBaseId: string;
	};

	let { document, knowledgeBaseId }: Props = $props();

	let uploadingDoc: ComhairleDocument = $derived(document);
	let timeout: ReturnType<typeof setTimeout> | null = null;
	let parseProgress = $derived(
		uploadingDoc.parse_progress >= 0 ? uploadingDoc.parse_progress : 1
	);

	const tween = $derived(
		new Tween(Math.round(parseProgress * 100), {
			duration: 1000,
			easing: cubicInOut
		})
	);

	async function poll() {
		if (parseProgress >= 1 || uploadingDoc.parse_status === 'DONE') {
			await stopPolling();
			return;
		}

		try {
			const response = await apiClient.GetDocument({
				params: { document_id: document.id, knowledge_base_id: knowledgeBaseId }
			});

			uploadingDoc = response;
			tween.set(Math.round(uploadingDoc.parse_progress * 100));
		} catch (e) {
			console.error(e);
		}

		timeout = setTimeout(poll, 10_000);
	}

	function startPolling() {
		timeout = setTimeout(poll, 10_000);
	}

	async function stopPolling() {
		if (timeout) {
			clearTimeout(timeout);
			timeout = null;
			await invalidateAll();
		}
	}

	onMount(() => {
		if (parseProgress < 1 && uploadingDoc.parse_status !== 'DONE') {
			poll();
			startPolling();
		}
	});

	onDestroy(stopPolling);

	async function stopParsingDocument() {
		try {
			await apiClient.StopParsingDocument(undefined, {
				params: { document_id: document.id, knowledge_base_id: knowledgeBaseId }
			});

			notifications.send({
				message: 'Stopped document parsing',
				priority: 'INFO'
			});
		} catch (e) {
			notifications.send({
				message: 'Failed to stop document parsing',
				priority: 'ERROR'
			});
			console.error(e);
		} finally {
			await invalidateAll();
		}
	}
</script>

<FileContainer>
	<div class="mb-4 flex justify-between">
		<div class="flex items-center gap-3">
			<File class="h-5 w-5" />
			<p class="font-bold">
				{document.name}
			</p>
			<span class="text-base-muted-foreground">{formatFileSize(document.size)}</span>
		</div>
		<div class="flex gap-2">
			<button
				type="button"
				class="rounded-full bg-gray-200 p-1"
				onclick={stopParsingDocument}
			>
				<X class="h-4 w-4 text-gray-600" />
			</button>
		</div>
	</div>
	<div>
		<p class={uploadingDoc.parse_status === 'FAIL' ? 'text-red-600' : 'text-brand'}>
			{#if uploadingDoc.parse_status === 'FAIL'}
				Error
			{:else}
				{Math.round(tween.current)}%
			{/if}
		</p>
	</div>
	<div class="flex items-center gap-4">
		<Progress.Root
			aria-labelledby={document.id}
			value={Math.round(tween.current)}
			max={100}
			class="shadow-mini-inset relative h-2 w-full overflow-hidden rounded-full bg-gray-100"
		>
			<div
				class={[
					'shadow-mini-inset h-full w-full flex-1 rounded-full',
					uploadingDoc.parse_status === 'FAIL' && 'bg-red-600',
					uploadingDoc.parse_status !== 'FAIL' && 'bg-brand'
				]}
				style={`transform: translateX(-${100 - (100 * (tween.current ?? 0)) / 100}%)`}
			></div>
		</Progress.Root>
		<span>{Math.round(100 - tween.current)}%</span>
	</div>
</FileContainer>
