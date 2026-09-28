import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent }) => {
	const { conversation, api } = await parent();
	return {
		streamedUsersAndPermissions: tryCatchAsync(() =>
			api.ListUsersWithPermission({
				params: {
					resource_type: 'conversation',
					resource_id: conversation.id
				},
				queries: { role_name: 'content_editor' }
			})
		)
	};
};
