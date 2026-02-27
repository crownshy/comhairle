import { sequence } from '@sveltejs/kit/hooks';
import type { Handle } from '@sveltejs/kit';
import { paraglideMiddleware } from '$lib/paraglide/server';
import { env } from '$env/dynamic/public';

const handleParaglide: Handle = ({ event, resolve }) =>
	paraglideMiddleware(event.request, ({ request, locale }) => {
		event.request = request;

		return resolve(event, {
			transformPageChunk: ({ html }) => html.replace('%paraglide.lang%', locale)
		});
	});

const handleTheme: Handle = async ({ event, resolve }) => {
	const themeName = env.PUBLIC_THEME ?? 'comhairle';
	const themeAttr = themeName !== 'comhairle' ? `data-theme="${themeName}"` : '';

	return resolve(event, {
		transformPageChunk: ({ html }) => html.replace('%comhairle.theme%', themeAttr)
	});
};

export const handle: Handle = sequence(handleTheme, handleParaglide);
