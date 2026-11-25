// Handbook glossary tooltips: the first mention of each ADMIN_GLOSSARY term on a page.
// Matches like src/lib/glossary/applyGlossary.ts (whole words, longest term first).
import { ADMIN_GLOSSARY, glossaryTermId, type AdminGuideTerm } from './admin_guides';
import { parseGuideText, type GuideTextPart } from './admin_guide_text';

const GLOSSARY_HREF = '/admin/info/glossary#';

export type GlossaryMatcher = {
	regex: RegExp;
	/** Any matchable form, lowercased → the glossary entry's `term`. */
	forms: Map<string, string>;
	/** Anchor id (term-seed-statement) → the glossary entry's `term`. */
	anchors: Map<string, string>;
	/** The glossary entry's `term` → the whole entry (definition, learnMore link). */
	entries: Map<string, AdminGuideTerm>;
};

function escapeRegExp(value: string) {
	return value.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

/** step → steps, policy → policies, process → processes. Multi-word terms pluralise the last word. */
export function pluralise(word: string) {
	if (/[^aeiou]y$/i.test(word)) return `${word.slice(0, -1)}ies`;
	if (/(s|x|z|ch|sh)$/i.test(word)) return `${word}es`;
	return `${word}s`;
}

export function createGlossaryMatcher(terms: AdminGuideTerm[]): GlossaryMatcher {
	const forms = new Map<string, string>();
	const anchors = new Map<string, string>();
	const entries = new Map<string, AdminGuideTerm>();

	for (const entry of terms) {
		entries.set(entry.term, entry);
		anchors.set(glossaryTermId(entry.term), entry.term);
		for (const form of [entry.term, pluralise(entry.term), ...(entry.aliases ?? [])]) {
			const key = form.toLowerCase();
			if (!forms.has(key)) forms.set(key, entry.term);
		}
	}

	const pattern = [...forms.keys()]
		.sort((a, b) => b.length - a.length)
		.map(escapeRegExp)
		.join('|');
	// Unicode-aware word boundary, so "step" doesn't light up inside "footstep".
	const regex = new RegExp(`(?<![\\p{L}\\p{N}_-])(?:${pattern})(?![\\p{L}\\p{N}_-])`, 'giu');
	return { regex, forms, anchors, entries };
}

export const HANDBOOK_GLOSSARY = createGlossaryMatcher(ADMIN_GLOSSARY.terms);

/** The glossary term a link points at, if it is a link to a glossary card. */
function linkedTerm(href: string, matcher: GlossaryMatcher) {
	if (!href.startsWith(GLOSSARY_HREF)) return undefined;
	return matcher.anchors.get(href.slice(GLOSSARY_HREF.length));
}

/**
 * Turns the first mention of each term in `available` into a `term` part, removing the
 * term from `available` as it goes. Pass the same Set through every string on a page so
 * only the first mention on the whole page is marked.
 */
export function markGlossaryTerms(
	parts: GuideTextPart[],
	available: Set<string>,
	matcher: GlossaryMatcher = HANDBOOK_GLOSSARY
): GuideTextPart[] {
	if (available.size === 0) return parts;
	const out: GuideTextPart[] = [];

	for (const part of parts) {
		if (part.kind === 'link') {
			const term = linkedTerm(part.href, matcher);
			if (term && available.delete(term)) {
				out.push({ kind: 'term', text: part.text, term });
			} else {
				out.push(part);
			}
			continue;
		}
		if (part.kind !== 'text') {
			out.push(part);
			continue;
		}

		let cursor = 0;
		for (const match of part.text.matchAll(matcher.regex)) {
			const term = matcher.forms.get(match[0].toLowerCase());
			if (!term || !available.delete(term)) continue;
			const index = match.index ?? 0;
			if (index > cursor) out.push({ kind: 'text', text: part.text.slice(cursor, index) });
			out.push({ kind: 'term', text: match[0], term });
			cursor = index + match[0].length;
		}
		if (cursor === 0) out.push(part);
		else if (cursor < part.text.length)
			out.push({ kind: 'text', text: part.text.slice(cursor) });
	}
	return out;
}

/**
 * Works out, for one page, which terms each string should mark. `texts` must be in the
 * order they appear on the page. Returns string → terms first mentioned in it; give that
 * Set to GuideRichText as `terms`.
 */
export function planGlossaryTerms(
	texts: (string | undefined)[],
	matcher: GlossaryMatcher = HANDBOOK_GLOSSARY
): Map<string, Set<string>> {
	const available = new Set(matcher.entries.keys());
	const plan = new Map<string, Set<string>>();

	for (const text of texts) {
		if (!text || plan.has(text) || available.size === 0) continue;
		const marked = markGlossaryTerms(parseGuideText(text), available, matcher);
		const terms = new Set(marked.flatMap((part) => (part.kind === 'term' ? [part.term] : [])));
		if (terms.size) plan.set(text, terms);
	}
	return plan;
}
