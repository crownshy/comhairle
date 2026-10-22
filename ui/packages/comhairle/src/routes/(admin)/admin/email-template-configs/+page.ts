import type { PageLoad } from './$types';
import { redirect } from '@sveltejs/kit';
import { key } from '$lib/utils/invalidationKey';
import { tryCatchAsync } from '$lib/utils/errorHandling';

export const load: PageLoad = async ({ parent, depends }) => {
	depends(key('admin/email-template-config'));
	const { api } = await parent();

	const result = await tryCatchAsync(() =>
		Promise.all([api.ListEmailTemplateConfigs(), api.ListEmailTemplateSchemas()])
	);
	if (result.err !== null) redirect(302, '/admin');

	const [emailConfigs, schemas] = result.ok;
	return { emailConfigs, schemas };
};
