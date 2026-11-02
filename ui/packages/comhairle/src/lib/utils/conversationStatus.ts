import type { ConversationDto } from '@crownshy/api-client/api';
import type { BadgeVariant } from '$lib/components/ui/badge';

export type ConversationStatus = 'live' | 'draft' | 'completed';

export const CONVERSATION_STATUS_LABELS: Record<ConversationStatus, string> = {
	live: 'Live',
	draft: 'Draft',
	completed: 'Completed'
};

export const CONVERSATION_STATUS_BADGE_VARIANTS: Record<ConversationStatus, BadgeVariant> = {
	live: 'default',
	draft: 'draft',
	completed: 'secondary'
};

// A finished conversation usually keeps `isLive`, so completed wins over live.
// The function and the API filters below must agree on that rule.
export function conversationStatus(
	conversation: Pick<ConversationDto, 'isLive' | 'isComplete'>
): ConversationStatus {
	if (conversation.isComplete) return 'completed';
	return conversation.isLive ? 'live' : 'draft';
}

export const CONVERSATION_STATUS_QUERIES: Record<
	ConversationStatus,
	{ is_live?: boolean; is_complete: boolean }
> = {
	live: { is_live: true, is_complete: false },
	draft: { is_live: false, is_complete: false },
	completed: { is_complete: true }
};
