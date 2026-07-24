import { tryCatchAsync, tryFetch } from '$lib/utils/errorHandling';
import type { LoadEvent } from '@sveltejs/kit';
import type { ComhairleDocument } from '@crownshy/api-client/api';

export async function load({ depends, params, fetch }: LoadEvent) {
	depends('knowledge-base:documents');

	const { conversation_id } = params;

	if (!conversation_id) {
		return;
	}
	const response = await tryFetch(
		`/api/conversation/${conversation_id}/documents`,
		undefined,
		fetch
	);

	if (response.err !== null) {
		console.error(response.err);
		return;
	}

	const documents = await tryCatchAsync(() => response.ok.json());
	if (documents.err !== null) {
		console.error(documents.err);
		return;
	}

	return { documents: documents.ok as ComhairleDocument[] };
}
