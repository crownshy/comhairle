import { tryCatchAsync } from '$lib/utils/errorHandling';
import { redirect, type LoadEvent } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { key } from '$lib/utils/invalidationKey';
import type { ApiClient } from '@crownshy/api-client/api';
import { HttpStatus } from '$lib/utils/constants';
import { resolve } from '$app/paths';
import { notifications } from '$lib/notifications.svelte';

export const load: PageLoad = async ({ depends, parent, params }: LoadEvent) => {
	depends(key('knowledge-base/documents'));

	// Not sure why but this type seems to not be coming through it says api is "any"
	const { api: parentApi } = await parent();
	const api = parentApi as ApiClient;

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
		)
	};
};
