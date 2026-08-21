export const allLanguages = [
	{ code: 'en', name: 'English' },
	{ code: 'gd', name: 'Gaelic' },
	{ code: 'cy', name: 'Welsh' },
	{ code: 'ar', name: 'Arabic' },
	{ code: 'es', name: 'Spanish' },
	{ code: 'fr', name: 'French' },
	{ code: 'pt', name: 'Portuguese' },
	{ code: 'zh', name: 'Chinese' },
	{ code: 'ps', name: 'Pashto' },
	{ code: 'prs', name: 'Dari' },
	{ code: 'fa', name: 'Farsi' }
] as const;

export type LanguageCode = (typeof allLanguages)[number]['code'];

export function getLanguageName(code: string): string {
	return allLanguages.find((l) => l.code === code)?.name ?? code;
}

/** Locales that are written right-to-left. */
export const rtlLanguages = new Set(['ar', 'ps', 'prs', 'fa']);

export function getTextDirection(code: string): 'rtl' | 'ltr' {
	return rtlLanguages.has(code) ? 'rtl' : 'ltr';
}
