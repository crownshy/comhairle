import type { PageLoad } from './$types';
import { redirect } from '@sveltejs/kit';
import { tryCatchAsync } from '$lib/utils/errorHandling';

export const load: PageLoad = async ({ parent }) => {
	const { api } = await parent();

	const result = await tryCatchAsync(() =>
		Promise.all([
			api.ListEmailTemplateSchemas(),
			api.GetOwnedConversations({ queries: { limit: 1000 } }),
			api.ListEmailTemplateConfigs()
		])
	);
	if (result.err !== null) redirect(302, '/admin/email-template-configs');

	const [schemas, conversations, emailConfigs] = result.ok;
	return { schemas, conversations: conversations.records, emailConfigs };
};
