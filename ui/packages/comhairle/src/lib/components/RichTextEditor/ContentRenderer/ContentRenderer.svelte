<script lang="ts">
	import { onMount, onDestroy, untrack } from 'svelte';
	import { Editor } from '@tiptap/core';
	import { detectContentType } from '$lib/utils/contentDetection';
	import { getBaseExtensions, getEditorProps } from '../editorConfig';
	import { SourceDocument } from '../extensions/sourceDocument';
	import type { ComhairleDocument } from '@crownshy/api-client/api';
	import PdfDocumentDialog from '$lib/components/PdfViewer/PdfDocumentDialog.svelte';
	import '../editor-content.css';

	function buildDocMap(docs: ComhairleDocument[]) {
		const map: Record<string, { name: string; size: number }> = {};
		for (const doc of docs) {
			map[doc.id] = { name: doc.name, size: doc.size };
		}
		return map;
	}

	type Props = {
		content?: string;
		class?: string;
		minimal?: boolean;
		availableDocuments?: ComhairleDocument[];
		conversationId?: string;
	};

	let {
		content = '',
		class: className = '',
		minimal = false,
		availableDocuments = [],
		conversationId = ''
	}: Props = $props();

	let editorElement = $state<HTMLElement>();
	let editor = $state<Editor>();
	let lastDocMapKey = $state('');

	let pdfDialog = $state<{
		open: boolean;
		src: string | null;
		name: string;
		downloadHref: string | null;
	}>({ open: false, src: null, name: '', downloadHref: null });

	function isPdf(fileName: string): boolean {
		return fileName.toLowerCase().endsWith('.pdf');
	}

	/* Intercept source-document badge clicks: open PDFs in an in-page viewer
	 * instead of downloading. Non-PDF files keep the default download behaviour. */
	function handleContentClick(event: MouseEvent) {
		if (event.button !== 0 || event.metaKey || event.ctrlKey || event.shiftKey) return;

		const target = event.target as HTMLElement | null;
		const badge = target?.closest<HTMLAnchorElement>('a.source-document-badge');
		if (!badge) return;

		const documentId = badge.getAttribute('data-document-id');
		if (!documentId) return;

		const doc = availableDocuments.find((d) => d.id === documentId);
		if (!doc || !isPdf(doc.name)) return;

		const href = badge.getAttribute('href');
		if (!href || href === '#') return;

		event.preventDefault();
		pdfDialog = { open: true, src: href, name: doc.name, downloadHref: href };
	}

	function createRenderer() {
		untrack(() => {
			if (editor) {
				editor.destroy();
				editor = undefined;
			}
		});
		if (!editorElement) return;

		try {
			const currentContent = untrack(() => content);
			const detected = detectContentType(currentContent);
			const docMap = untrack(() => buildDocMap(availableDocuments));
			lastDocMapKey = JSON.stringify({
				docMap,
				conversationId: untrack(() => conversationId)
			});

			editor = new Editor({
				element: editorElement,
				extensions: [
					...getBaseExtensions({ mode: 'renderer' }).filter(
						(ext) => ext.name !== 'sourceDocument'
					),
					SourceDocument.configure({ documents: docMap, conversationId })
				],
				content: detected.content,
				contentType: detected.type,
				editable: false,
				editorProps: minimal ? {} : getEditorProps()
			});
		} catch (error) {
			console.error('[ContentRenderer] Failed to initialize:', error);
		}
	}

	onMount(() => {
		createRenderer();
	});

	$effect(() => {
		if (editor && content !== undefined) {
			try {
				const detected = detectContentType(content);

				editor.commands.setContent(detected.content, {
					contentType: detected.type,
					emitUpdate: false
				});
			} catch (error) {
				console.error('[ContentRenderer] Failed to update content:', error);
			}
		}
	});

	// When availableDocuments / conversationId change (e.g. async fetch resolves),
	// recreate the editor so SourceDocument nodes re-render with correct name/size/href.
	// (Tiptap extension options are captured at construction; setContent on identical
	// docs won't redraw atom nodeViews.)
	$effect(() => {
		const docMap = buildDocMap(availableDocuments);
		const newKey = JSON.stringify({ docMap, conversationId });
		if (newKey !== lastDocMapKey && editorElement) {
			createRenderer();
		}
	});

	// Delegated click handling for source-document badges rendered by Tiptap.
	$effect(() => {
		const el = editorElement;
		if (!el) return;
		el.addEventListener('click', handleContentClick);
		return () => el.removeEventListener('click', handleContentClick);
	});

	onDestroy(() => {
		if (editor) {
			editor.destroy();
		}
	});
</script>

<div
	class="content-renderer {className}"
	class:content-renderer--minimal={minimal}
	bind:this={editorElement}
>
	<!-- Tiptap editor renders here -->
</div>

<PdfDocumentDialog
	bind:open={pdfDialog.open}
	src={pdfDialog.src}
	name={pdfDialog.name}
	downloadHref={pdfDialog.downloadHref}
/>

<style>
	.content-renderer {
		width: 100%;
	}

	:global(.content-renderer .tiptap) {
		min-height: unset;
	}
</style>
