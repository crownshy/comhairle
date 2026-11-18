import { setPage } from '$lib/pagination/utils';
import { m } from '$lib/paraglide/messages';

export const PAGE_SIZE = 6;

export const getSort = (url: URL): SortBy => {
	const sortBy = url.searchParams.get('sort');
	if (sortBy === null) return 'created_at+desc';
	return sortBy as SortBy;
};
export const setSort = (_url: URL, v: string) => {
	const url = new URL(_url);
	url.searchParams.set('sort', v);
	return url;
};

export type SortBy = 'title+asc' | 'title+desc' | 'created_at+asc' | 'created_at+desc';

export const parseSort = (o: SortBy) => {
	switch (o) {
		case 'created_at+asc':
			return m.oldest();
		case 'created_at+desc':
			return m.newest();
		case 'title+asc':
			return m.title();
		case 'title+desc':
			return m.title_descending();
		default:
			return m.title();
	}
};

export const getSearch = (url: URL): string => {
	const search = url.searchParams.get('search');
	if (search === null) return '';
	return search;
};

export const setSearch = (_url: URL, search: string): URL => {
	const url = new URL(_url);
	url.searchParams.delete('page');
	if (search) {
		url.searchParams.set('search', search);
		setPage(url, 0);
	} else {
		url.searchParams.delete('search');
		setPage(url, 0);
	}
	return url;
};

export const STATUS_FILTERS = ['open', 'ended', 'all'] as const;
export type StatusFilter = (typeof STATUS_FILTERS)[number];

/** Open is the default, so a plain "/" shows what it always has. */
export const getStatusFilter = (url: URL): StatusFilter => {
	const value = url.searchParams.get('status');
	return value === 'ended' || value === 'all' ? value : 'open';
};

/** A new filter changes the results, so it goes back to the first page. */
export const statusFilterHref = (url: URL, filter: StatusFilter): string => {
	const target = new URL(url);
	target.searchParams.delete('page');
	if (filter === 'open') target.searchParams.delete('status');
	else target.searchParams.set('status', filter);
	return `${target.pathname}${target.search}`;
};

/** The API query for each filter. The API itself only ever returns live conversations. */
export const STATUS_FILTER_QUERY: Record<StatusFilter, string> = {
	open: '&is_complete=false',
	ended: '&is_complete=true',
	all: ''
};
