import { goto } from '$app/navigation';
import { page } from '$app/state';

function refresh(href: string = new URL(page.url).href) {
	// eslint-disable-next-line svelte/no-navigation-without-resolve
	goto(href, { replaceState: true, keepFocus: true, noScroll: true });
}

function get<T extends string>(key: T, url?: URL): string | null {
	const Url = url ?? new URL(page.url);
	return Url.searchParams.get(key);
}

function set<T extends string>(key: T, value: string, url?: URL) {
	const Url = url ?? new URL(page.url);
	if (!value) {
		Url.searchParams.delete(key);
	} else {
		Url.searchParams.set(key, value);
	}

	return {
		set: <T extends string>(key: T, value: string) => set(key, value, Url),
		update: () => refresh(Url.href)
	};
}

function remove<T extends string>(key: T, url?: URL): void {
	const Url = url ?? new URL(page.url);
	Url.searchParams.delete(key);
	refresh(Url.href);
}

const QueryParams = {
	get,
	set,
	remove
};

export default QueryParams;
