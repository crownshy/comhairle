import type { Cookies } from '@sveltejs/kit';
import setCookieParser from 'set-cookie-parser';

type SameSite = boolean | 'lax' | 'strict' | 'none' | undefined;

export function copySetCookies(cookies: Cookies, response: Response) {
	for (const cookie of setCookieParser(response.headers.getSetCookie())) {
		cookies.set(cookie.name, cookie.value, {
			path: cookie.path ?? '/',
			maxAge: cookie.maxAge,
			httpOnly: cookie.httpOnly,
			secure: cookie.secure,
			sameSite: cookie.sameSite as SameSite
		});
	}
}
