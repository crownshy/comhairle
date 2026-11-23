import { setPage } from '$lib/pagination/utils';

export const PAGE_SIZE = 6;

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
