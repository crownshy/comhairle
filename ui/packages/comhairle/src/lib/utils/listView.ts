import type { ConversationCardLayout } from '$lib/components/ConversationCard.svelte'; /** The list shows cards unless `?view=list` asks for one row per conversation. */
export function listViewFromUrl(url: URL): ConversationCardLayout {
	return url.searchParams.get('view') === 'list' ? 'list' : 'card';
}

/** Keeps the filter, search and page, since switching view doesn't change the results. */
export function listViewHref(url: URL, view: ConversationCardLayout): string {
	const target = new URL(url);
	if (view === 'list') target.searchParams.set('view', view);
	else target.searchParams.delete('view');
	return `${target.pathname}${target.search}`;
}
