import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent, params }) => {
	return { conversationId: params.conversation_id };
};
