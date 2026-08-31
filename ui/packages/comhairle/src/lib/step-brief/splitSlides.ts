import { detectContentType } from '$lib/utils/contentDetection';
import { CONTENT_TYPES } from '$lib/components/RichTextEditor/types';
import type { JSONContent } from '@tiptap/core';

/**
 * A markdown thematic break on its own line: three or more -, * or _, optionally spaced.
 * Anchored to a line that contains nothing else, so `--- text` is not a break.
 */
const THEMATIC_BREAK = /^[ \t]*(?:(?:-[ \t]*){3,}|(?:\*[ \t]*){3,}|(?:_[ \t]*){3,})$/;

/** Matches a lone markdown image on a line, for the cover illustration. */
const MARKDOWN_IMAGE = /!\[[^\]]*\]\(([^)\s]+)/;

/** The same image including its closing paren and any title, for removal. */
const MARKDOWN_IMAGE_FULL = /!\[[^\]]*\]\([^)]*\)/;

function isBlank(line: string): boolean {
	return line.trim().length === 0;
}

/**
 * Splits a Markdown or plain-text description.
 *
 * A break only counts when the preceding line is blank or absent. Markdown reads `---`
 * directly under text as a setext H2, so treating every match as a break would turn every
 * underlined heading into a slide boundary.
 */
function splitMarkdown(source: string): string[] {
	const lines = source.split('\n');
	const slides: string[] = [];
	let current: string[] = [];

	for (let i = 0; i < lines.length; i++) {
		const line = lines[i];
		const previous = i === 0 ? '' : lines[i - 1];
		const isBreak = THEMATIC_BREAK.test(line) && (i === 0 || isBlank(previous));

		if (isBreak) {
			slides.push(current.join('\n'));
			current = [];
			continue;
		}
		current.push(line);
	}
	slides.push(current.join('\n'));

	return slides.map((slide) => slide.trim()).filter((slide) => slide.length > 0);
}

/**
 * Splits a ProseMirror document at its top-level `horizontalRule` nodes, returning one
 * serialised `doc` per run. The rules are consumed and never rendered.
 */
function splitProseMirror(document: JSONContent): string[] {
	const nodes = Array.isArray(document.content) ? document.content : [];
	const runs: JSONContent[][] = [[]];

	for (const node of nodes) {
		if (node?.type === 'horizontalRule') {
			runs.push([]);
			continue;
		}
		runs[runs.length - 1].push(node);
	}

	return runs
		.filter((run) => run.length > 0)
		.map((run) => JSON.stringify({ ...document, content: run }));
}

/**
 * Splits a step description into the slides of its step brief (ADR-0017).
 *
 * Each returned string is renderable content in the same shape the description arrived in,
 * so it can go straight to `ContentRenderer` with no other change. A description with no
 * break is one slide; an empty description is no slides at all, which callers render as a
 * cover with title and meta only.
 */
export function splitSlides(description: string | null | undefined): string[] {
	const detected = detectContentType(description);
	if (!detected.content) return [];

	if (detected.type === CONTENT_TYPES.JSON) {
		return splitProseMirror(detected.content as JSONContent);
	}
	return splitMarkdown(String(detected.content));
}

/** Depth-first search for the first `image` node's `src`. */
function findImageSrc(node: JSONContent): string | null {
	if (node?.type === 'image' && typeof node.attrs?.src === 'string') {
		return node.attrs.src;
	}
	for (const child of node?.content ?? []) {
		const found = findImageSrc(child);
		if (found) return found;
	}
	return null;
}

/**
 * The illustration for a slide: its first image, or null to fall back to the tool icon.
 * The cover lifts this out as a hero, so pair it with {@link withoutFirstImage}.
 */
export function firstImageSrc(slide: string | null | undefined): string | null {
	const detected = detectContentType(slide);
	if (!detected.content) return null;

	if (detected.type === CONTENT_TYPES.JSON) {
		return findImageSrc(detected.content as JSONContent);
	}
	return String(detected.content).match(MARKDOWN_IMAGE)?.[1] ?? null;
}

/** Removes the first `image` node found, depth first. Returns whether it removed one. */
function stripFirstImage(node: JSONContent): boolean {
	const children = node?.content;
	if (!Array.isArray(children)) return false;

	for (let i = 0; i < children.length; i++) {
		const child = children[i];
		if (child?.type === 'image') {
			children.splice(i, 1);
			return true;
		}
		if (stripFirstImage(child)) {
			// An image can be the only thing in a paragraph. Drop the empty husk so the
			// slide does not render a blank line where the hero used to be.
			if (Array.isArray(child.content) && child.content.length === 0) {
				children.splice(i, 1);
			}
			return true;
		}
	}
	return false;
}

/**
 * The slide's content with its first image removed, for callers that render that image
 * separately as a hero. Content with no image comes back unchanged.
 */
export function withoutFirstImage(slide: string | null | undefined): string {
	const detected = detectContentType(slide);
	if (!detected.content) return '';

	if (detected.type !== CONTENT_TYPES.JSON) {
		return String(detected.content).replace(MARKDOWN_IMAGE_FULL, '').trim();
	}

	const document = JSON.parse(JSON.stringify(detected.content)) as JSONContent;
	stripFirstImage(document);
	return JSON.stringify(document);
}
