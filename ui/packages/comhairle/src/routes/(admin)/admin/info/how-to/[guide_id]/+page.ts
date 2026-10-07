import { error, redirect } from '@sveltejs/kit';
import { resolve } from '$app/paths';
import { ADMIN_GUIDES } from '$lib/admin_guides';
import type { PageLoad } from './$types';

export const load: PageLoad = ({ params }) => {
	if (!Object.hasOwn(ADMIN_GUIDES, params.guide_id)) error(404, 'Guide not found');
	const guide = ADMIN_GUIDES[params.guide_id];
	redirect(
		307,
		resolve('/admin/info/how-to/[guide_id]/[topic_id]', {
			guide_id: guide.key,
			topic_id: guide.topics[0].key
		})
	);
};
