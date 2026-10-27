import { error, redirect } from '@sveltejs/kit';
import { resolve } from '$app/paths';
import { ADMIN_GUIDES } from '$lib/admin_guides';
import type { EntryGenerator, PageServerLoad } from './$types';
import { HttpStatus } from '$lib/utils/constants';

// Not linked anywhere (the nav links straight to a guide's first topic), so the
// prerender crawler never reaches it on its own.
export const entries: EntryGenerator = () =>
	Object.keys(ADMIN_GUIDES).map((guide_id) => ({ guide_id }));

export const load: PageServerLoad = ({ params }) => {
	if (!Object.hasOwn(ADMIN_GUIDES, params.guide_id))
		error(HttpStatus.NotFound, 'Guide not found');
	const guide = ADMIN_GUIDES[params.guide_id];
	redirect(
		HttpStatus.Found,
		resolve('/(admin)/admin/info/how-to/[guide_id]/[topic_id]', {
			guide_id: guide.key,
			topic_id: guide.topics[0].key
		})
	);
};
