import { tryFetch, tryCatchAsync } from '$lib/utils/errorHandling';
import { fail, type LoadEvent } from '@sveltejs/kit';
import type { RequestEvent } from './$types';
import type { MediaDto } from '@crownshy/api-client/api';
import MediaSchema from '$lib/schemas/MediaSchema';
import Media from '$lib/interfaces/Media';
import { HttpStatus } from '$lib/utils/constants';

export async function load({ fetch, depends }: LoadEvent) {
	depends('media-library:media');

	const response = await tryFetch('/api/media', undefined, fetch);
	if (response.err !== null) {
		return fail(HttpStatus.InternalServerError, {
			error: "Couldn't get media from the server"
		});
	}
	const data = await tryCatchAsync(() => response.ok.json());
	if (data.err !== null) {
		// FIX: Return JSON error
		return fail(HttpStatus.InternalServerError, {
			error: 'Failed to parse the response from the server'
		});
	}

	return {
		media: data.ok.records as MediaDto[]
	};
}

export const actions = {
	upload: async ({ request, fetch }: RequestEvent) => {
		const data = await request.formData();

		const media = new Media();
		const response = await tryCatchAsync(() =>
			media.upload('/api/media', data, { fetchRef: fetch, schema: MediaSchema })
		);

		if (response.err !== null) {
			return fail(HttpStatus.UnprocessableContent);
		}
	},
	delete: async ({ request, fetch }: RequestEvent) => {
		const data = await request.formData();
		const media = data.getAll('media') as string[];

		const results: { id: string; request: ReturnType<typeof tryFetch> }[] = [];
		for (const id of media) {
			results.push({
				id,
				request: tryFetch(`/api/media/${id}`, { method: 'DELETE' }, fetch)
			});
		}

		const failures: string[] = [];
		for (const result of results) {
			const response = await result.request;
			if (response.err !== null) {
				failures.push(`${result.id} failed to delete, ${response.err.message}`);
			}
		}

		if (failures.length > 0) {
			return fail(500, { failures });
		}
	}
};
