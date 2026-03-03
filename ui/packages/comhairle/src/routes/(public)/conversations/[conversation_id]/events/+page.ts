import type { PageLoad } from './$types';
import type { LocalizedEventDto } from '@crown-shy/api-client/api';

export const load: PageLoad = async ({ parent, params }) => {
	const { api } = await parent();
	const conversation_id = params.conversation_id;

	try {
		const events = await api.ListEvents({
			params: { conversation_id },
			queries: { limit: 50 }
		});

		return {
			conversationId: conversation_id,
			events: events.records as LocalizedEventDto[],
			total: events.total as number
		};
	} catch (e) {
		console.error('Failed to load events:', e);
		return {
			conversationId: conversation_id,
			events: [] as LocalizedEventDto[],
			total: 0
		};
	}
};
