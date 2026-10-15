import { getPage } from '$lib/pagination';
import {
	CONVERSATION_STATUS_LABELS,
	CONVERSATION_STATUS_QUERIES
} from '$lib/utils/conversationStatus';

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

/** Changing a filter changes the results, so it always goes back to the first page. */
function listHref(url: URL, name: 'status' | 'q', value: string | undefined): string {
	const target = new URL(url);
	target.searchParams.delete('page');
	if (value) target.searchParams.set(name, value);
	else target.searchParams.delete(name);
	return `${target.pathname}${target.search}`;
}

/** Keeps the search, and drops `status` for All so it has one URL. */
export function statusFilterHref(url: URL, filter: ConversationStatusFilter): string {
	return listHref(url, 'status', filter === 'all' ? undefined : filter);
}

export function searchFromUrl(url: URL): string {
	return url.searchParams.get('q') ?? '';
}

/** Keeps the chip. */
export function searchHref(url: URL, search: string): string {
	return listHref(url, 'q', search.trim() ? search : undefined);
}

/** True when only the page number moved, so the list should start again from the top. */
export function isPageTurn(from: URL, to: URL): boolean {
	return (
		getPage(from) !== getPage(to) &&
		statusFilterFromUrl(from) === statusFilterFromUrl(to) &&
		searchFromUrl(from) === searchFromUrl(to)
	);
}

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
		case 'all':
			return {};
		case 'shared':
			return { exclude_owner_id: userId };
		default:
			return CONVERSATION_STATUS_QUERIES[filter];
	}
}
