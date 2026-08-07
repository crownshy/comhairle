/**
 * A conversation glossary: a shared list of defined terms that get an automatic
 * hover tooltip wherever they appear in rendered rich text (Learn steps for now).
 *
 * This is the demo shape agreed in the discussion on issue #815. It is intentionally
 * small; conversation-level storage and a builder UI are the follow-up work.
 */
export interface GlossaryEntry {
	/**
	 * The term plus any synonyms or translated forms that should all show the same
	 * explanation, e.g. `["bus", "autobus"]`. Matching is case-insensitive and
	 * whole-word.
	 */
	text: string[];
	/** Plain-text explanation shown on hover / focus. */
	tooltip: string;
}

export type Glossary = GlossaryEntry[];

/**
 * The stored, translatable form of an entry: BOTH the term list and the explanation are held
 * per locale, so a term matches in the language it's written in and shows a same-language
 * tooltip. Resolved down to a plain `GlossaryEntry` (single locale) for rendering by
 * resolveGlossary. See [[localizedGlossary]].
 */
export interface LocalizedGlossaryEntry {
	/** Terms (and synonyms) keyed by locale code, e.g. `{ en: ["bus"], es: ["autobús"] }`. */
	text: Record<string, string[]>;
	/** Explanation keyed by locale code, e.g. `{ en: "...", es: "..." }`. */
	tooltip: Record<string, string>;
}

export type LocalizedGlossary = LocalizedGlossaryEntry[];
