import type { Glossary } from './types';
import { parseGlossary } from './parseGlossary';

/**
 * Parses CSV text into rows of string cells (RFC 4180: quoted fields may contain commas,
 * newlines, and "" escaped quotes). Hand-rolled because there's no CSV dep in the repo and
 * the glossary's two-column shape doesn't warrant adding one.
 */
export function parseCsvRows(text: string): string[][] {
	const rows: string[][] = [];
	let row: string[] = [];
	let field = '';
	let inQuotes = false;

	// Skip a UTF-8 BOM if a spreadsheet exported one.
	for (let i = text.charCodeAt(0) === 0xfeff ? 1 : 0; i < text.length; i++) {
		const char = text[i];

		if (inQuotes) {
			if (char === '"') {
				if (text[i + 1] === '"') {
					field += '"';
					i++;
				} else {
					inQuotes = false;
				}
			} else {
				field += char;
			}
			continue;
		}

		if (char === '"') {
			inQuotes = true;
		} else if (char === ',') {
			row.push(field);
			field = '';
		} else if (char === '\n') {
			row.push(field);
			rows.push(row);
			row = [];
			field = '';
		} else if (char !== '\r') {
			// A lone or CRLF '\r' is dropped; '\n' handles the row break.
			field += char;
		}
	}

	row.push(field);
	rows.push(row);

	// Drop blank lines (a trailing newline leaves an empty final row).
	return rows.filter((cells) => !(cells.length === 1 && cells[0].trim() === ''));
}

const HEADER_TERM_NAMES = ['term', 'terms', 'word', 'words', 'phrase'];
const HEADER_DEFINITION_NAMES = ['tooltip', 'definition', 'explanation', 'meaning', 'description'];

function looksLikeHeader(cells: string[]): boolean {
	const first = cells[0]?.trim().toLowerCase() ?? '';
	const second = cells[1]?.trim().toLowerCase() ?? '';
	return HEADER_TERM_NAMES.includes(first) || HEADER_DEFINITION_NAMES.includes(second);
}

/**
 * Parses a CSV glossary. Two columns: the first holds the term plus any synonyms separated
 * by `;` or `|` (comma is the CSV field separator, so it can't double as the synonym one);
 * the second holds the explanation. A header row is skipped when the first row's cells look
 * like column names. Malformed rows are dropped via parseGlossary.
 */
export function parseGlossaryCsv(text: string): Glossary {
	const rows = parseCsvRows(text);
	if (rows.length === 0) return [];

	const body = looksLikeHeader(rows[0]) ? rows.slice(1) : rows;

	const raw = body.map((cells) => ({
		text: (cells[0] ?? '')
			.split(/[;|]/)
			.map((term) => term.trim())
			.filter(Boolean),
		tooltip: (cells[1] ?? '').trim()
	}));

	return parseGlossary(raw);
}
