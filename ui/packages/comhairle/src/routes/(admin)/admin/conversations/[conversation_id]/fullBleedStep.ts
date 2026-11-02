/**
 * The HeyForm Setup tab embeds the whole HeyForm builder, so it fills the content region
 * instead of sitting in the centred reading column. The header and tabs above it follow suit.
 */
export function isFullBleedStepPage(
	pathname: string,
	toolType: string | undefined,
	canEdit: boolean
): boolean {
	return canEdit && toolType === 'heyform' && pathname.replace(/\/+$/, '').endsWith('/setup');
}

/** The step id in a `/design/step/<id>/...` path, or undefined off a step page. */
export function stepIdFromPath(pathname: string): string | undefined {
	return pathname.match(/\/design\/step\/([^/]+)/)?.[1];
}
