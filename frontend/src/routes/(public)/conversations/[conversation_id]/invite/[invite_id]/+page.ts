import { redirect } from "@sveltejs/kit";
import type { PageLoad } from "./$types"
import { conversation_url } from "$lib/urls";

export const load: PageLoad = async ({ params, parent }) => {
	// const offset = event.url.searchParams.get('offset') || '0';
	let { api, conversation, user } = await parent();
	let { invite_id } = params;

	try {
		const invite = await api.GetInvite({ params: { conversation_id: conversation.id, invite_id } });
		if (!user && invite.login_behaviour == "auto_create_annon") {
			await api.SignupAnnonUser(undefined, {})
			redirect(307, conversation_url(conversation.id))
		}
		return { invite, conversation, user };
	}
	catch (e) {
		return { error: e.response.data.err, conversation }
	}
}
