import * as m from '$lib/paraglide/messages';

export const PAGE_SIZE = 6;

export const getSort = (url: URL): SortBy => {
	const sortBy = url.searchParams.get('sort');
	if (sortBy === null) return 'title+asc';
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
	if (search) {
		url.searchParams.set('search', search);
	} else {
		url.searchParams.delete('search');
	}
	return url;
};
