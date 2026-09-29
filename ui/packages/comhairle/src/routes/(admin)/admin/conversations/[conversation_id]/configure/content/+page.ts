import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent, params }) => {
	const { api } = await parent();
	const { conversation_id } = params;

	return {
		streamedAvailableDocuments: tryCatchAsync(() =>
			api
				.ListDocuments({
					params: { conversation_id }
				})
				.then((result) => result.filter((d) => d.parse_status === 'DONE') ?? [])
		)
	};
};
