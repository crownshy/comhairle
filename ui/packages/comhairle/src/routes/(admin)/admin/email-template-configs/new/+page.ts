import type { PageLoad } from '../../$types';

export const load: PageLoad = async ({ parent }) => {
	const { api } = await parent();

	try {
		const schemas = await api.ListEmailSlotSchemas();

		return { schemas };
	} catch (e) {
		console.error(e);

		return {
			schemas: [],
			error: e.response?.data?.err || 'Something went wrong loading email schemas'
		};
	}
};
