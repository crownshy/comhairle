import { tryFetch, tryCatchAsync } from '$lib/utils/errorHandling';
import { fail, type LoadEvent } from '@sveltejs/kit';
import type { RequestEvent } from './$types';
import Media from '$lib/interfaces/Media';
import type { MediaDto } from '@crownshy/api-client/api';
import { message, superValidate } from 'sveltekit-superforms';
import { zod } from 'sveltekit-superforms/adapters';
import MediaSchema from '$lib/components/Media/schema';

export async function load({ fetch, depends }: LoadEvent) {
	depends('media-library:media');

	const response = await tryFetch('/api/media', undefined, fetch);
	if (response.err !== null) {
		return fail(500, { error: "Couldn't get media from the server" });
	}
	const data = await tryCatchAsync(() => response.ok.json());
	if (data.err !== null) {
		// FIX: Return JSON error
		return fail(500, { error: 'Failed to parse the response from the server' });
	}

	return {
		media: data.ok.records as MediaDto[]
	};
}

export const actions = {
	upload: async ({ request }: RequestEvent) => {
		const form = await superValidate(request, zod(MediaSchema));
		console.log(form);

		if (!form.valid) {
			return message(form, 'Please try again');
		}

		return message(form, 'uploaded');
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
