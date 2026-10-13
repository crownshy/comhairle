/** A cleared rich editor still saves markup like `<p></p>` or `<p>&nbsp;</p>`. */
export function isBlankRichText(html: string | null | undefined): boolean {
	const text = (html ?? '')
		.replace(/<[^>]*>/g, '')
		.replace(/&nbsp;/g, '')
		.trim();
	return text.length === 0;
}
