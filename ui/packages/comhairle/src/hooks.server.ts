import { sequence } from '@sveltejs/kit/hooks';
import type { Handle, HandleFetch } from '@sveltejs/kit';
import { paraglideMiddleware } from '$lib/paraglide/server';
import { env } from '$env/dynamic/public';
import { resolveThemeName, DEFAULT_THEME, THEMES } from '$lib/types/theme';
import { getTextDirection } from '$lib/paraglide/runtime';
import { serverApiBaseUrl } from '$lib/apiBaseUrl';

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
// on requests to our own API only, so it never leaks to third-party hosts.
export const handleFetch: HandleFetch = ({ event, request, fetch }) => {
	const url = new URL(request.url);
	// Matches our API whether it is reached through the public origin or, in a deployment that
	// sets one, the internal service address.
	const isApiRequest =
		(url.origin === event.url.origin && url.pathname.startsWith('/api')) ||
		request.url.startsWith(serverApiBaseUrl(event.url));
	if (isApiRequest) {
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
	return fetch(request);
};
