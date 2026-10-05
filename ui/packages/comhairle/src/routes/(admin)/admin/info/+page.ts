import { redirect } from '@sveltejs/kit';
import { resolve } from '$app/paths';
import { ADMIN_GUIDE_NAV } from '$lib/admin_guides';
import type { PageLoad } from './$types';

export const load: PageLoad = () => {
	const firstGuide = ADMIN_GUIDE_NAV[0];
	const firstTopic = firstGuide.topics[0];

	redirect(
		307,
		resolve('/admin/info/how-to/[guide_id]/[topic_id]', {
			guide_id: firstGuide.key,
			topic_id: firstTopic.key
		})
	);
};
