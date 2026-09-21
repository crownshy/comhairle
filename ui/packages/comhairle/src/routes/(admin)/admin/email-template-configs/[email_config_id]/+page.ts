import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { key } from '$lib/utils/invalidationKey';

export const load: PageLoad = async ({ params, parent, depends }) => {
	depends(key('email-template-config'));
	const { api } = await parent();
	const { email_config_id } = params;

	try {
		const emailConfig = await api.GetEmailTemplateConfig({ params: { email_config_id } });
		const schema = await api.GetEmailTemplateSchema({ params: { email_config_id } });

		return { emailConfig, schema };
	} catch (e) {
		console.error(e);

		redirect(302, '/admin/email-template-configs');
	}
};
