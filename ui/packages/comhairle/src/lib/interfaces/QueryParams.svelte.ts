import { goto } from '$app/navigation';
import { page } from '$app/state';

function refresh(href: string = new URL(page.url).href) {
	// eslint-disable-next-line svelte/no-navigation-without-resolve
	goto(href, { replaceState: true, keepFocus: true, noScroll: true });
}

function get<T extends string>(key: T): string | null {
	const url = new URL(page.url);
	return url.searchParams.get(key);
}

function set<T extends string>(key: T, value: string): void {
	const url = new URL(page.url);
	if (!value) {
		url.searchParams.delete(key);
	} else {
		url.searchParams.set(key, value);
	}
	refresh(url.href);
}

function remove<T extends string>(key: T): void {
	const url = new URL(page.url);
	url.searchParams.delete(key);
	refresh(url.href);
}

const QueryParams = {
	get,
	set,
	remove
};

export default QueryParams;
