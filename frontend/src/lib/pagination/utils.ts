export const getPage = (url: URL): number => {
	const page = Number(url.searchParams.get('page'));
	if (!page || Number.isNaN(page)) return 1;
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
