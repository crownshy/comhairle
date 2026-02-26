import type { PageLoad } from '../design/$types';

export const load: PageLoad = async ({ parent }) => {
	const { conversation, api } = await parent();

	try {
		const events = await api.ListEvents({
			params: { conversation_id: conversation.id }
		});

		return { conversation, events };
	} catch (e) {
		console.error(e);
		return;
	}
};
