import type { ConversationDto } from '@crownshy/api-client/api';
import { conversationStatus, conversationStatusLabels } from './conversationStatus';

export const conversationStatusFilters = ['all', 'live', 'draft', 'completed', 'shared'] as const;
export type ConversationStatusFilter = (typeof conversationStatusFilters)[number];

/** The filters that get their own URL segment; All is the bare list route. */
export type ConversationFilterSegment = Exclude<ConversationStatusFilter, 'all'>;

export const conversationStatusFilterLabels: Record<ConversationStatusFilter, string> = {
	all: 'All',
	...conversationStatusLabels,
	shared: 'Shared'
};

export function isConversationStatusFilter(value: string): value is ConversationStatusFilter {
	return conversationStatusFilters.some((filter) => filter === value);
}

export function isConversationFilterSegment(value: string): value is ConversationFilterSegment {
	return value !== 'all' && isConversationStatusFilter(value);
}

export function filterToSegment(
	filter: ConversationStatusFilter
): ConversationFilterSegment | undefined {
	return filter === 'all' ? undefined : filter;
}

export function filterFromSegment(segment: string | undefined): ConversationStatusFilter {
	return segment && isConversationFilterSegment(segment) ? segment : 'all';
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
