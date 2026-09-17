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
