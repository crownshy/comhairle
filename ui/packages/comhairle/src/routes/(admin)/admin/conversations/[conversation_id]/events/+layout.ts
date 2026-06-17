import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ parent, depends }) => {
	depends('conversation:events');

	const { conversation, api } = await parent();

	const events = await api.ListEvents({
		params: { conversation_id: conversation.id },
		queries: { created_at: 'desc' }
	});

	return { conversation, events };
};
