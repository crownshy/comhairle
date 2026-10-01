import type { PageLoad } from './$types';
import { canPerformAction } from '$lib/utils/permissions';
import { loadRoleManagement } from '$lib/components/permissions/roleAssignments';
import { key } from '$lib/utils/invalidationKey';

export const load: PageLoad = async ({ parent, depends }) => {
	depends(key('admin/conversation/permissions'));
	const { api, conversation, conversationActions } = await parent();
	if (!canPerformAction(conversationActions, 'conversation_admin'))
		return { roleManagement: null };
	const roleManagement = await loadRoleManagement(api, 'conversation', conversation.id);
	return { roleManagement };
};
