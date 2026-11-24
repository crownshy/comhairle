import type { ApiClient, LocalizedConversationDto } from '@crownshy/api-client/api';
import type { BadgeVariant } from '$lib/components/ui/badge';

export const statuses = ['live', 'draft', 'completed'] as const;
export type Status = (typeof statuses)[number];

export type Queries = NonNullable<
	NonNullable<Parameters<ApiClient['ListConversations']>[0]>['queries']
>;

export const CONVERSATION_STATUS_BADGE_VARIANTS: Record<ConversationStatus, BadgeVariant> = {
	live: 'default',
	draft: 'draft',
	completed: 'secondary'
};

// A finished conversation usually keeps `isLive`, so completed wins over live.
// The function and the API filters below must agree on that rule.
export function getStatus(conversation: LocalizedConversationDto): Status {
	if (conversation.isComplete) {
		return 'completed';
	}
	if (conversation.isLive) {
		return 'live';
	}
	return 'draft';
}

type Params = Pick<Queries, 'is_live' | 'is_complete'>;

export function getParams(status: Status): Params {
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
