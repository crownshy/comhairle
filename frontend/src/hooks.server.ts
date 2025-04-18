import type { Handle } from '@sveltejs/kit';
import { i18n } from '$lib/i18n';
import { createApiClient } from '$lib/api/client';

const handleParaglide = i18n.handle();

export const handle: Handle = async ({ event, resolve }) => {
	// 1. Run your own logic: create api client
	const token = event.cookies.get('auth_token');
	let url = new URL( event.request.url)

  const apiClient = createApiClient(url.origin+"/api", token, "server")

	event.locals.api = apiClient;

	// 2. Run Paraglide (i18n) handle
	return handleParaglide({ event, resolve });
};


