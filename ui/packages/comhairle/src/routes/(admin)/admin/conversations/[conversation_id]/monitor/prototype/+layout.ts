import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ params, parent, depends }) => {
	return { conversationId: params.conversation_id };
};
