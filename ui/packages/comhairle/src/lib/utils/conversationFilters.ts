import type { ConversationDto } from '@crownshy/api-client/api';
import { conversationStatus, conversationStatusLabels } from './conversationStatus';

export const conversationStatusFilters = ['all', 'live', 'draft', 'completed', 'shared'] as const;
export type ConversationStatusFilter = (typeof conversationStatusFilters)[number];

/** The filters that get their own URL segment; All is the bare list route. */
export type ConversationFilterSegment = Exclude<ConversationStatusFilter, 'all'>;

export const statusFilterLabels: Record<ConversationStatusFilter, string> = {
	all: 'All',
	...conversationStatusLabels,
	shared: 'Shared'
};

export function isConversationFilterSegment(value: string): value is ConversationFilterSegment {
	return value !== 'all' && conversationStatusFilters.some((filter) => filter === value);
}

type ListedConversation = Pick<ConversationDto, 'id' | 'isLive' | 'isComplete'>;

export function filterConversations<T extends ListedConversation>(
	owned: T[],
	shared: T[],
	filter: ConversationStatusFilter
): T[] {
	if (filter === 'shared') return shared;
	const all = [...owned, ...shared];
	if (filter === 'all') return all;
	return all.filter((conversation) => conversationStatus(conversation) === filter);
}
