import { parseCsvRows } from '$lib/utils/csv';

/**
 * Headings that name the statement column, lowercased. Used to spot a header row in a
 * one-column file and to pick the default column in a wider one. `statement_text` is what our
 * own statements export writes, so a re-imported export needs no choosing.
 */
const STATEMENT_HEADINGS = [
	'statement',
	'statements',
	'statement_text',
	'text',
	'comment',
	'comment_body',
	'comment-body'
];

/** A column the admin can take the statements from, when the file has more than one. */
export type SeedColumn = {
	/** Position of the column in each row. */
	index: number;
	/** The column's heading from the first row, or "Column N" when that cell is blank. */
	heading: string;
};

export type ParsedSeedCsv = {
	/** One statement per surviving row, trimmed, in file order. */
	statements: string[];
	/** The heading that was skipped, or null when the file had no header row. */
	header: string | null;
	/** Parser complaints worth showing above the preview. Empty when the file is clean. */
	problems: string[];
	/** Every column with content in it, when there is more than one. Empty otherwise. */
	columns: SeedColumn[];
	/** Index of the column the statements came from, or null for a one-column file. */
	column: number | null;
};

function isStatementHeading(value: string): boolean {
	return STATEMENT_HEADINGS.includes(value.trim().toLowerCase());
}

/**
 * Reads the statements out of a seed statement CSV.
 *
 * A one-column file gives one statement per row, taken from the first cell that has anything
 * in it. A file exported from a spreadsheet arrives padded: every row carries the column count
 * of the widest row, so a one-column file reads `text,,,,,,` and a blank line reads `,,,,,,`.
 * Excel hides that, which is why the padding reaches us at all. parseCsvRows drops the rows
 * that are all padding; taking the first cell with content in it drops the rest.
 *
 * A file with content in more than one column is read as having a header row, and the
 * statements come from one column: `column` when given, otherwise the first whose heading
 * names statements, otherwise the first with content. A multi-column file with no header row
 * loses its first row to the headings, which the preview shows.
 *
 * @param text the raw file contents, byte order mark and all
 * @param column index of the column to read, from a previous parse's `columns`
 */
export async function parseSeedCsv(text: string, column?: number): Promise<ParsedSeedCsv> {
	const parsed = await parseCsvRows(text);
	const rows = parsed.rows.map((cells) => cells.map((cell) => cell.trim()));
	const { problems } = parsed;

	const width = Math.max(0, ...rows.map((cells) => cells.length));
	const populated = Array.from({ length: width }, (_, index) => index).filter((index) =>
		rows.some((cells) => Boolean(cells[index]))
	);

	if (populated.length > 1) {
		const headings = rows[0];
		const columns = populated.map((index) => ({
			index,
			heading: headings[index] || `Column ${index + 1}`
		}));
		const chosen =
			column !== undefined && populated.includes(column)
				? column
				: (columns.find((candidate) => isStatementHeading(candidate.heading)) ?? columns[0])
						.index;

		return {
			statements: rows
				.slice(1)
				.map((cells) => cells[chosen] ?? '')
				.filter(Boolean),
			header: headings[chosen] || null,
			problems,
			columns,
			column: chosen
		};
	}

	const values = rows.map((cells) => cells.find(Boolean) ?? '').filter(Boolean);
	const first = values[0] ?? '';
	const header = isStatementHeading(first) ? first : null;

	return {
		statements: header === null ? values : values.slice(1),
		header,
		problems,
		columns: [],
		column: null
	};
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
