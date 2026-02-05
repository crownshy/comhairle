import { apiClient } from '$lib/api/client';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent }) => {
	const { conversation } = await parent();
	let documents;
	try {
		documents = await apiClient.ListDocuments({
			params: { conversation_id: conversation.id }
		});
	} catch (e) {
		console.error(e);
	}
	return { documents, conversation };
};
