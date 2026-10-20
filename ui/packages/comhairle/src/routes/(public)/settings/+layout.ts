import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent, depends }) => {
	depends('settings:demographics');
	const { user, api } = await parent();

	if (!user) {
		redirect(307, '/');
	}
	const [participation, conversation_settings, demographicAnswers] = await Promise.all([
		api.GetConversationsUserIsParticipatingIn(),
		api.GetAllUserConversationPreferences(),
		api.ListMyDemographicAnswers()
	]);

	return { participation, conversation_settings, user, demographicAnswers };
};
