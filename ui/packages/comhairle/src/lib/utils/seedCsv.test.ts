import { describe, expect, it } from 'vitest';
import { findSeedIssues, parseSeedCsv } from './seedCsv';

describe('parseSeedCsv', () => {
	it('reads one statement per line', async () => {
		expect(await parseSeedCsv('First statement\nSecond statement')).toEqual({
			statements: ['First statement', 'Second statement'],
			header: null,
			problems: [],
			columns: [],
			column: null
		});
	});

	// The file that caused #1218: exported from a spreadsheet with six trailing empty columns.
	it('drops the trailing empty columns a spreadsheet exports', async () => {
		const file = [
			',,,,,,',
			'I trust the people who run this service to do the right thing,,,,,,',
			'I would feel more confident if I knew how often checks happened,,,,,,'
		].join('\n');

		expect(await parseSeedCsv(file)).toEqual({
			statements: [
				'I trust the people who run this service to do the right thing',
				'I would feel more confident if I knew how often checks happened'
			],
			header: null,
			problems: [],
			columns: [],
			column: null
		});
	});

	it('drops a row that is nothing but separators', async () => {
		const parsed = await parseSeedCsv('Keep me\n,,,,,,\n,,\nKeep me too');
		expect(parsed.statements).toEqual(['Keep me', 'Keep me too']);
	});

	it('recognises a header row that carries trailing columns', async () => {
		const parsed = await parseSeedCsv('statement,,,,,,\nA statement,,,,,,');
		expect(parsed.statements).toEqual(['A statement']);
		expect(parsed.header).toBe('statement');
	});

	it('recognises a header row behind a byte order mark', async () => {
		const parsed = await parseSeedCsv('﻿Statements\nA statement');
		expect(parsed.statements).toEqual(['A statement']);
		expect(parsed.header).toBe('Statements');
	});

	it('strips a byte order mark from the first statement when there is no header', async () => {
		const parsed = await parseSeedCsv('﻿A statement');
		expect(parsed.statements).toEqual(['A statement']);
	});

	it('keeps commas inside a quoted field', async () => {
		const file = '"Buses, trains and ferries should be one ticket",,,\nCycling is safe enough';
		const parsed = await parseSeedCsv(file);

		expect(parsed.statements).toEqual([
			'Buses, trains and ferries should be one ticket',
			'Cycling is safe enough'
		]);
	});

	it('keeps a newline inside a quoted field as one statement', async () => {
		const parsed = await parseSeedCsv('"line one\nline two"');
		expect(parsed.statements).toEqual(['line one\nline two']);
	});

	it('takes the first cell with anything in it when the row is padded in front', async () => {
		const parsed = await parseSeedCsv(',,A statement,,');
		expect(parsed.statements).toEqual(['A statement']);
	});

	// A hand-written file: one statement per line, commas unquoted. The comma lines split
	// into more cells than their neighbours, which must not read as a multi-column file.
	it('keeps an unquoted comma inside a hand-written statement', async () => {
		const file = [
			'We should act now, not later',
			'Cycling is safe enough',
			'Buses, trains and trams need money'
		].join('\n');
		const parsed = await parseSeedCsv(file);

		expect(parsed.statements).toEqual([
			'We should act now, not later',
			'Cycling is safe enough',
			'Buses, trains and trams need money'
		]);
		expect(parsed.columns).toEqual([]);
		expect(parsed.header).toBeNull();
	});

	it('keeps an unquoted comma in a statement padded with trailing columns', async () => {
		const parsed = await parseSeedCsv(
			'Buses, trains and trams need money,,,,\nCycling is fine'
		);
		expect(parsed.statements).toEqual([
			'Buses, trains and trams need money',
			'Cycling is fine'
		]);
	});

	it('reads a one-line file with a comma as one statement', async () => {
		const parsed = await parseSeedCsv('We should act now, not later');
		expect(parsed.statements).toEqual(['We should act now, not later']);
		expect(parsed.columns).toEqual([]);
	});

	it('reads a file with no statements as empty', async () => {
		expect((await parseSeedCsv(',,,,,,\n\n,,')).statements).toEqual([]);

		const headerOnly = await parseSeedCsv('statement,,,,,,');
		expect(headerOnly.statements).toEqual([]);
		expect(headerOnly.header).toBe('statement');
	});

	it('does not mistake a statement for a heading', async () => {
		const parsed = await parseSeedCsv('Statements should be short,,\nSo should this');
		expect(parsed.header).toBeNull();
	});

	// A spreadsheet set to a non-UK locale exports semicolon separated. The delimiter is
	// sniffed, so the statement column is read rather than the whole line.
	it('reads a semicolon separated export', async () => {
		const file = 'statement;theme\nBuses are late;transport\nTrains are worse;transport';
		const parsed = await parseSeedCsv(file);

		expect(parsed.statements).toEqual(['Buses are late', 'Trains are worse']);
		expect(parsed.header).toBe('statement');
	});

	// Sniffing scores field-count consistency across rows, so one stray semicolon in a
	// sentence must not beat the comma that actually separates the columns.
	it('does not split a statement on a semicolon inside it', async () => {
		const parsed = await parseSeedCsv(
			'Buses are late; trains are worse\nCycling is fine\nWe need more routes'
		);

		expect(parsed.statements).toEqual([
			'Buses are late; trains are worse',
			'Cycling is fine',
			'We need more routes'
		]);
	});

	// Our own statements export: the statement sits in the sixth column, not the first.
	it('reads the statement column of a multi-column export by its heading', async () => {
		const file = [
			'user_id,agrees,statement_text,moderation_status',
			'f64cd7f3,2,seed 1,accepted',
			'f64cd7f3,1,seed 2,accepted'
		].join('\n');
		const parsed = await parseSeedCsv(file);

		expect(parsed.statements).toEqual(['seed 1', 'seed 2']);
		expect(parsed.header).toBe('statement_text');
		expect(parsed.column).toBe(2);
		expect(parsed.columns.map((column) => column.heading)).toEqual([
			'user_id',
			'agrees',
			'statement_text',
			'moderation_status'
		]);
	});

	it('reads the column it is asked for', async () => {
		const file = 'user_id,statement_text,note\na,seed 1,first\nb,seed 2,second';
		const parsed = await parseSeedCsv(file, 2);

		expect(parsed.statements).toEqual(['first', 'second']);
		expect(parsed.header).toBe('note');
	});

	it('falls back to the first column when no heading names statements', async () => {
		const parsed = await parseSeedCsv('idea,theme\nBuses are late,transport');

		expect(parsed.column).toBe(0);
		expect(parsed.statements).toEqual(['Buses are late']);
	});

	it('ignores a column asked for that has nothing in it', async () => {
		const parsed = await parseSeedCsv('statement,,theme,,\nBuses are late,,transport,,', 1);

		expect(parsed.column).toBe(0);
		expect(parsed.columns.map((column) => column.index)).toEqual([0, 2]);
	});

	it('names a column with a blank heading by its position', async () => {
		const parsed = await parseSeedCsv('statement,\nBuses are late,transport');
		expect(parsed.columns[1].heading).toBe('Column 2');
	});

	it('skips a row with nothing in the chosen column', async () => {
		const parsed = await parseSeedCsv('id,statement\n1,Buses are late\n2,\n3,Trains too');
		expect(parsed.statements).toEqual(['Buses are late', 'Trains too']);
	});

	it('offers no columns for a one-column file', async () => {
		const parsed = await parseSeedCsv('statement,,,\nA statement,,,');
		expect(parsed.columns).toEqual([]);
		expect(parsed.column).toBeNull();
	});

	it('reports a file the parser could not read cleanly', async () => {
		// An unterminated quote swallows the rest of the file into one statement, which looks
		// plausible in the preview unless the problem is named.
		const parsed = await parseSeedCsv('"Buses are late\nCycling is fine\nMore routes please');

		expect(parsed.problems.length).toBeGreaterThan(0);
		expect(parsed.problems[0]).toMatch(/quote/i);
	});

	it('reports no problems for a clean file', async () => {
		expect((await parseSeedCsv('One\nTwo')).problems).toEqual([]);
	});
});

describe('findSeedIssues', () => {
	it('leaves clean statements unflagged', () => {
		expect(findSeedIssues(['One', 'Two'], ['Something else'])).toEqual([null, null]);
	});

	it('flags a statement that is empty after editing', () => {
		expect(findSeedIssues(['', '   '], [])).toEqual(['empty', 'empty']);
	});

	it('flags the copies of a statement repeated in the file, not the first', () => {
		expect(findSeedIssues(['Same', 'Other', 'Same'], [])).toEqual([
			null,
			null,
			'duplicate-in-file'
		]);
	});

	it('flags a statement that is already in the step', () => {
		expect(findSeedIssues(['Already there'], ['Already there'])).toEqual(['duplicate-in-step']);
	});

	it('ignores case and extra whitespace when comparing', () => {
		expect(findSeedIssues(['  already   THERE '], ['Already there'])).toEqual([
			'duplicate-in-step'
		]);
	});

	it('prefers the in-file flag for later copies of a statement already in the step', () => {
		expect(findSeedIssues(['Same', 'Same'], ['Same'])).toEqual([
			'duplicate-in-step',
			'duplicate-in-file'
		]);
	});
});
