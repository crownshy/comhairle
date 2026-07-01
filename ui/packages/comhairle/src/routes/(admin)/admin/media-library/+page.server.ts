import { tryFetch } from '$lib/utils/errorHandling';
import { fail, json } from '@sveltejs/kit';
import type { RequestEvent } from './$types';

export const actions = {
	upload: async ({ request, fetch }: RequestEvent) => {
		const data = await request.formData();

		const response = await tryFetch(
			'/api/media',
			{
				method: 'POST',
				body: data
			},
			fetch
		);

		console.error('response:', response);

		if (response.err !== null) {
			switch (response.err.id) {
				case 'NETWORK_ERROR':
					return fail(400, { error: response.err.message });
				case 'HTTP_ERROR':
					return fail(response.err.status, { error: response.err.message });
			}
		}
		return json({ message: 'ok' });
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

		const failed: [string, string][] = [];
		for (const result of results) {
			const response = await result.request;
			if (response.err !== null) {
				failed.push([result.id, response.err.message]);
			}
		}

		if (failed.length > 0) {
			return fail(500, { failed });
		}

		return new Response('ok', { status: 200 });
	}
};
