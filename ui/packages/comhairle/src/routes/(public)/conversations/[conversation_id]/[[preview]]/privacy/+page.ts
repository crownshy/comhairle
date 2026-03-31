import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent }) => {
	const { conversation } = await parent();

	return { conversation };
};
