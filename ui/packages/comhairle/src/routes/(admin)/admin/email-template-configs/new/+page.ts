import type {
	EmailTemplateConfigWithScopeDto,
	EmailTypeSchema,
	LocalizedConversationDto,
	LocalizedEventDto
} from '@crownshy/api-client/api';
import type { PageLoad } from '../../$types';

type Result = {
	schemas: EmailTypeSchema[];
	conversations: LocalizedConversationDto[];
	events: LocalizedEventDto[];
	emailConfigs: EmailTemplateConfigWithScopeDto[];
	error?: string;
};

export const load: PageLoad = async ({ parent }): Promise<Result> => {
	const { api } = await parent();

	try {
		const schemas = await api.ListEmailTemplateSchemas();
		const conversations = await api.GetOwnedConversations({ queries: { limit: 1000 } });
		const events = await api.GetOwnedEvents({ queries: { limit: 1000 } });
		const emailConfigs = await api.ListEmailTemplateConfigs();

		return {
			schemas,
			conversations: conversations.records,
			events: events.records,
			emailConfigs
		};
	} catch (e) {
		console.error(e);

		return {
			schemas: [],
			conversations: [],
			events: [],
			emailConfigs: [],
			error: e.response?.data?.err || 'Something went wrong loading email schemas'
		};
	}
};
