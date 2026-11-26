import Media from '$lib/interfaces/Media.svelte';
import QueryParams from '$lib/interfaces/QueryParams.svelte';
import type { MediaLibraryParams } from '$lib/interfaces/QueryParams.types';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { key } from '$lib/utils/invalidationKey';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent, depends, url }) => {
	depends(key('admin/media-library/media'));

	const p = await parent();

	const filter = QueryParams.get<MediaLibraryParams>('filter', url);

	// FIX: Change to query on the backend
	return {
		media: await tryCatchAsync(() =>
			p.api
				.ListMedia()
				.then((res) =>
					res.records.filter((record) =>
						!filter
							? true
							: Media.mediaTypeFromContentType(record.contentType) === filter
					)
				)
		)
	};
};
