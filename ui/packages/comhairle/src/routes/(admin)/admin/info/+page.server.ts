import { redirect } from '@sveltejs/kit';
import { resolve } from '$app/paths';
import { ADMIN_GUIDE_NAV } from '$lib/admin_guides';
import type { PageServerLoad } from './$types';
import { HttpStatus } from '$lib/utils/constants';

export const load: PageServerLoad = () => {
	const firstGuide = ADMIN_GUIDE_NAV[0];
	const firstTopic = firstGuide.topics[0];

	redirect(
		HttpStatus.Found,
		resolve('/(admin)/admin/info/how-to/[guide_id]/[topic_id]', {
			guide_id: firstGuide.key,
			topic_id: firstTopic.key
		})
	);
};
