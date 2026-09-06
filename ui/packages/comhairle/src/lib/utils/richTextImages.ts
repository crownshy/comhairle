import { detectContentType } from './contentDetection';
import { CONTENT_TYPES } from '$lib/components/RichTextEditor/types';
import type { JSONContent } from '@tiptap/core';

/** Markdown images, capturing the url. Global: we want every image, not just the first. */
const MARKDOWN_IMAGE = /!\[[^\]]*\]\(([^)\s]+)/g;

function collectImageSources(node: JSONContent, into: string[]) {
	if (node?.type === 'image' && typeof node.attrs?.src === 'string') {
		into.push(node.attrs.src);
	}
	for (const child of node?.content ?? []) {
		collectImageSources(child, into);
	}
}

/**
 * Every image url in stored rich text, in document order and de-duplicated. Sibling of
 * `firstImageSrc` in step-brief/splitSlides, which wants only the hero.
 */
export function imageSourcesIn(content: string | null | undefined): string[] {
	const detected = detectContentType(content);
	if (!detected.content) return [];

	const sources: string[] = [];
	if (detected.type === CONTENT_TYPES.JSON) {
		collectImageSources(detected.content as JSONContent, sources);
	} else {
		for (const match of String(detected.content).matchAll(MARKDOWN_IMAGE)) {
			sources.push(match[1]);
		}
	}
	return [...new Set(sources)];
}

/**
 * Fetches and decodes `sources`, resolving when they are all ready or after `timeoutMs`,
 * whichever comes first. Never rejects: a broken url must not wedge whatever is waiting.
 *
 * The point is turning a page without the reflow that an <img> causes when it arrives
 * after the text around it. Once decoded, the same url paints at its real size the moment
 * the new markup lands. The timeout is what stops a slow image from holding the page
 * hostage: past it the reader gets the page and the image fills in the old way.
 */
export function preloadImages(sources: string[], timeoutMs = 900): Promise<void> {
	if (sources.length === 0 || typeof window === 'undefined') return Promise.resolve();

	const decoded = sources.map((src) => {
		const image = new Image();
		image.src = src;
		return image.decode().catch(() => {});
	});

	return Promise.race([
		Promise.all(decoded).then(() => undefined),
		new Promise<void>((resolve) => setTimeout(resolve, timeoutMs))
	]);
}
