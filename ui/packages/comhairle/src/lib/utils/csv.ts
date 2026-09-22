export type CsvParseResult = {
	/** One array of cell strings per row, blank rows already dropped. */
	rows: string[][];
	/**
	 * Anything the parser objected to, phrased for a person. Non-fatal: `rows` still holds the
	 * parser's best reading. An unterminated quote is the one that matters, because it swallows
	 * the rest of the file into a single cell without otherwise looking wrong.
	 */
	problems: string[];
};

/**
 * Parses CSV text into rows of string cells (RFC 4180: quoted fields may contain the delimiter,
 * newlines, and "" escaped quotes).
 *
 * The delimiter is sniffed, not assumed, so a file exported by a spreadsheet set to a non-UK
 * locale (semicolon separated) reads correctly. Sniffing scores field-count consistency across
 * rows, so a stray semicolon inside one sentence does not beat the real separator. A byte order
 * mark is stripped, and rows holding nothing but separators and whitespace are dropped, which is
 * what a spreadsheet's trailing empty columns look like.
 *
 * papaparse is loaded on demand: the writing helpers below are imported by report and export
 * views that never parse, and those chunks should not carry the parser.
 *
 * @param text the raw file contents, byte order mark and all
 */
export async function parseCsvRows(text: string): Promise<CsvParseResult> {
	const { default: Papa } = await import('papaparse');
	const parsed = Papa.parse<string[]>(text, { skipEmptyLines: 'greedy' });

	return {
		rows: parsed.data,
		problems: parsed.errors
			// A file with one column per row has no delimiter to find, which is normal here and
			// not worth reporting. Every other complaint is.
			.filter((error) => error.code !== 'UndetectableDelimiter')
			.map((error) =>
				error.row === undefined ? error.message : `${error.message} (row ${error.row + 1})`
			)
	};
}

/**
 * Quote a value for a CSV cell: wrap in quotes, double internal quotes, normalise newlines.
 * Text starting with a character a spreadsheet reads as a formula (= + - @, tab, carriage
 * return) gets a leading apostrophe, because exported cells hold participant-written text.
 */
export function csvField(value: unknown): string {
	if (value === null || value === undefined) return '';
	let text = String(value);
	if (typeof value === 'string' && /^[=+\-@\t\r]/.test(text)) text = `'${text}`;
	text = text.replace(/\r\n|\r/g, '\n');
	return `"${text.replace(/"/g, '""')}"`;
}

/** Rows of cells to a CSV string, one line per row. */
export function toCsv(rows: unknown[][]): string {
	return rows.map((row) => row.map(csvField).join(',')).join('\n');
}

/** Waits this long before freeing the file, the same delay FileSaver.js uses. */
const REVOKE_DELAY_MS = 40_000;

/**
 * Save a CSV string as a file in the browser. The byte order mark makes Excel open it as
 * UTF-8; without it, accented text (Gaelic, Welsh, most translations) comes out garbled.
 */
export function downloadCsv(filename: string, csv: string): void {
	const blob = new Blob(['﻿', csv], { type: 'text/csv;charset=utf-8' });
	const url = URL.createObjectURL(blob);
	const link = document.createElement('a');
	link.href = url;
	link.download = filename;
	document.body.appendChild(link);
	link.click();
	link.remove();
	// Revoking straight after click() can cancel the download before the browser reads the file.
	setTimeout(() => URL.revokeObjectURL(url), REVOKE_DELAY_MS);
}
