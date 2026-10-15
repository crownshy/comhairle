import type { createApiClient } from '@crownshy/api-client/client';
import type { LocalizedConversationDto } from '@crownshy/api-client/api';

type ConversationListApi = Pick<
	ReturnType<typeof createApiClient>,
	'GetOwnedConversations' | 'GetPermittedConversations'
>;

export type AdminConversations = {
	owned: LocalizedConversationDto[];
	shared: LocalizedConversationDto[];
};

export const LATEST_CONVERSATION_COUNT = 5;

const noConversations = { records: [], total: 0 };

/**
 * The permitted list also returns conversations you own, so shared is permitted minus owned.
 * Both endpoints sort newest created first, which is what makes `limit` mean "latest".
 */
export async function loadAdminConversations(
	api: ConversationListApi,
	limit?: number
): Promise<AdminConversations> {
	const request = limit === undefined ? undefined : { queries: { limit } };
	const [owned, permitted] = await Promise.all([
		api.GetOwnedConversations(request).catch(() => noConversations),
		api.GetPermittedConversations(request).catch(() => noConversations)
	]);
	const ownedIds = new Set(owned.records.map((conversation) => conversation.id));

	return {
		owned: owned.records,
		shared: permitted.records.filter((conversation) => !ownedIds.has(conversation.id))
	};
}

type DatedConversation = Pick<LocalizedConversationDto, 'createdAt'>;

export function latestConversations<T extends DatedConversation>(
	conversations: { owned: T[]; shared: T[] },
	count = LATEST_CONVERSATION_COUNT
): T[] {
	return [...conversations.owned, ...conversations.shared]
		.sort((a, b) => Date.parse(b.createdAt) - Date.parse(a.createdAt))
		.slice(0, count);
}
