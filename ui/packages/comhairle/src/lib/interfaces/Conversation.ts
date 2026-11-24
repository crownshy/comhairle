import type { ApiClient, LocalizedConversationDto } from '@crownshy/api-client/api';

export const statuses = ['live', 'draft', 'completed'] as const;
export type ConversationStatus = (typeof statuses)[number];

export type Queries = NonNullable<
	NonNullable<Parameters<ApiClient['ListConversations']>[0]>['queries']
>;

type ConversationParam = Pick<LocalizedConversationDto, 'isLive' | 'isComplete'> & {
	[key: string]: unknown;
};

// A finished conversation usually keeps `isLive`, so completed wins over live.
// The function and the API filters below must agree on that rule.
export function getStatus(conversation: ConversationParam): ConversationStatus {
	if (conversation.isComplete) {
		return 'completed';
	}
	if (conversation.isLive) {
		return 'live';
	}
	return 'draft';
}

type Params = Pick<Queries, 'is_live' | 'is_complete'>;

export function getParams(status: ConversationStatus): Params {
	switch (status) {
		case 'live':
			return { is_live: true, is_complete: false };
		case 'draft':
			return { is_live: false, is_complete: false };
		case 'completed':
			return { is_live: undefined, is_complete: false };
	}
}

export default {
	statuses,
	getStatus,
	getParams
};
