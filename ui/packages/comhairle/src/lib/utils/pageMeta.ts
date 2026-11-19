/** What a load function can set to describe its page in link previews. */
export type PageMeta = {
	title?: string;
	description?: string;
	image?: string;
};

export type ResolvedPageMeta = {
	title: string;
	description: string;
	url: string;
	image: string;
};

type Defaults = {
	title: string;
	description: string;
	image: string;
};

/** Social sites cut descriptions off at around this length, so longer text is trimmed. */
export const DESCRIPTION_MAX_LENGTH = 200;

/**
 * Fills the gaps in a page's meta from the site defaults. Crawlers don't resolve relative
 * URLs, so the image and page URL are made absolute against the site origin.
 */
export function resolvePageMeta(
	meta: PageMeta | undefined,
	defaults: Defaults,
	pageUrl: URL
): ResolvedPageMeta {
	const description = meta?.description?.trim() || defaults.description;
	const image = meta?.image?.trim() || defaults.image;

	return {
		title: meta?.title?.trim() || defaults.title,
		description: truncate(description, DESCRIPTION_MAX_LENGTH),
		url: new URL(pageUrl.pathname, pageUrl.origin).href,
		image: new URL(image, pageUrl.origin).href
	};
}

function truncate(text: string, maxLength: number): string {
	const singleLine = text.replace(/\s+/g, ' ');
	if (singleLine.length <= maxLength) return singleLine;
	return singleLine.slice(0, maxLength - 1).trimEnd() + '…';
}
