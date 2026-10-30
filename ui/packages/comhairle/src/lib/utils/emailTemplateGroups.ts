import type {
	EmailTemplateConfigWithScopeDto,
	EmailType,
	EmailTypeSchema
} from '@crownshy/api-client/api';

/**
 * Which version of an email a row stands for: one conversation's own version, the
 * version for all conversations, or Comhairle's built-in default.
 */
export type TemplateSource = 'conversation' | 'all' | 'default';

export type EmailTemplateRow = {
	/** Stable key for keyed `each` blocks. */
	key: string;
	source: TemplateSource;
	subject: string;
	/** The saved custom version. Absent for Comhairle's default. */
	configId?: string;
	createdAt?: string;
	conversationId?: string;
	conversationTitle?: string;
	/** The saved wording. Absent for Comhairle's default, which comes from the schema. */
	slots?: EmailTemplateConfigWithScopeDto['slots'];
};

export type EmailTypeGroup = {
	emailType: EmailType;
	/** The base row (custom for all, or the default) first, then per-conversation rows. */
	rows: EmailTemplateRow[];
};

export type ScopedConversation = { id: string; title: string };

/**
 * Only invites can be given a per-conversation version. This matches the new-template
 * form, which only offers a conversation picker for invites.
 */
export function canScopeToConversation(emailType: EmailType): boolean {
	return emailType === 'conversation_invite';
}

function conversationRow(
	config: EmailTemplateConfigWithScopeDto,
	schema: EmailTypeSchema
): EmailTemplateRow {
	return {
		key: config.id,
		source: 'conversation',
		subject: config.subject ?? schema.default_subject,
		configId: config.id,
		createdAt: config.createdAt,
		conversationId: config.conversationId ?? undefined,
		conversationTitle: config.conversationTitle ?? undefined,
		slots: config.slots
	};
}

function baseRow(
	schema: EmailTypeSchema,
	allConversationsConfig: EmailTemplateConfigWithScopeDto | undefined
): EmailTemplateRow {
	if (!allConversationsConfig) {
		return {
			key: `${schema.email_type}-default`,
			source: 'default',
			subject: schema.default_subject
		};
	}
	return {
		key: allConversationsConfig.id,
		source: 'all',
		subject: allConversationsConfig.subject ?? schema.default_subject,
		configId: allConversationsConfig.id,
		createdAt: allConversationsConfig.createdAt,
		slots: allConversationsConfig.slots
	};
}

/**
 * One group per email type, in schema order, so types nobody has customised still show
 * with Comhairle's default.
 * @param schemas Every email type the platform sends.
 * @param configs The custom versions saved so far.
 */
export function buildEmailTypeGroups(
	schemas: EmailTypeSchema[],
	configs: EmailTemplateConfigWithScopeDto[]
): EmailTypeGroup[] {
	return schemas.map((schema) => {
		const configsOfType = configs.filter((config) => config.emailType === schema.email_type);
		const allConversationsConfig = configsOfType.find((config) => !config.conversationId);
		const perConversation = configsOfType
			.filter((config) => !!config.conversationId)
			.map((config) => conversationRow(config, schema))
			.sort((a, b) => (a.conversationTitle ?? '').localeCompare(b.conversationTitle ?? ''));

		return {
			emailType: schema.email_type,
			rows: [baseRow(schema, allConversationsConfig), ...perConversation]
		};
	});
}

/** A conversation, as far as the email list needs it. */
export type ConversationSummary = { id: string; title: string };

export type EmailTypeView = {
	emailType: EmailType;
	/** What every conversation sends unless it has its own: the custom default, or Comhairle's. */
	base: EmailTemplateRow;
	/** Conversations with their own version, by title. */
	conversationVersions: EmailTemplateRow[];
	/** Conversations that send the base version. Always empty for event emails. */
	usingBase: ConversationSummary[];
};

/**
 * What one email type's tab shows: the default, the conversations with their own version,
 * and the conversations that fall back to the default.
 */
export function buildEmailTypeView(
	schema: EmailTypeSchema,
	configs: EmailTemplateConfigWithScopeDto[],
	conversations: ConversationSummary[]
): EmailTypeView {
	const [base, ...conversationVersions] = buildEmailTypeGroups([schema], configs)[0].rows;
	const customised = new Set(conversationVersions.map((row) => row.conversationId));
	const usingBase = canScopeToConversation(schema.email_type)
		? conversations
				.filter((conversation) => !customised.has(conversation.id))
				.map(({ id, title }) => ({ id, title }))
		: [];

	return { emailType: schema.email_type, base, conversationVersions, usingBase };
}
