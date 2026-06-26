import { tryFetch } from '$lib/utils/errorHandling';
import { fail } from '@sveltejs/kit';
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
		if (response.err !== null) {
			return fail(response.err.status, { error: response.err.message });
		}
	},
	delete: async ({ request }: RequestEvent) => {
		const data = await request.formData();
		console.log('data:', data);
	}
};
