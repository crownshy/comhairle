import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { LoadEvent } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { apiClient } from '@crownshy/api-client/client';

export const load: PageLoad = async ({ depends, params }: LoadEvent) => {
	depends('knowledge-base:documents');

	const { conversation_id } = params;
	if (!conversation_id) {
		return;
	}

	const response = await tryCatchAsync(() =>
		apiClient.ListDocuments({
			params: { conversation_id }
		})
	);

	if (response.err !== null) {
		console.error(response.err);
		return;
	}

	return { documents: response.ok };
};
