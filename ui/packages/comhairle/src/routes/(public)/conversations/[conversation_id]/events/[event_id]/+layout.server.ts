import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { copySetCookies } from '$lib/server/cookies';

export const load: LayoutServerLoad = async ({ url, params, fetch, cookies, parent }) => {
	const accessToken = url.searchParams.get('access');
	if (!accessToken) return;

	const { user } = await parent();

	if (!user) {
		const redeemedResult = await tryCatchAsync(() =>
			fetch(`/api/conversation/${params.conversation_id}/events/${params.event_id}/access`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ token: accessToken })
			})
		);

		if (redeemedResult.err === null && redeemedResult.ok.ok) {
			copySetCookies(cookies, redeemedResult.ok);
		}
	}

	const urlWithoutToken = new URL(url);
	urlWithoutToken.searchParams.delete('access');
	redirect(302, urlWithoutToken.pathname + urlWithoutToken.search);
};
