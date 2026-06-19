import type { PageLoad } from './$types';
import { redirect } from '@sveltejs/kit';

export const load: PageLoad = async ({ parent }) => {
	const { api } = await parent();

	try {
		const emailConfigs = await api.ListEmailTemplateConfigs();

		return { emailConfigs };
	} catch (e) {
		console.error(e);

		redirect(302, '/admin');
	}
};
