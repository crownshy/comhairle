import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { LoadEvent } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ depends, params, parent }: LoadEvent) => {
	const { api } = await parent();
	depends('knowledge-base:documents');

	const { conversation_id } = params;
	if (!conversation_id) {
		return;
	}

	const docsResponse = await tryCatchAsync(() =>
		api.ListDocuments({
			params: { conversation_id }
		})
	);

	if (docsResponse.err !== null) {
		console.error(docsResponse.err);
		return;
	}

	const chatResponse = await tryCatchAsync(() =>
		api.GetChat({
			params: { conversation_id }
		})
	);

	if (chatResponse.err !== null) {
		console.error(chatResponse.err);
		return;
	}

	return { documents: docsResponse.ok, chat: chatResponse.ok };
};
