import { sequence } from '@sveltejs/kit/hooks';
import type { Handle } from '@sveltejs/kit';
import { paraglideMiddleware } from '$lib/paraglide/server';
import { env } from '$env/dynamic/public';
import { resolveThemeName, DEFAULT_THEME, THEMES } from '$lib/types/theme';

const isEmbeddable = (pathname: string) =>
	EMBEDDABLE_PATHS.some((path) => pathname.startsWith(path));

const handleParaglide: Handle = ({ event, resolve }) =>
	paraglideMiddleware(event.request, ({ request, locale }) => {
		event.request = request;

		return resolve(event, {
			transformPageChunk: ({ html }) => html.replace('%paraglide.lang%', locale)
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
