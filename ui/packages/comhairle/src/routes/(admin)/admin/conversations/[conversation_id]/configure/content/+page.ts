import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { PageLoad } from './$types';
import { key } from '$lib/utils/invalidationKey';

export const load: PageLoad = async ({ parent, params, depends }) => {
	depends(key('conversation/documents'));

	const { api } = await parent();
	const { conversation_id } = params;

	const documents = await tryCatchAsync(() =>
		api.ListDocuments({
			params: { conversation_id }
		})
	);

	if (documents.err !== null) {
		console.warn('failed to load conversation documents', documents.err);
	}

	const availableDocuments = documents.ok?.filter((d) => d.parse_status === 'DONE') ?? [];

	return {
		availableDocuments
	};
};
