import { tryFetch, tryCatchAsync } from '$lib/utils/errorHandling';
import { fail } from '@sveltejs/kit';
import type { RequestEvent } from './$types';
import Media from '$lib/interfaces/Media';
import type { MediaDto } from '@crownshy/api-client/api';

export type ContentType = MediaDto['contentType'] | 'audio/mp3';
export type MockData = Pick<MediaDto, 'id'> &
	Record<'src', string> &
	Record<'contentType', ContentType>;
export async function load({ fetch }: RequestEvent) {
	const response = await tryFetch('/api/media', undefined, fetch);
	if (response.err !== null) {
		// FIX:
		return fail(500);
	}
	const data = await tryCatchAsync(() => response.ok.json());
	if (data.err !== null) {
		// FIX:
		return fail(500);
	}
	return {
		media: data.ok.records
	};
}
export const actions = {
	upload: async ({ request, fetch }: RequestEvent) => {
		const data = await request.formData();
		const files = data.getAll('media');
		if (files === null) {
			return fail(422, { failures: ["Couldn't find files"] });
		}

		const media = new Media();
		const responses = await media.upload('/api/media', files as File[], { fetchRef: fetch });

		const errors = responses.filter((r) => r.err !== null);
		if (responses.some((r) => r.err !== null)) {
			return fail(422, { failures: errors.map((e) => e.err.message) });
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
