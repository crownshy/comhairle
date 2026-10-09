import type { PageLoad } from './$types';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { canPerformAction } from '$lib/utils/permissions';
import { key } from '$lib/utils/invalidationKey';

async function collectPages<RecordType>(loadPage: (offset: number) => Promise<RecordType[]>) {
	const records: RecordType[] = [];
	for (let offset = 0; ; offset += 100) {
		const page = await loadPage(offset);
		records.push(...page);
		if (page.length < 100) return records;
	}
}

export const load: PageLoad = async ({ parent, depends }) => {
	depends(key('admin/conversation/permissions'));
	const { api, conversation, conversationActions } = await parent();
	if (!canPerformAction(conversationActions, 'conversation_admin'))
		return { roleManagement: null };
	const params = { resource_type: 'conversation', resource_id: conversation.id };
	const roleManagement = await tryCatchAsync(async () => {
		const [roles, assignments, users, organizations] = await Promise.all([
			api.GetPermissionRoles({ params: { resource_type: 'conversation' } }),
			collectPages((offset) =>
				api
					.ListResourcePermissions({ params, queries: { limit: 100, offset } })
					.then((page) => page.records)
			),
			collectPages((offset) =>
				api.ListUsersWithPermission({ params, queries: { limit: 100, offset } })
			),
			collectPages((offset) =>
				api
					.ListOrganizations({ queries: { limit: 100, offset } })
					.then((page) => page.records)
			)
		]);
		return { roles, assignments, users, organizations };
	});
	return { roleManagement };
};
