import type { LocalizedGlossary } from './types';
import { aiTranslateContent } from '$lib/components/Translation/translationUtils';

/**
 * Fills a target locale's missing terms and explanations by translating each entry's
 * primary-locale text, used when a new language is added to a conversation. Returns a new
 * glossary and never overwrites text already present in the target locale. Best-effort: an
 * entry whose translation fails is left unchanged.
 *
 * `translate` is injected so this stays unit-testable; it defaults to the real AI translation.
 */
export async function translateGlossaryToLocale(
	glossary: LocalizedGlossary,
	targetLocale: string,
	primaryLocale: string,
	translate: (
		text: string,
		target: string,
		primary: string
	) => Promise<string> = aiTranslateContent
): Promise<LocalizedGlossary> {
	if (targetLocale === primaryLocale) return glossary;

	const result: LocalizedGlossary = [];
	for (const entry of glossary) {
		const text = { ...entry.text };
		const tooltip = { ...entry.tooltip };

		const sourceTerms = entry.text[primaryLocale];
		if (sourceTerms?.length && !entry.text[targetLocale]?.length) {
			try {
				const translated = await translate(
					sourceTerms.join(', '),
					targetLocale,
					primaryLocale
				);
				const terms = translated
					.split(',')
					.map((term) => term.trim())
					.filter(Boolean);
				if (terms.length) text[targetLocale] = terms;
			} catch {
				// leave this entry's terms untranslated
			}
		}

		const sourceTip = entry.tooltip[primaryLocale];
		if (sourceTip?.trim() && !entry.tooltip[targetLocale]?.trim()) {
			try {
				const translated = await translate(sourceTip, targetLocale, primaryLocale);
				if (translated.trim()) tooltip[targetLocale] = translated.trim();
			} catch {
				// leave this entry's explanation untranslated
			}
		}

		result.push({ text, tooltip });
	}
	return result;
}
