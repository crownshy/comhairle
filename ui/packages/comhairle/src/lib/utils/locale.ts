import { setLocale, type Locale } from '$lib/paraglide/runtime';
import { Day } from '$lib/utils/units';

/** hooks.server.ts reads this so the API answers in the same language. */
export const LOCALE_COOKIE = 'COMHAIRLE_LOCALE';

export function switchLocale(locale: Locale) {
	const expires = new Date(Date.now() + 365 * Day).toUTCString();
	document.cookie = `${LOCALE_COOKIE}=${locale};expires=${expires};path=/;SameSite=Lax`;
	setLocale(locale);
}
