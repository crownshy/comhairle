import type { JSONContent } from '@tiptap/core';
import type { Glossary } from './types';

export interface ApplyGlossaryOptions {
	/**
	 * Tooltip only the first occurrence of each term across the whole document
	 * (keeps content from getting noisy when a term repeats). Default: true.
	 */
	firstOccurrenceOnly?: boolean;
}

/** The mark name applied to matched terms; must match the GlossaryTerm extension. */
const GLOSSARY_MARK = 'glossaryTerm';

function escapeRegExp(value: string): string {
	return value.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

interface Matcher {
	regex: RegExp;
	/** lowercased term -> tooltip */
	lookup: Map<string, string>;
}

/**
 * Compiles the glossary into a single whole-word, case-insensitive regex plus a
 * term -> tooltip lookup. Longer terms are tried first so a multi-word entry wins
 * over one of its own words. Returns null when there is nothing to match.
 */
function buildMatcher(glossary: Glossary): Matcher | null {
	const lookup = new Map<string, string>();
	const terms: string[] = [];

	for (const entry of glossary) {
		const tooltip = entry.tooltip?.trim();
		if (!tooltip) continue;
		for (const raw of entry.text ?? []) {
			const term = raw?.trim();
			if (!term) continue;
			const key = term.toLowerCase();
			if (lookup.has(key)) continue;
			lookup.set(key, tooltip);
			terms.push(term);
		}
	}

	if (terms.length === 0) return null;

	terms.sort((a, b) => b.length - a.length);
	const pattern = terms.map(escapeRegExp).join('|');
	// Unicode-aware "word boundary": a term only matches when it isn't touching
	// another letter/number/underscore, so "bus" doesn't light up inside "business".
	const regex = new RegExp(`(?<![\\p{L}\\p{N}_])(?:${pattern})(?![\\p{L}\\p{N}_])`, 'giu');
	return { regex, lookup };
}

function splitTextNode(
	node: JSONContent,
	matcher: Matcher,
	used: Set<string>,
	firstOccurrenceOnly: boolean
): JSONContent[] {
	const text = node.text;
	if (typeof text !== 'string' || text.length === 0) return [node];

	const existingMarks = node.marks ?? [];
	// Never nest a glossary mark inside another (e.g. content that already carries one).
	if (existingMarks.some((mark) => mark.type === GLOSSARY_MARK)) return [node];

	const { regex, lookup } = matcher;
	regex.lastIndex = 0;

	const segments: JSONContent[] = [];
	let cursor = 0;
	let match: RegExpExecArray | null;

	while ((match = regex.exec(text)) !== null) {
		const matched = match[0];
		const key = matched.toLowerCase();

		// Guard against a zero-width match locking the loop.
		if (matched.length === 0) {
			regex.lastIndex += 1;
			continue;
		}

		if (firstOccurrenceOnly && used.has(key)) continue;

		const start = match.index;
		if (start > cursor) {
			segments.push({ ...node, text: text.slice(cursor, start) });
		}

		used.add(key);
		segments.push({
			...node,
			text: matched,
			marks: [...existingMarks, { type: GLOSSARY_MARK, attrs: { tooltip: lookup.get(key) } }]
		});
		cursor = start + matched.length;
	}

	if (cursor === 0) return [node];
	if (cursor < text.length) {
		segments.push({ ...node, text: text.slice(cursor) });
	}
	return segments;
}

function transformNode(
	node: JSONContent,
	matcher: Matcher,
	used: Set<string>,
	firstOccurrenceOnly: boolean
): JSONContent[] {
	if (node.type === 'text') {
		return splitTextNode(node, matcher, used, firstOccurrenceOnly);
	}

	if (Array.isArray(node.content)) {
		const content: JSONContent[] = [];
		for (const child of node.content) {
			content.push(...transformNode(child, matcher, used, firstOccurrenceOnly));
		}
		return [{ ...node, content }];
	}

	return [node];
}

/**
 * Walks a ProseMirror document and wraps every glossary term in a `glossaryTerm`
 * mark carrying its tooltip, so the renderer can show a definition on hover.
 *
 * Pure: returns a new document and never mutates the input. Applied at render time
 * (not stored), so the same content picks up glossary edits without being re-saved.
 */
export function applyGlossary(
	doc: JSONContent,
	glossary: Glossary,
	options: ApplyGlossaryOptions = {}
): JSONContent {
	const matcher = buildMatcher(glossary);
	if (!matcher) return doc;

	const firstOccurrenceOnly = options.firstOccurrenceOnly ?? true;
	const used = new Set<string>();
	return transformNode(doc, matcher, used, firstOccurrenceOnly)[0] ?? doc;
}
