import {
	CONVERSATION_STATUS_LABELS,
	CONVERSATION_STATUS_QUERIES
} from '$lib/utils/conversationStatus';

export const CONVERSATION_STATUS_FILTERS = ['live', 'draft', 'completed', 'shared'] as const;
export type ConversationStatusFilter = (typeof CONVERSATION_STATUS_FILTERS)[number];

export const CONVERSATION_STATUS_FILTER_LABELS: Record<ConversationStatusFilter, string> = {
	...CONVERSATION_STATUS_LABELS,
	shared: 'Shared'
};

type ConversationListQueries = {
	is_live?: boolean;
	is_complete?: boolean;
	exclude_owner_id?: string;
};

/**
 * Query params for `/user/permitted_conversations`, which returns the conversations you own
 * as well as the ones shared with you.
 */
export function conversationListQueries(
	filter: ConversationStatusFilter,
	userId: string | undefined
): ConversationListQueries {
	switch (filter) {
		case 'shared':
			return { exclude_owner_id: userId };
		default:
			return CONVERSATION_STATUS_QUERIES[filter];
	}
}

/** e.g. "1 live conversation", "3 conversations". */
export function conversationNoun(filter: ConversationStatusFilter, count: number): string {
	const noun = count === 1 ? 'conversation' : 'conversations';
	return `${CONVERSATION_STATUS_FILTER_LABELS[filter].toLowerCase()} ${noun}`;
}
