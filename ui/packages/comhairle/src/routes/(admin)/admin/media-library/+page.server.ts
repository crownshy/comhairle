import type { RequestEvent } from './$types';

export const actions = {
	upload: async ({ request }: RequestEvent) => {
		const data = await request.formData();
		console.log('data:', data);
	}
};
