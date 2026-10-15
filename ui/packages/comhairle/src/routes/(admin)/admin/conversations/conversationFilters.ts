import type { ConversationDto } from '@crownshy/api-client/api';
import { conversationStatus, CONVERSATION_STATUS_LABELS } from '$lib/utils/conversationStatus';

export const CONVERSATION_STATUS_FILTERS = ['all', 'live', 'draft', 'completed', 'shared'] as const;
export type ConversationStatusFilter = (typeof CONVERSATION_STATUS_FILTERS)[number];

export const CONVERSATION_STATUS_FILTER_LABELS: Record<ConversationStatusFilter, string> = {
	all: 'All',
	...CONVERSATION_STATUS_LABELS,
	shared: 'Shared'
};

export function isConversationStatusFilter(value: string): value is ConversationStatusFilter {
	return CONVERSATION_STATUS_FILTERS.some((filter) => filter === value);
}

export function statusFilterFromUrl(url: URL): ConversationStatusFilter {
	const value = url.searchParams.get('status');
	return value && isConversationStatusFilter(value) ? value : 'all';
}

/** Keeps the rest of the query string; All drops `status` so it has one URL. */
export function statusFilterHref(url: URL, filter: ConversationStatusFilter): string {
	const target = new URL(url);
	if (filter === 'all') target.searchParams.delete('status');
	else target.searchParams.set('status', filter);
	return `${target.pathname}${target.search}`;
}

type ListedConversation = Pick<ConversationDto, 'id' | 'isLive' | 'isComplete'>;

export function filterConversations<T extends ListedConversation>(
	owned: T[],
	shared: T[],
	filter: ConversationStatusFilter
): T[] {
	if (filter === 'shared') return shared;
	const all = owned.concat(shared);
	if (filter === 'all') return all;
	return all.filter((conversation) => conversationStatus(conversation) === filter);
}
