import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params }) => {
	return {
		eventId: params.event_id
	};
};
