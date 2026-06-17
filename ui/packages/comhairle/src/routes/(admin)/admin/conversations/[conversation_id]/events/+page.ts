import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent }) => {
	const { conversation, events } = await parent();
	return { conversation, events };
};
