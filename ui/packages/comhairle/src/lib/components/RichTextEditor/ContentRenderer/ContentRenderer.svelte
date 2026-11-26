<script module lang="ts">
	// Per-instance counter so each renderer's glossary tooltip gets a unique id for
	// aria-describedby. Increments deterministically, so SSR and hydration agree.
	let glossaryInstance = 0;
</script>

<script lang="ts">
	import { renderRichTextToHtml } from '$lib/utils/renderRichText';
	import { EDITOR_HTML_ATTRIBUTES } from '../editorConfig';
	import { cn } from '$lib/utils';
	import type { ComhairleDocument } from '@crownshy/api-client/api';
	import type { Glossary } from '$lib/glossary/types';
	import PdfDocumentDialog from '$lib/components/PdfViewer/PdfDocumentDialog.svelte';
	import { getPreviewKind, type PreviewKind } from '$lib/utils/previewKind';
	import { openImageLightbox } from './imageLightbox';
	import { attachGlossaryTooltip } from './glossaryTooltip';
	import '../editor-content.css';

	type Props = {
		content?: string;
		class?: string;
		minimal?: boolean;
		/** Documents referenced by source-document badges, for filename and download link. */
		availableDocuments?: ComhairleDocument[];
		conversationId?: string;
		/** Glossary terms to auto-tooltip in the rendered content. */
		glossary?: Glossary;
	};

	let {
		content = '',
		class: className = '',
		minimal = false,
		availableDocuments = [],
		conversationId = '',
		glossary = []
	}: Props = $props();

	// $derived (not onMount + an editor instance) so the content is present in the SSR
	// markup. This used to mount a headless Tiptap editor on the client, which meant every
	// call site painted blank until hydration, and blanked again on each remount.
	let html = $derived(
		renderRichTextToHtml(content, { documents: availableDocuments, conversationId, glossary })
	);

	let contentElement = $state<HTMLElement>();

	// Glossary tooltip. Rendered as a single position:fixed element (below) and positioned by
	// JS from the hovered/focused term's rect, so it escapes the article's overflow clipping.
	let glossaryTooltipEl = $state<HTMLDivElement>();
	const glossaryTooltipId = `glossary-tooltip-${glossaryInstance++}`;

	let previewDialog = $state<{
		open: boolean;
		kind: PreviewKind;
		src: string | null;
		name: string;
		downloadHref: string | null;
	}>({ open: false, kind: 'pdf', src: null, name: '', downloadHref: null });

	function zoomableImage(target: EventTarget | null): HTMLImageElement | null {
		return target instanceof HTMLImageElement && !target.closest('a') ? target : null;
	}

	/* Intercept source-document badge clicks: open PDFs, images, and Word
	 * documents in an in-page viewer instead of downloading. Other file types
	 * keep default download. Images outside a link open in a zoomable lightbox. */
	function handleContentClick(event: MouseEvent) {
		if (event.button !== 0 || event.metaKey || event.ctrlKey || event.shiftKey) return;

		const image = zoomableImage(event.target);
		if (image) {
			openImageLightbox(image);
			return;
		}

		const target = event.target as HTMLElement | null;
		const badge = target?.closest<HTMLAnchorElement>('a.source-document-badge');
		if (!badge) return;

		const documentId = badge.getAttribute('data-document-id');
		if (!documentId) return;

		const doc = availableDocuments.find((d) => d.id === documentId);
		if (!doc) return;
		const kind = getPreviewKind(doc.name);
		if (!kind) return;

		const href = badge.getAttribute('href');
		if (!href || href === '#') return;

		event.preventDefault();
		previewDialog = { open: true, kind, src: href, name: doc.name, downloadHref: href };
	}

	// Rendered images carry role="button" (see ZoomableImage), so they open like a button does.
	function handleContentKeydown(event: KeyboardEvent) {
		if (event.key !== 'Enter' && event.key !== ' ') return;
		const image = zoomableImage(event.target);
		if (!image) return;
		event.preventDefault();
		openImageLightbox(image);
	}

	// Delegated rather than an inline onclick: the badges come from {@html}, and putting a
	// handler on the static wrapper would trip the a11y interactive-element rules.
	$effect(() => {
		const el = contentElement;
		if (!el) return;
		el.addEventListener('click', handleContentClick);
		el.addEventListener('keydown', handleContentKeydown);
		return () => {
			el.removeEventListener('click', handleContentClick);
			el.removeEventListener('keydown', handleContentKeydown);
		};
	});

	$effect(() => {
		const container = contentElement;
		const tooltip = glossaryTooltipEl;
		if (!container || !tooltip) return;
		return attachGlossaryTooltip(container, tooltip);
	});
</script>

<div class="content-renderer {className}" class:content-renderer--minimal={minimal}>
	<!-- `.tiptap` carries all of editor-content.css, and the prose classes are what the editor
		applied through editorProps. Both are reproduced here so rendered content keeps the
		typography it had when a live editor was drawing it. -->
	<div
		bind:this={contentElement}
		class={cn('tiptap', !minimal && EDITOR_HTML_ATTRIBUTES.editor.class)}
	>
		<!-- Safe because `html` is not author-supplied markup: renderRichTextToHtml builds it by
			walking a ProseMirror document, so only nodes and marks in our schema can emit tags and
			anything else survives as escaped text. Feeding it raw markup (via a markdown-to-HTML
			library, say) would make this a real XSS hole. renderRichText.test.ts pins that down. -->
		<!-- eslint-disable-next-line svelte/no-at-html-tags -->
		{@html html}
	</div>
</div>

<!-- Single shared glossary tooltip, positioned by JS. position:fixed so it escapes the
	article's overflow clipping; text/coords are set imperatively in showGlossaryTooltip. -->
<div
	bind:this={glossaryTooltipEl}
	id={glossaryTooltipId}
	class="glossary-tooltip"
	role="tooltip"
	aria-hidden="true"
></div>

<PdfDocumentDialog
	bind:open={previewDialog.open}
	kind={previewDialog.kind}
	src={previewDialog.src}
	name={previewDialog.name}
	downloadHref={previewDialog.downloadHref}
/>

<style>
	.content-renderer {
		width: 100%;
	}

	:global(.content-renderer .tiptap) {
		min-height: unset;
	}

	:global(.content-renderer img:not(a img)) {
		cursor: zoom-in;
	}
</style>
