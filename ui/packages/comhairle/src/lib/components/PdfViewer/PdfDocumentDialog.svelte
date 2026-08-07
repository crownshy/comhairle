<script lang="ts">
	import { browser } from '$app/environment';
	import type { Component } from 'svelte';
	import DownloadIcon from '@lucide/svelte/icons/download';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import type { PdfHighlight } from './highlights';

	type PdfViewerProps = { src: string; highlights?: PdfHighlight[]; initialPage?: number | null };

	type Props = {
		open: boolean;
		src: string | null;
		name?: string;
		downloadHref?: string | null;
		kind?: 'pdf' | 'image' | 'docx';
		/** Passage rectangles to shade (PDF only). */
		highlights?: PdfHighlight[];
		/** Page to open on (1-based, PDF only). */
		page?: number | null;
	};

	let {
		open = $bindable(false),
		src,
		name = 'Document',
		downloadHref = null,
		kind = 'pdf',
		highlights = [],
		page = null
	}: Props = $props();

	// pdfjs-dist (~1MB) and mammoth (~500KB) are heavy and not SSR-safe, so the
	// viewers are loaded lazily in the browser the first time a document is opened.
	let PdfViewer = $state<Component<PdfViewerProps> | null>(null);
	let DocxViewer = $state<Component<{ src: string }> | null>(null);

	$effect(() => {
		if (!browser || !open) return;
		if (kind === 'pdf' && !PdfViewer) {
			import('./PdfViewer.svelte').then((m) => {
				PdfViewer = m.default as unknown as Component<PdfViewerProps>;
			});
		} else if (kind === 'docx' && !DocxViewer) {
			import('./DocxViewer.svelte').then((m) => {
				DocxViewer = m.default as unknown as Component<{ src: string }>;
			});
		}
	});
</script>

<Dialog.Root bind:open>
	<Dialog.Content
		class="flex h-[95vh] max-h-screen w-[98vw] max-w-270 flex-col gap-0 overflow-hidden p-0 sm:w-[92vw] sm:max-w-270"
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
				{#if kind === 'image'}
					<div class="flex h-full w-full items-center justify-center overflow-auto p-4">
						<img {src} alt={name} class="max-h-full max-w-full object-contain" />
					</div>
				{:else if kind === 'docx' && DocxViewer}
					<DocxViewer {src} />
				{:else if kind === 'pdf' && PdfViewer}
					<PdfViewer {src} {highlights} initialPage={page} />
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
