import { m } from '$lib/paraglide/messages';
import { locales } from '$lib/paraglide/runtime';

export type LanguageCode = (typeof locales)[number];

/**
 * "native" would give the language name in the language itself, local would give it in the current user locale,
 * e.g. if the currently user locale is 'en' then:
 * getLanguageName('es', 'local') = 'Spanish'
 * getLanguageName('es', 'native') = 'Español'
 */
export function getLanguageName(
	languageCode: LanguageCode,
	as: 'native' | 'local' = 'local'
): string {
	switch (as) {
		case 'native':
			return m.$language({}, { locale: languageCode });
		case 'local': {
			// This would usually just be m.$language();
			// But currently we only want English on the Admin side, so it's hardcoded instead of using translations
			switch (languageCode) {
				case 'en':
					return 'English';
				case 'gd':
					return 'Gaelic';
				case 'cy':
					return 'Welsh';
				case 'zh':
					return 'Chinese';
				case 'es':
					return 'Spanish';
				case 'fr':
					return 'French';
				case 'ar':
					return 'Arabic';
				case 'pt':
					return 'Portuguese';
				case 'ps':
					return 'Pashto';
				case 'prs':
					return 'Dari';
				case 'fa':
					return 'Persian';
			}
		}
	}
}

export function getTextDirection(code: LanguageCode): 'rtl' | 'ltr' {
	// Locales that are written right-to-left
	const rtlLanguages: LanguageCode[] = ['ar', 'ps', 'prs', 'fa'];
	return rtlLanguages.includes(code) ? 'rtl' : 'ltr';
}
