import { tryFetch } from '$lib/utils/errorHandling';
import { fail, json } from '@sveltejs/kit';
import type { RequestEvent } from './$types';

export const actions = {
	upload: async ({ request, fetch }: RequestEvent) => {
		const data = await request.formData();

		const response = await tryFetch(() =>
			fetch('/api/media', {
				method: 'POST',
				body: data
			})
		);

		console.error('response:', response);

		if (response.err !== null) {
			return fail(response.err.status, { error: response.err.message });
		}
		return json({ message: 'ok' });
	},
	delete: async ({ request }: RequestEvent) => {
		const data = await request.formData();
		console.log('data:', data);
	}
};
