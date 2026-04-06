import { isRedirect, redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { conversation_url } from '$lib/urls';

export const load: PageLoad = async ({ params, parent }) => {
	const { api, conversation, user, workflows, participation } = await parent();
	const { invite_id } = params;

	try {
		const invite = await api.GetInvite({
			params: { conversation_id: conversation.id, invite_id }
		});
		if (!user && invite.loginBehaviour == 'auto_create_annon') {
			await api.SignupAnnonUser(undefined, {});
			redirect(307, conversation_url(conversation.id));
		}
		if (user && invite.status === 'accepted') {
			return redirect(307, conversation_url(conversation.id));
		}
		// Auto-redirect if user is already registered for the conversation
		if (user && participation) {
			const firstWorkflow = workflows[0];
			redirect(307, `/conversations/${conversation.id}/workflow/${firstWorkflow.id}/next`);
		}
		return { invite, conversation, user, workflows, participation };
	} catch (e) {
		if (isRedirect(e)) {
			throw e;
		}
		console.error(e);
		return { error: e.response.data.err, conversation };
	}
};
