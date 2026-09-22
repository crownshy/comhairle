import { parseCsvRows } from '$lib/utils/csv';

/** First-cell values that mark a first row as column headings rather than a statement. */
const HEADER_NAMES = ['statement', 'statements', 'text'];

export type ParsedSeedCsv = {
	/** One statement per surviving row, trimmed, in file order. */
	statements: string[];
	/** The heading that was skipped, or null when the file had no header row. */
	header: string | null;
	/** Parser complaints worth showing above the preview. Empty when the file is clean. */
	problems: string[];
};

/**
 * Reads the statements out of a seed statement CSV. One statement per row, taken from the
 * first cell that has anything in it.
 *
 * A file exported from a spreadsheet arrives padded: every row carries the column count of
 * the widest row, so a one-column file reads `text,,,,,,` and a blank line reads `,,,,,,`.
 * Excel hides that, which is why the padding reaches us at all. parseCsvRows drops the rows
 * that are all padding; taking the first cell with content in it drops the rest.
 *
 * Only the first populated column is read. A file with a second meaningful column (a theme, a
 * note) needs the admin to say which one holds the statement, which is its own piece of work.
 *
 * @param text the raw file contents, byte order mark and all
 */
export async function parseSeedCsv(text: string): Promise<ParsedSeedCsv> {
	const { rows, problems } = await parseCsvRows(text);

	const values = rows
		.map((cells) => cells.map((cell) => cell.trim()).find(Boolean) ?? '')
		.filter(Boolean);

	const first = values[0] ?? '';
	const header = HEADER_NAMES.includes(first.toLowerCase()) ? first : null;

	return { statements: header === null ? values : values.slice(1), header, problems };
}

/** Why a parsed statement is worth a second look before it is posted. */
export type SeedStatementIssue = 'empty' | 'duplicate-in-file' | 'duplicate-in-step';

function normalise(statement: string): string {
	return statement.trim().toLowerCase().replace(/\s+/g, ' ');
}

/**
 * Flags each statement that is worth a second look, in the same order. Advisory only: the
 * admin decides whether to edit, remove or post anyway.
 *
 * Comparison ignores case and collapses whitespace, so near-misses surface alongside exact
 * repeats. The first copy of a repeated statement is left unflagged and the ones after it
 * are flagged, so removing every flagged row still leaves one of each.
 *
 * @param statements the parsed statements, as currently edited
 * @param existingStatements every statement already in the step, rejected ones included
 */
export function findSeedIssues(
	statements: string[],
	existingStatements: string[]
): (SeedStatementIssue | null)[] {
	const existing = new Set(existingStatements.map(normalise));
	const seen = new Set<string>();

	return statements.map((statement) => {
		const key = normalise(statement);
		if (!key) return 'empty';
		if (seen.has(key)) return 'duplicate-in-file';
		seen.add(key);
		if (existing.has(key)) return 'duplicate-in-step';
		return null;
	});
}
