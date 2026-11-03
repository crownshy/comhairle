const SNIPPET_LENGTH = 110;

const ENTITIES: Record<string, string> = {
	'&nbsp;': ' ',
	'&amp;': '&',
	'&lt;': '<',
	'&gt;': '>',
	'&quot;': '"',
	'&#39;': "'",
	'&rsquo;': '’',
	'&lsquo;': '‘'
};

/**
 * The line of text an inbox shows under the subject: the email's visible text, with
 * markup, styles and image alt text left out, cut to a sensible length.
 * @param html The rendered email.
 */
export function inboxSnippet(html: string): string {
	const text = html
		.replace(/<(style|script|head|title)[\s\S]*?<\/\1>/gi, ' ')
		.replace(/<[^>]+>/g, ' ')
		.replace(/&[a-z#0-9]+;/gi, (entity) => ENTITIES[entity.toLowerCase()] ?? ' ')
		.replace(/\s+/g, ' ')
		.trim();

	return text.length > SNIPPET_LENGTH ? `${text.slice(0, SNIPPET_LENGTH).trimEnd()}…` : text;
}
