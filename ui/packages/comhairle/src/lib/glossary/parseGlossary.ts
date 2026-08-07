import type { Glossary, GlossaryEntry } from './types';

/** The key the glossary lives under in a conversation's `metadata` jsonb blob. */
export const GLOSSARY_METADATA_KEY = 'glossary';

function isRecord(value: unknown): value is Record<string, unknown> {
	return typeof value === 'object' && value !== null;
}

/**
 * Coerces an untyped value (the glossary comes out of `metadata`, typed `unknown`) into a
 * valid Glossary, dropping anything malformed. Never throws, so bad stored data degrades to
 * "no tooltips" rather than a render crash.
 */
export function parseGlossary(value: unknown): Glossary {
	if (!Array.isArray(value)) return [];

	const entries: GlossaryEntry[] = [];
	for (const raw of value) {
		if (!isRecord(raw)) continue;

		const tooltip = typeof raw.tooltip === 'string' ? raw.tooltip.trim() : '';
		const text = Array.isArray(raw.text)
			? raw.text
					.filter((term): term is string => typeof term === 'string')
					.map((term) => term.trim())
					.filter(Boolean)
			: [];

		if (!tooltip || text.length === 0) continue;
		entries.push({ text, tooltip });
	}
	return entries;
}

/** Reads the glossary out of a conversation's `metadata` blob (`conversation.metadata`). */
export function glossaryFromMetadata(metadata: unknown): Glossary {
	if (!isRecord(metadata)) return [];
	return parseGlossary(metadata[GLOSSARY_METADATA_KEY]);
}
