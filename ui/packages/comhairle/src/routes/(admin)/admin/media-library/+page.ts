import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { PageLoad } from './$types';
import { key } from '$lib/utils/invalidationKey';

export const load: PageLoad = async ({ parent, depends }) => {
	depends(key('admin/media'));

	const { api } = await parent();

	// TODO: Add in limit & pagination
	return {
		streamedMedia: tryCatchAsync(() => api.ListMedia().then((res) => res.records))
	};
};

// export const actions = {
// 	upload: async ({ request, fetch }: RequestEvent) => {
// 		const data = await request.formData();
//
// 		const media = new Media();
// 		const response = await tryCatchAsync(() =>
// 			media.upload('/api/media', data, { fetchRef: fetch, schema: MediaSchema })
// 		);
//
// 		if (response.err !== null) {
// 			return fail(HttpStatus.UnprocessableContent);
// 		}
// 	},
// 	delete: async ({ request, fetch }: RequestEvent) => {
// 		const data = await request.formData();
// 		const media = data.getAll('media') as string[];
//
// 		const results: { id: string; request: ReturnType<typeof tryFetch> }[] = [];
// 		for (const id of media) {
// 			results.push({
// 				id,
// 				request: tryFetch(`/api/media/${id}`, { method: 'DELETE' }, fetch)
// 			});
// 		}
//
// 		const failures: string[] = [];
// 		for (const result of results) {
// 			const response = await result.request;
// 			if (response.err !== null) {
// 				failures.push(`${result.id} failed to delete, ${response.err.message}`);
// 			}
// 		}
//
// 		if (failures.length > 0) {
// 			return fail(500, { failures });
// 		}
// 	}
// };
