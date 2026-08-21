import { m } from '$lib/paraglide/messages';
import { typed } from '$lib/utils/types';
import { locales } from '$lib/paraglide/runtime';

export type LanguageCode = (typeof locales)[number];

interface Language {
	code: LanguageCode;
	name: string;
}

export const languages: Language[] = locales.map((code) => ({
	code,
	name: m.$language({}, { locale: code })
}));

export function getLanguageName(code: string): string {
	return languages.find((l) => l.code === code)?.name ?? code;
}

/** Locales that are written right-to-left. */
export const rtlLanguages = new Set<LanguageCode>(typed<LanguageCode[]>(['ar', 'ps', 'prs', 'fa']));

export function getTextDirection(code: LanguageCode): 'rtl' | 'ltr' {
	return rtlLanguages.has(code) ? 'rtl' : 'ltr';
}
