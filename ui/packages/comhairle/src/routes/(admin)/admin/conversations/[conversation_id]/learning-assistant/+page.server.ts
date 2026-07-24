import { apiClient } from '@crownshy/api-client/client';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { LoadEvent } from '@sveltejs/kit';
import type { ComhairleDocument } from '@crownshy/api-client/api';

export async function load({ depends, params }: LoadEvent) {
	depends('knowledge-base:documents');

	let documents: ComhairleDocument[] = [];

	const { conversation_id } = params;

	if (conversation_id) {
		const response = await tryCatchAsync(() =>
			apiClient.ListDocuments({
				params: { conversation_id: conversation_id }
			})
		);

		if (response.err !== null) {
			console.error(response.err);
		} else {
			documents = response.ok;
		}
	}

	return { documents };
}
