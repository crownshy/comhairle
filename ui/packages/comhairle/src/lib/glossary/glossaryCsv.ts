import { parseCsvRows } from '$lib/utils/csv';
import type { Glossary } from './types';
import { parseGlossary } from './parseGlossary';

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
export async function parseGlossaryCsv(text: string): Promise<Glossary> {
	const { rows } = await parseCsvRows(text);
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
