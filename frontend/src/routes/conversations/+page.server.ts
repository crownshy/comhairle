import { redirect } from '@sveltejs/kit';
import { setSearch } from './utils';

export const actions = {
	search: async ({ request }) => {
		const data = await request.formData();
		const search = data.get('search');

		const url = setSearch(request.url, search);
		url.searchParams.delete('/search');

		return redirect(302, url.toString());
	}
};
