import type { ConversationDto } from '@crownshy/api-client/api';

export type ConversationStatus = 'live' | 'draft' | 'completed';

export const conversationStatusLabels: Record<ConversationStatus, string> = {
	live: 'Live',
	draft: 'Draft',
	completed: 'Completed'
};

export function conversationStatus(
	conversation: Pick<ConversationDto, 'isLive' | 'isComplete'>
): ConversationStatus {
	if (conversation.isComplete) return 'completed';
	return conversation.isLive ? 'live' : 'draft';
}
