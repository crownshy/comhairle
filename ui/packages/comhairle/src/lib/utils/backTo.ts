const HOME = '/';

/**
 * Turns a `backTo` query value into a path on this site. Full URLs to our own origin become
 * their path; other hosts, `javascript:` links and anything unparseable send people home.
 */
export function safeBackTo(backTo: string | null | undefined, origin: string): string {
	if (!backTo) return HOME;

	const url = URL.parse(backTo, origin);
	if (!url || url.origin !== origin) return HOME;

	// A path starting with "//" would be read as another host when used as a link.
	if (url.pathname.startsWith('//')) return HOME;

	return url.pathname + url.search + url.hash;
}

export function backToFromUrl(url: URL): string {
	return safeBackTo(url.searchParams.get('backTo'), url.origin);
}
