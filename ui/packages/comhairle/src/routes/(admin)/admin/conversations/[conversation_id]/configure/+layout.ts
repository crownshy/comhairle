import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ parent }) => {
	const { conversation, user } = await parent();

	return {
		isConversationOwner: user.id === conversation.ownerId
	};
};
