import { tryCatchAsync } from '$lib/utils/errorHandling';
import { key } from '$lib/utils/invalidationKey';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent, depends }) => {
	depends(key('admin/conversation/preview-password'));
	const { api, conversation } = await parent();
	const previewPassword = await tryCatchAsync(() =>
		api.GetConversationPreviewPassword({ params: { conversation_id: conversation.id } })
	);
	const previewPasswordStatus = previewPassword.err === null ? previewPassword.ok : null;

	if (!conversation.image) {
		return {
			streamedMedia: null,
			previewPasswordStatus
		};
	}

	return {
		streamedMedia: tryCatchAsync(() =>
			api.GetMedia({ params: { media_id: conversation.image! } })
		),
		previewPasswordStatus
	};
};
