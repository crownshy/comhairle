import type { PageLoad } from '../../$types';

export const load: PageLoad = async ({ parent }) => {
	const { api } = await parent();

	try {
		const schemas = await api.ListEmailTemplateSchemas();
		const conversations = await api.GetOwnedConversations({ queries: { limit: 1000 } });
		const emailConfigs = await api.ListEmailTemplateConfigs();

		return { schemas, conversations: conversations.records, emailConfigs };
	} catch (e) {
		console.error(e);

		return {
			schemas: [],
			conversations: [],
			error: e.response?.data?.err || 'Something went wrong loading email schemas'
		};
	}
};
