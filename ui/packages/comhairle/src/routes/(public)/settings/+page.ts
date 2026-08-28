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
		const [questionsRes, responsesRes] = await Promise.all([
			api
				.GetDemographicsQuestions({ queries: { limit: 100 } })
				.catch(() => ({ records: [] })),
			api
				.GetDemographicsResponses({ queries: { user_id: user.id, limit: 100 } })
				.catch(() => ({ records: [] }))
		]);

		return {
			participation,
			conversation_settings,
			user,
			demographicQuestions: questionsRes.records || [],
			demographicResponses: responsesRes.records || []
		};
	} catch (e) {
		return { error: e };
	}
};
