<script lang="ts">
	import { browser } from '$app/environment';
	import type { Component } from 'svelte';
	import DownloadIcon from '@lucide/svelte/icons/download';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';

	type Props = {
		open: boolean;
		src: string | null;
		name?: string;
		downloadHref?: string | null;
	};

	let { open = $bindable(false), src, name = 'Document', downloadHref = null }: Props = $props();

	// pdfjs-dist is heavy (~1MB) and not SSR-safe, so the viewer is loaded
	// lazily in the browser the first time a document is opened.
	let PdfViewer = $state<Component<{ src: string }> | null>(null);

	$effect(() => {
		if (browser && open && !PdfViewer) {
			import('./PdfViewer.svelte').then((m) => {
				PdfViewer = m.default as unknown as Component<{ src: string }>;
			});
		}
	});
</script>

<Dialog.Root bind:open>
	<Dialog.Content
		class="flex h-[95vh] max-h-screen w-[95vw] flex-col gap-0 overflow-hidden p-0 sm:w-[90vw] sm:max-w-[95vw]"
	>
		<Dialog.Header
			class="flex-row items-center justify-between gap-4 border-b px-4 py-3 pe-12 text-left"
		>
			<Dialog.Title class="truncate text-base">{name}</Dialog.Title>
			{#if downloadHref}
				<Button
					href={downloadHref}
					download={name}
					target="_blank"
					rel="noopener noreferrer"
					variant="outline"
					size="sm"
					aria-label="Download"
				>
					<DownloadIcon class="size-4" />
					<span class="hidden sm:inline">Download</span>
				</Button>
			{/if}
		</Dialog.Header>

		<div class="bg-muted min-h-0 flex-1 overflow-hidden">
			{#if browser && open && src}
				{#if PdfViewer}
					<PdfViewer {src} />
				{:else}
					<div
						class="text-muted-foreground flex h-full items-center justify-center text-sm"
					>
						Loading viewer…
					</div>
				{/if}
			{/if}
		</div>
	</Dialog.Content>
</Dialog.Root>
