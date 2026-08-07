import type { Glossary, LocalizedGlossary, LocalizedGlossaryEntry } from './types';
import { GLOSSARY_METADATA_KEY } from './parseGlossary';

function isRecord(value: unknown): value is Record<string, unknown> {
	return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function cleanTerms(value: unknown): string[] {
	if (!Array.isArray(value)) return [];
	return value
		.filter((term): term is string => typeof term === 'string')
		.map((term) => term.trim())
		.filter(Boolean);
}

/**
 * Normalizes stored glossary entries into the translatable shape, never throwing. Handles the
 * older shapes so existing data keeps working: `text` may be a locale->terms map (current) or a
 * flat `string[]` (taken as the primary locale's terms); `tooltip` may be a locale->string map
 * or a bare string (taken as the primary locale's text). Empty values and entries with no term
 * or no explanation are dropped.
 */
export function parseLocalizedGlossary(value: unknown, primaryLocale: string): LocalizedGlossary {
	if (!Array.isArray(value)) return [];

	const entries: LocalizedGlossaryEntry[] = [];
	for (const raw of value) {
		if (!isRecord(raw)) continue;

		const text: Record<string, string[]> = {};
		if (Array.isArray(raw.text)) {
			const terms = cleanTerms(raw.text);
			if (terms.length) text[primaryLocale] = terms;
		} else if (isRecord(raw.text)) {
			for (const [locale, terms] of Object.entries(raw.text)) {
				const cleaned = cleanTerms(terms);
				if (cleaned.length) text[locale] = cleaned;
			}
		}

		const tooltip: Record<string, string> = {};
		if (typeof raw.tooltip === 'string') {
			const trimmed = raw.tooltip.trim();
			if (trimmed) tooltip[primaryLocale] = trimmed;
		} else if (isRecord(raw.tooltip)) {
			for (const [locale, localeText] of Object.entries(raw.tooltip)) {
				if (typeof localeText === 'string' && localeText.trim()) {
					tooltip[locale] = localeText.trim();
				}
			}
		}

		if (Object.keys(text).length === 0 || Object.keys(tooltip).length === 0) continue;
		entries.push({ text, tooltip });
	}
	return entries;
}

/** Reads the translatable glossary out of a conversation's `metadata` blob. */
export function localizedGlossaryFromMetadata(
	metadata: unknown,
	primaryLocale: string
): LocalizedGlossary {
	if (!isRecord(metadata)) return [];
	return parseLocalizedGlossary(metadata[GLOSSARY_METADATA_KEY], primaryLocale);
}

/**
 * Flattens a translatable glossary to a single-locale Glossary for rendering. An entry's terms
 * are the current locale's (falling back to the primary locale's terms when it hasn't been
 * translated), and its tooltip is the current locale's text, falling back to the primary then
 * to any translation. Entries with no usable term or tooltip are dropped.
 */
export function resolveGlossary(
	entries: LocalizedGlossary,
	locale: string,
	primaryLocale: string
): Glossary {
	const glossary: Glossary = [];
	for (const entry of entries) {
		const text = entry.text[locale]?.length ? entry.text[locale] : entry.text[primaryLocale];
		if (!text || text.length === 0) continue;

		const tooltip =
			entry.tooltip[locale] ??
			entry.tooltip[primaryLocale] ??
			Object.values(entry.tooltip)[0];
		if (!tooltip) continue;

		glossary.push({ text, tooltip });
	}
	return glossary;
}

/** Convenience: read + resolve a conversation's glossary for the participant's locale. */
export function resolveGlossaryFromMetadata(
	metadata: unknown,
	locale: string,
	primaryLocale: string
): Glossary {
	return resolveGlossary(
		localizedGlossaryFromMetadata(metadata, primaryLocale),
		locale,
		primaryLocale
	);
}
