import { sequence } from '@sveltejs/kit/hooks';
import type { Handle, HandleFetch } from '@sveltejs/kit';
import { paraglideMiddleware } from '$lib/paraglide/server';
import { env } from '$env/dynamic/public';
import { resolveThemeName, DEFAULT_THEME, THEMES } from '$lib/types/theme';
import { getTextDirection } from '$lib/paraglide/runtime';
import setCookieParser from 'set-cookie-parser';

const isEmbeddable = (pathname: string) =>
	EMBEDDABLE_PATHS.some((path) => pathname.startsWith(path));

const handleParaglide: Handle = ({ event, resolve }) =>
	paraglideMiddleware(event.request, ({ request, locale }) => {
		event.request = request;

		return resolve(event, {
			transformPageChunk: ({ html }) =>
				html
					.replace('%paraglide.lang%', locale)
					.replace('%paraglide.textDirection%', getTextDirection(locale))
		});
	});

const handleTheme: Handle = async ({ event, resolve }) => {
	const themeName = resolveThemeName(env.PUBLIC_THEME);
	const themeAttr = themeName !== DEFAULT_THEME ? `data-theme="${themeName}"` : '';
	const { favicon } = THEMES[themeName];

	return resolve(event, {
		transformPageChunk: ({ html }) =>
			html.replace('%comhairle.theme%', themeAttr).replace('%comhairle.favicon%', favicon)
	});
};

/// Sets up security headers for the app
/// Allow some paths to be embded but not others
const EMBEDDABLE_PATHS = ['/conversations'];

const handleHeaders: Handle = async ({ event, resolve }) => {
	const response = await resolve(event);
	const { pathname } = event.url;

	if (isEmbeddable(pathname)) {
		// Allow any site to embed these paths
		response.headers.set('Content-Security-Policy', 'frame-ancestors *');
		response.headers.delete('X-Frame-Options'); // XFO has no wildcard — must remove it
		// Allow jitsi iframes to access camera / microphone on embeddable paths
		response.headers.set(
			'Permissions-Policy',
			'geolocation=(), camera=(self, https://jitsi.comhairle.scot), microphone=(self, https://jitsi.comhairle.scot)'
		);
	} else {
		// Deny framing everywhere else
		response.headers.set('Content-Security-Policy', "frame-ancestors 'none'");
		response.headers.set('X-Frame-Options', 'DENY');
		response.headers.set('Permissions-Policy', 'geolocation=(), camera=(), microphone=()');
	}

	response.headers.set('X-Content-Type-Options', 'nosniff');
	response.headers.set('Referrer-Policy', 'strict-origin-when-cross-origin');
	response.headers.set(
		'Strict-Transport-Security',
		'max-age=63072000; includeSubDomains; preload'
	);
	// Belt-and-braces frame-ancestors if you have prerendered pages:
	// response.headers.set('Content-Security-Policy', "frame-ancestors 'none'");

	return response;
};

export const handle: Handle = sequence(handleTheme, handleParaglide, handleHeaders);

// Server-side `event.fetch('/api/...')` calls (form actions, load funcs) originate
// from the frontend pod, so without this the API records the NAT gateway IP instead
// of the real client. Forward the browser IP (set by nginx on the inbound request)
// on same-origin /api requests only, so it never leaks to third-party hosts.
export const handleFetch: HandleFetch = async ({ event, request, fetch }) => {
	const url = new URL(request.url);
	if (url.origin === event.url.origin && url.pathname.startsWith('/api')) {
		const xff = event.request.headers.get('x-forwarded-for');
		if (xff) {
			request.headers.set('x-forwarded-for', xff);
		}
		const realIp = event.request.headers.get('x-real-ip');
		if (realIp && !request.headers.has('x-real-ip')) {
			request.headers.set('x-real-ip', realIp);
		}
		// Forward the real browser signature; otherwise the API records the
		// server-side HTTP client's UA (e.g. "axios/1.x") instead of the user's.
		const userAgent = event.request.headers.get('user-agent');
		if (userAgent) {
			request.headers.set('user-agent', userAgent);
		}
	}

	let response = await fetch(request);

	if (response.status === 401 && !request.url.includes('/api/auth/refresh')) {
		const refreshResponse = await fetch(`${event.url.origin}/api/auth/refresh`, {
			method: 'POST',
			credentials: request.credentials
		});

		if (refreshResponse.ok) {
			// Update cookie jar to those from `/refresh` request to get new
			// `auth-token` cookie.
			const refreshSetCookieHeader = refreshResponse.headers.getSetCookie();
			const parsedSetCookies = setCookieParser(refreshSetCookieHeader);
			for (const cookie of parsedSetCookies) {
				event.cookies.set(cookie.name, cookie.value, {
					path: cookie.path ?? '/',
					maxAge: cookie.maxAge,
					httpOnly: cookie.httpOnly,
					secure: cookie.secure,
					sameSite: cookie.sameSite as SameSite
				});
			}

			const retryRequest = request.clone();
			// Delete cookie header from request after cloning to force updated
			// cookie jar (`event.cookies`) with valid `auth-token` to be used
			retryRequest.headers.delete('cookie');

			response = await fetch(retryRequest);
		}
	}

	return response;
};

type SameSite = boolean | 'lax' | 'strict' | 'none' | undefined;
