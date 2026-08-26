import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { LoadEvent } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { apiClient } from '@crownshy/api-client/client';
import { key } from '$lib/utils/invalidationKey';

export const load: PageLoad = async ({ depends, params }: LoadEvent) => {
	depends(key('knowledge-base/documents'));

	const { conversation_id } = params;
	if (!conversation_id) {
		return;
	}

	const docsResponse = await tryCatchAsync(() =>
		apiClient.ListDocuments({
			params: { conversation_id }
		})
	);

	if (docsResponse.err !== null) {
		console.error(docsResponse.err);
		return;
	}

	const chatResponse = await tryCatchAsync(() =>
		apiClient.GetChat({
			params: { conversation_id }
		})
	);

	if (chatResponse.err !== null) {
		console.error(chatResponse.err);
		return;
	}

	return { documents: docsResponse.ok, chat: chatResponse.ok };
};
