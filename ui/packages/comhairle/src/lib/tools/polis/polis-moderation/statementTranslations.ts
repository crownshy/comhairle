import type { PolisStatementTranslation } from '@crownshy/api-client/api';
import { getLocale } from '$lib/paraglide/runtime';

type TranslationFlags = Pick<PolisStatementTranslation, 'ai_generated' | 'requires_validation'>;

export function needsReview(translation: TranslationFlags): boolean {
	return translation.ai_generated && translation.requires_validation;
}

export function languageName(locale: string, displayLocale: string = getLocale()): string {
	try {
		return new Intl.DisplayNames([displayLocale], { type: 'language' }).of(locale) ?? locale;
	} catch {
		return locale;
	}
}

/**
 * The translation a moderator reads in place of the original, when the statement was written in
 * another language. Null when it is already in their language or no translation exists yet.
 */
export function translationForReader(
	sourceLocale: string | null | undefined,
	translations: PolisStatementTranslation[],
	readerLocale: string
): PolisStatementTranslation | null {
	if (!sourceLocale || sameLanguage(sourceLocale, readerLocale)) return null;
	return translations.find((t) => sameLanguage(t.locale, readerLocale)) ?? null;
}

/** Whether a statement was written in a language other than the reader's. */
export function isForeign(sourceLocale: string | null | undefined, readerLocale: string): boolean {
	return !!sourceLocale && !sameLanguage(sourceLocale, readerLocale);
}

function sameLanguage(a: string, b: string): boolean {
	return a.split('-')[0].toLowerCase() === b.split('-')[0].toLowerCase();
}
