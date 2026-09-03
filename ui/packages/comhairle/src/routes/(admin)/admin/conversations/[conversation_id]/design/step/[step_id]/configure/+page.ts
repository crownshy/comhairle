import { key } from '$lib/utils/invalidationKey';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent, params, depends }) => {
	depends(key('design/step/configure/documents'));

	const { api } = await parent();
	const { conversation_id } = params;

	return {
		streamedAvailableDocuments: tryCatchAsync(() =>
			api
				.ListDocuments({ params: { conversation_id } })
				.then((docs) => docs.filter((d) => d.parse_status === 'DONE'))
		)
	};
};
