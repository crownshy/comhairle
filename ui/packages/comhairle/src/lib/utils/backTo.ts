const HOME = '/';

/**
 * Turns a `backTo` query value into a path on this site. Full URLs to our own origin become
 * their path; other hosts, `javascript:` links and anything unparseable go to `fallback`.
 */
export function safeBackTo(
	backTo: string | null | undefined,
	origin: string,
	fallback: string = HOME
): string {
	if (!backTo) return fallback;

	const url = parseUrl(backTo, origin);
	if (!url || url.origin !== origin) return fallback;

	// A path starting with "//" would be read as another host when used as a link.
	if (url.pathname.startsWith('//')) return fallback;

	return pathWithQuery(url) + url.hash;
}

export function backToFromUrl(url: URL, fallback: string = HOME): string {
	return safeBackTo(url.searchParams.get('backTo'), url.origin, fallback);
}

/** The path and query of a URL, which is what a `backTo` link should carry. */
export function pathWithQuery(url: URL): string {
	return url.pathname + url.search;
}

// Not URL.parse: some pages run this in the browser, and URL.parse is missing before iOS 18.
function parseUrl(value: string, base: string): URL | null {
	try {
		return new URL(value, base);
	} catch {
		return null;
	}
}
