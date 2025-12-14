// Shared language configuration
export const allLanguages = [
	{ code: 'en', name: 'English' },
	{ code: 'gd', name: 'Gaelic' },
	{ code: 'cy', name: 'Welsh' },
	{ code: 'ar', name: 'Arabic' },
	{ code: 'es', name: 'Spanish' },
	{ code: 'fr', name: 'French' },
	{ code: 'pt', name: 'Portuguese' },
	{ code: 'zh', name: 'Chinese' }
] as const;

export type LanguageCode = (typeof allLanguages)[number]['code'];

export function getLanguageName(code: string): string {
	return allLanguages.find(l => l.code === code)?.name ?? code;
}
