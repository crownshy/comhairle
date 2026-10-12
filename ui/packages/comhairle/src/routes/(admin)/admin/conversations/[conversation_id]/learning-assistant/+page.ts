import { tryCatchAsync } from '$lib/utils/errorHandling';
import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { key } from '$lib/utils/invalidationKey';
import { HttpStatus } from '$lib/utils/constants';
import { resolve } from '$app/paths';
import { notifications } from '$lib/notifications.svelte';

export const load: PageLoad = async ({ depends, parent, params }) => {
	depends(key('admin/knowledge-base/documents'));

	const { api } = await parent();
	const { conversation_id } = params;

	if (!conversation_id) {
		notifications.addFlash({
			message: 'Conversation ID not found',
			priority: 'ERROR'
		});
		redirect(HttpStatus.Found, resolve('/(admin)/admin/conversations'));
	}

	return {
		streamedDocuments: tryCatchAsync(() =>
			api.ListDocuments({
				params: { conversation_id }
			})
		),
		streamedChat: tryCatchAsync(() =>
			api.GetChat({
				params: { conversation_id }
			})
		),
		streamedChatInstructions: tryCatchAsync(() =>
			api.GetConversationChatInstructions({ params: { conversation_id } })
		)
	};
};
