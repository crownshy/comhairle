/**
 * Markdown-table detection. The parsing itself is done by `@tiptap/markdown` (via
 * `contentType: 'markdown'`); this module only decides *when* a paste is a genuine
 * GFM table, so ordinary pastes are left to the normal behaviour.
 */

/**
 * A GFM table separator row, e.g. `| --- | :--: |`. It contains only pipes,
 * dashes, colons and spaces, and has at least one dash and one pipe - which a
 * normal data row (with letters) never does, and a plain `---` horizontal rule
 * (no pipe) never does either.
 */
export function isMarkdownTableSeparator(line: string): boolean {
	const trimmed = line.trim();
	if (!trimmed.includes('-') || !trimmed.includes('|')) return false;
	return /^[|\-:\s]+$/.test(trimmed);
}

/**
 * True if the text contains a GFM table: a pipe row immediately followed by a
 * separator row. The separator requirement keeps false positives away from
 * ordinary text that happens to contain a stray `|`.
 */
export function containsMarkdownTable(text: string): boolean {
	const lines = text.split(/\r?\n/);
	for (let i = 0; i < lines.length - 1; i++) {
		const header = lines[i];
		if (!header.includes('|') || isMarkdownTableSeparator(header)) continue;
		if (isMarkdownTableSeparator(lines[i + 1])) return true;
	}
	return false;
}
