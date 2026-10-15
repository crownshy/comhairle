import { CONVERSATION_STATUS_LABELS } from '$lib/utils/conversationStatus';

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

/**
 * Keeps the search, drops the page number (the new chip may have fewer pages),
 * and drops `status` for All so it has one URL.
 */
export function statusFilterHref(url: URL, filter: ConversationStatusFilter): string {
	const target = new URL(url);
	target.searchParams.delete('page');
	if (filter === 'all') target.searchParams.delete('status');
	else target.searchParams.set('status', filter);
	return `${target.pathname}${target.search}`;
}

export function searchFromUrl(url: URL): string {
	return url.searchParams.get('q') ?? '';
}

/** Keeps the chip and drops the page number, since the results change. */
export function searchHref(url: URL, search: string): string {
	const target = new URL(url);
	target.searchParams.delete('page');
	if (search.trim()) target.searchParams.set('q', search);
	else target.searchParams.delete('q');
	return `${target.pathname}${target.search}`;
}

type ConversationListQueries = {
	is_live?: boolean;
	is_complete?: boolean;
	exclude_owner_id?: string;
};

/**
 * Query params for `/user/permitted_conversations`, which returns the conversations you own
 * as well as the ones shared with you. Completed wins over live, matching `conversationStatus`.
 */
export function conversationListQueries(
	filter: ConversationStatusFilter,
	userId: string
): ConversationListQueries {
	switch (filter) {
		case 'all':
			return {};
		case 'live':
			return { is_live: true, is_complete: false };
		case 'draft':
			return { is_live: false, is_complete: false };
		case 'completed':
			return { is_complete: true };
		case 'shared':
			return { exclude_owner_id: userId };
	}
}
