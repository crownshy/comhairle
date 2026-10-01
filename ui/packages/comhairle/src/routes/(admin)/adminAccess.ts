export function adminReturnPath(referer: string | null, currentUrl: URL): string {
	if (!referer || !URL.canParse(referer)) return '/';
	const previousUrl = new URL(referer);
	if (
		previousUrl.origin !== currentUrl.origin ||
		previousUrl.pathname.startsWith('//') ||
		previousUrl.pathname.includes('\\') ||
		/^\/admin(?:\/|$)/.test(previousUrl.pathname)
	) {
		return '/';
	}
	return previousUrl.pathname + previousUrl.search + previousUrl.hash;
}
