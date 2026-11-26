import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ params }) => {
	const { conversation_id } = params;

	return { conversationId: conversation_id };
};
