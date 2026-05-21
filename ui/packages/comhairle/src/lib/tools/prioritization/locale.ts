import type { TextContentWithTranslations } from './types';

/** Pick the best content for a translatable field. Tries the active locale, then the primary locale, then any available translation, then the fallback. Mirrors the same precedence as `getTextInLocale` in the comhairle Translation utils. */
export function pickLocalized(
	tc: TextContentWithTranslations | undefined | null,
	locale: string,
	fallback = ''
): string {
	if (!tc) return fallback;
	const exact = tc.textTranslations.find((t) => t.locale === locale);
	if (exact) return exact.content;
	const primary = tc.textTranslations.find((t) => t.locale === tc.textContent.primaryLocale);
	if (primary) return primary.content;
	return tc.textTranslations[0]?.content ?? fallback;
}
