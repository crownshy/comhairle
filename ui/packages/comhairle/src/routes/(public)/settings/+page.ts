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

		const localizedParticipation = await Promise.all(
			participation.map(async (c) => {
				try {
					const localized = await api.GetConversation({
						params: { conversation_id: c.id }
					});
					return { id: c.id, title: localized.title };
				} catch {
					return { id: c.id, title: c.id };
				}
			})
		);

		return {
			participation: localizedParticipation,
			conversation_settings,
			user
		};
	} catch (e) {
		return { error: e };
	}
};
