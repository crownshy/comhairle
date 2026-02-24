import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent }) => {
	const { user, api } = await parent();

	if (!user) {
		redirect(307, '/');
	}
	try {
		const participation = await api.GetConversationsUserIsParticipatingIn();
		const conversation_settings = await api.GetAllUserConversationPreferences();

		return { participation, conversation_settings, user };
	} catch (e) {
		return { error: e };
	}
};
