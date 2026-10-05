import { error } from '@sveltejs/kit';
import { ADMIN_GUIDES } from '$lib/admin_guides';
import type { PageLoad } from './$types';

export const load: PageLoad = ({ params }) => {
	if (!Object.hasOwn(ADMIN_GUIDES, params.guide_id)) error(404, 'Guide not found');
	const guide = ADMIN_GUIDES[params.guide_id];
	if (!guide) error(404, 'Guide not found');
	return { guide };
};
