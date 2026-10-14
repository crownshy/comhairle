import type { ConversationDto } from '@crownshy/api-client/api';
import { conversationStatus, conversationStatusLabels } from '$lib/utils/conversationStatus';

export const conversationStatusFilters = ['all', 'live', 'draft', 'completed', 'shared'] as const;
export type ConversationStatusFilter = (typeof conversationStatusFilters)[number];

export const statusFilterLabels: Record<ConversationStatusFilter, string> = {
	all: 'All',
	...conversationStatusLabels,
	shared: 'Shared'
};

export function parseStatusFilter(value: string | null): ConversationStatusFilter {
	return conversationStatusFilters.find((filter) => filter === value) ?? 'all';
}

export function statusFilterHref(url: URL, filter: ConversationStatusFilter): string {
	const params = new URLSearchParams(url.searchParams);
	if (filter === 'all') params.delete('status');
	else params.set('status', filter);
	const query = params.toString();
	return query ? `${url.pathname}?${query}` : url.pathname;
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
