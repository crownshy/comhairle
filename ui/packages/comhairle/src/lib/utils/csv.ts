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

/**
 * Save a CSV string as a file in the browser. The byte order mark makes Excel open it as
 * UTF-8; without it, accented text (Gaelic, Welsh, most translations) comes out garbled.
 */
export function downloadCsv(filename: string, csv: string): void {
	const blob = new Blob(['﻿', csv], { type: 'text/csv;charset=utf-8' });
	const url = URL.createObjectURL(blob);
	const a = document.createElement('a');
	a.href = url;
	a.download = filename;
	document.body.appendChild(a);
	a.click();
	a.remove();
	URL.revokeObjectURL(url);
}
