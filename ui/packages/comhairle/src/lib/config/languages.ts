import { m } from '$lib/paraglide/messages';
import { type Locale } from '$lib/paraglide/runtime';

/**
 * "native" would give the language name in the language itself, local would give it in the current user locale,
 * e.g. if the currently user locale is 'en' then:
 * getLanguageName('es', 'local') = 'Spanish'
 * getLanguageName('es', 'native') = 'Español'
 */
export function getLanguageName(locale: Locale, as: 'local' | 'native' = 'local'): string {
	switch (as) {
		case 'native':
			return m.$language({}, { locale });
		case 'local': {
			// This would usually just be m.$language();
			// But currently we only want English on the Admin side, so it's hardcoded instead of using translations
			switch (locale) {
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

export function getTextDirection(code: Locale): 'rtl' | 'ltr' {
	// Locales that are written right-to-left
	const rtlLanguages: Locale[] = ['ar', 'ps', 'prs', 'fa'];
	return rtlLanguages.includes(code) ? 'rtl' : 'ltr';
}
