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
		conversationTitle: config.conversationTitle ?? undefined
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
		createdAt: allConversationsConfig.createdAt
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

/**
 * Narrows each group to the one version a conversation's participants actually receive:
 * the conversation's own version if it has one, otherwise the base row.
 * @param conversationId The conversation to show, or null to show every version.
 */
export function filterGroupsForConversation(
	groups: EmailTypeGroup[],
	conversationId: string | null
): EmailTypeGroup[] {
	if (!conversationId) return groups;

	return groups.map((group) => {
		const own = group.rows.find((row) => row.conversationId === conversationId);
		return { ...group, rows: [own ?? group.rows[0]] };
	});
}

/** The conversations that have their own version of at least one email, by title. */
export function scopedConversations(
	configs: EmailTemplateConfigWithScopeDto[]
): ScopedConversation[] {
	const byId = new Map<string, string>();
	for (const config of configs) {
		if (config.conversationId && !byId.has(config.conversationId)) {
			byId.set(config.conversationId, config.conversationTitle ?? 'Untitled conversation');
		}
	}
	return [...byId.entries()]
		.map(([id, title]) => ({ id, title }))
		.sort((a, b) => a.title.localeCompare(b.title));
}
