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
		kind: 'pdf' | 'image' | 'docx';
		src: string | null;
		name: string;
		downloadHref: string | null;
	}>({ open: false, kind: 'pdf', src: null, name: '', downloadHref: null });

	const IMAGE_EXTENSIONS = ['.jpg', '.jpeg', '.png', '.gif', '.webp', '.bmp', '.svg', '.avif'];
	const DOCX_EXTENSIONS = ['.doc', '.docx'];

	function getPreviewKind(fileName: string): 'pdf' | 'image' | 'docx' | null {
		const lower = fileName.toLowerCase();
		if (lower.endsWith('.pdf')) return 'pdf';
		if (IMAGE_EXTENSIONS.some((ext) => lower.endsWith(ext))) return 'image';
		if (DOCX_EXTENSIONS.some((ext) => lower.endsWith(ext))) return 'docx';
		return null;
	}

	/* Intercept source-document badge clicks: open PDFs, images, and Word
	 * documents in an in-page viewer instead of downloading. Other file types
	 * keep default download. */
	function handleContentClick(event: MouseEvent) {
		if (event.button !== 0 || event.metaKey || event.ctrlKey || event.shiftKey) return;

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

	// Delegated rather than an inline onclick: the badges come from {@html}, and putting a
	// handler on the static wrapper would trip the a11y interactive-element rules.
	$effect(() => {
		const el = contentElement;
		if (!el) return;
		el.addEventListener('click', handleContentClick);
		return () => el.removeEventListener('click', handleContentClick);
	});

	function showGlossaryTooltip(trigger: HTMLElement) {
		const el = glossaryTooltipEl;
		if (!el) return;
		const text = trigger.getAttribute('data-glossary-tooltip');
		if (!text) return;

		el.textContent = text;
		el.setAttribute('data-visible', 'true');
		el.setAttribute('aria-hidden', 'false');
		trigger.setAttribute('aria-describedby', glossaryTooltipId);

		// Anchor at the term's top-centre; the CSS transform lifts and centres the box. Clamp
		// horizontally so a term near an edge doesn't push the tooltip off-screen.
		const rect = trigger.getBoundingClientRect();
		const margin = 8;
		const halfWidth = el.offsetWidth / 2;
		const centre = rect.left + rect.width / 2;
		const left = Math.min(
			Math.max(centre, halfWidth + margin),
			window.innerWidth - halfWidth - margin
		);
		el.style.left = `${left}px`;
		el.style.top = `${rect.top}px`;
	}

	function hideGlossaryTooltip(trigger?: HTMLElement | null) {
		const el = glossaryTooltipEl;
		if (!el) return;
		el.setAttribute('data-visible', 'false');
		el.setAttribute('aria-hidden', 'true');
		trigger?.removeAttribute('aria-describedby');
	}

	// Glossary terms come from {@html}, so the tooltip is driven by delegated hover/focus
	// listeners on the wrapper rather than per-element handlers.
	$effect(() => {
		const el = contentElement;
		if (!el) return;

		const closestTerm = (event: Event) =>
			(event.target as HTMLElement | null)?.closest<HTMLElement>('.glossary-term') ?? null;
		const onOver = (event: Event) => {
			const term = closestTerm(event);
			if (term) showGlossaryTooltip(term);
		};
		const onOut = (event: Event) => {
			const term = closestTerm(event);
			if (term) hideGlossaryTooltip(term);
		};
		const onScrollResize = () => hideGlossaryTooltip();

		el.addEventListener('mouseover', onOver);
		el.addEventListener('mouseout', onOut);
		el.addEventListener('focusin', onOver);
		el.addEventListener('focusout', onOut);
		// Capture so a scroll in any ancestor (the article scroll container) dismisses it.
		window.addEventListener('scroll', onScrollResize, true);
		window.addEventListener('resize', onScrollResize);

		return () => {
			el.removeEventListener('mouseover', onOver);
			el.removeEventListener('mouseout', onOut);
			el.removeEventListener('focusin', onOver);
			el.removeEventListener('focusout', onOut);
			window.removeEventListener('scroll', onScrollResize, true);
			window.removeEventListener('resize', onScrollResize);
		};
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
</style>
