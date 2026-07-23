import { apiClient } from '@crownshy/api-client/client';
import type { PageLoad } from './$types';
import { tryCatchAsync } from '$lib/utils/errorHandling';

export const load: PageLoad = async ({ parent, depends }) => {
	depends('knowledge-base:documents');

	const { conversation } = await parent();

	const response = await tryCatchAsync(() =>
		apiClient.ListDocuments({
			params: { conversation_id: conversation.id }
		})
	);

	if (response.err !== null) {
		console.error(response.err);
	}

	return { documents: response.ok ?? [], conversation };
};
