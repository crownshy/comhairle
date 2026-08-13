import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent }) => {
	const parentData = await parent();
	const ownedConversations = parentData?.ownedConversations ?? { records: [] };
	const permittedConversations = parentData?.permittedConversations ?? { records: [] };

	return { ownedConversations, permittedConversations };
};
