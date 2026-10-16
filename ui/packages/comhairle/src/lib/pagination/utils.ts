export const getPage = (url: URL): number => {
	const page = Math.floor(Number(url.searchParams.get('page')));
	if (!Number.isFinite(page) || page < 1) return 1;
	return page;
};
export const setPage = (_url: URL, v: number): URL => {
	const url = new URL(_url);
	if (v === 1) {
		url.searchParams.delete('page');
	} else {
		url.searchParams.set('page', String(v));
	}
	return url;
};

export const calcOffset = (opts: { page: number; pageSize: number }): number => {
	if (opts.page <= 1) return 0;
	return (opts.page - 1) * opts.pageSize;
};

export const calcPage = (opts: { offset: number; pageSize: number }): number => {
	if (opts.offset < opts.pageSize) return 1;
	return Math.floor(opts.offset / opts.pageSize);
};

export const calcPageCount = (opts: { pageSize: number; total: number }) =>
	opts.total < 1 ? 1 : Math.ceil(opts.total / opts.pageSize);

export type PageItem = { type: 'page'; page: number } | { type: 'ellipsis'; key: string };

/**
 * The page numbers to show around the current page, always including the first and last.
 * A gap of exactly one page shows that page instead of an ellipsis, so the row keeps a steady width.
 */
export const pageItems = (opts: {
	current: number;
	pageCount: number;
	siblings?: number;
}): PageItem[] => {
	const { current, pageCount, siblings = 1 } = opts;
	const start = Math.max(1, current - siblings);
	const end = Math.min(pageCount, current + siblings);
	const pages = new Set([1, pageCount]);
	for (let page = start; page <= end; page++) pages.add(page);
	if (start === 3) pages.add(2);
	if (end === pageCount - 2) pages.add(pageCount - 1);

	const items: PageItem[] = [];
	let previous = 0;
	for (const page of [...pages].sort((a, b) => a - b)) {
		if (page - previous > 1) items.push({ type: 'ellipsis', key: `after-${previous}` });
		items.push({ type: 'page', page });
		previous = page;
	}
	return items;
};
