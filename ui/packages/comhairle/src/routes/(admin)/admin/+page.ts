import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent }) => {
	let { ownedConversations } = await parent();

	return { conversations: ownedConversations };
};
