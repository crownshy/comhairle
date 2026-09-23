import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent, params }) => {
	const { api } = await parent();
	api.GetDefaultModerationPolicyReasons({ params: { conversation_id: params.conversation_id } });
	api.ListConversationModerationPolicies({ params: { conversation_id: params.conversation_id } });
};
