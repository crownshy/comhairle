import { isRedirect, redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { conversation_url, loginRedirectServer } from '$lib/urls';
import { apiErrorStatus } from '$lib/utils/errorHandling';
import { HttpStatus } from '$lib/utils/constants';
import type { InviteDto } from '@crownshy/api-client/api';

export const load: PageLoad = async ({ params, parent, url }) => {
	const { api, conversation, user, workflows, participation } = await parent();
	const { invite_id } = params;

	// Preserve query parameters for redirects
	const queryString = url.search;

	try {
		const invite: InviteDto = await api.GetInvite({
			params: { conversation_id: conversation.id, invite_id }
		});

		// Temporarily check against `auto_create_annon` for backwards support after rename migration
		if (
			!user &&
			(invite.loginBehaviour == 'auto_create_annon' ||
				invite.loginBehaviour == 'auto_create_guest')
		) {
			await api.SignupGuestUser(undefined, {});
			redirect(307, conversation_url(conversation.id) + queryString);
		}
		if (user && invite.status === 'accepted') {
			return redirect(307, conversation_url(conversation.id) + queryString);
		}
		// Auto-redirect if user is already registered for the conversation
		if (user && participation) {
			const firstWorkflow = workflows[0];
			redirect(
				307,
				`/conversations/${conversation.id}/workflow/${firstWorkflow.id}/next${queryString}`
			);
		}
		return { invite, conversation, user, workflows, participation };
	} catch (e) {
		if (isRedirect(e)) {
			throw e;
		}
		if (apiErrorStatus(e) === HttpStatus.Unauthorized) {
			loginRedirectServer(url.pathname + url.search, 'Log in to open this invite');
		}
		console.error(e);
		return { error: inviteErrorText(e), conversation };
	}
};

/** The API's error text, or an empty string when the request never got a response. */
function inviteErrorText(error: unknown): string {
	const text = (error as { response?: { data?: { err?: unknown } } } | null)?.response?.data?.err;
	return typeof text === 'string' ? text : '';
}
