import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { MediaDto, UserWithPermissionDto } from '@crownshy/api-client/api';
import type { LayoutLoad } from './$types';

/**
 * The Content tab's rich fields (FAQ, thank-you, privacy policy, short privacy policy) offer an
 * "Insert Source Document" control. It needs the conversation's parsed knowledge base documents to
 * populate the picker, and it needs them again on the render path to resolve each badge's name/size
 * and download link. We fetch them once here (page-scoped so unrelated conversation sub-pages don't
 * pay for it) and only surface the DONE-parsed ones, matching the Learn step path. A failed fetch
 * falls back to an empty list, so the picker shows its empty state rather than a raw backend error.
 */
export const load: LayoutLoad = async ({ parent }) => {
	const { api, conversation, user } = await parent();

	let usersWithPermission: UserWithPermissionDto[] = [];
	let isConversationOwner = false;

	if (user.id === conversation.ownerId) {
		isConversationOwner = true;
		const result = await tryCatchAsync(() =>
			api.ListUsersWithPermission({
				params: {
					resource_type: 'conversation',
					resource_id: conversation.id
				},
				queries: { role_name: 'content_editor' }
			})
		);
		usersWithPermission = result.ok ?? [];
	}

	return {
		isConversationOwner,
		usersWithPermission
	};
};
