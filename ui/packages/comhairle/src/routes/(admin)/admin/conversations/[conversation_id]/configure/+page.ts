import type { PageLoad } from '../design/$types';

export const load: PageLoad = async ({ parent }) => {
	const { conversation, workflows, api } = await parent();

	try {
		// TODO: filter by image types
		const results = await api.ListMedia();

		return { conversation, workflows, media: results.records };
	} catch (e) {
		console.error(e);

		return { conversation, workflows, media: [] };
	}
};
