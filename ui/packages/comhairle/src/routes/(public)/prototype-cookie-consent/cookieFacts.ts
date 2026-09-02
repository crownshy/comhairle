// PROTOTYPE - throwaway. See NOTES.md in this folder.
//
// The real cookies this app sets, taken from CookiesSettingsContent.svelte. Umami is
// listed separately because it sets no cookie at all; whether it belongs in a consent
// UI is one of the questions these variants are poking at.

export type CookieFact = {
	name: string;
	what: string;
	plain: string;
	lasts: string;
};

export const necessary: CookieFact[] = [
	{
		name: 'auth_token',
		what: 'Keeps you signed in',
		plain: 'Without it you would have to log in again on every page.',
		lasts: 'Until you log out'
	},
	{
		name: 'paraglide_lang',
		what: 'Remembers your language',
		plain: 'So the site comes back in the language you picked.',
		lasts: '1 year'
	}
];

export const optional: CookieFact = {
	name: 'Umami',
	what: 'Counts visits',
	plain: 'Tells us how many people came and which pages they read. No cookie, no name, no profile.',
	lasts: 'Nothing stored on your device'
};
