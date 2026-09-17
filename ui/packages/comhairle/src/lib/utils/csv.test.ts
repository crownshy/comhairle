import { describe, expect, it } from 'vitest';
import { csvField, toCsv } from './csv';

describe('csvField', () => {
	it('quotes values and doubles internal quotes', () => {
		expect(csvField('say "hi"')).toBe('"say ""hi"""');
		expect(csvField(3)).toBe('"3"');
		expect(csvField(false)).toBe('"false"');
	});

	it('leaves null and undefined empty', () => {
		expect(csvField(null)).toBe('');
		expect(csvField(undefined)).toBe('');
	});

	it('normalises Windows and old Mac newlines', () => {
		expect(csvField('a\r\nb\rc')).toBe('"a\nb\nc"');
	});

	it('defuses text a spreadsheet would run as a formula', () => {
		expect(csvField('=HYPERLINK("x")')).toBe('"\'=HYPERLINK(""x"")"');
		expect(csvField('+1')).toBe('"\'+1"');
		expect(csvField('-cmd')).toBe('"\'-cmd"');
		expect(csvField('@SUM(A1)')).toBe('"\'@SUM(A1)"');
	});

	it('does not touch negative numbers', () => {
		expect(csvField(-1)).toBe('"-1"');
	});
});

describe('toCsv', () => {
	it('joins cells with commas and rows with newlines', () => {
		expect(
			toCsv([
				['id', 'text'],
				[1, 'hello, world']
			])
		).toBe('"id","text"\n"1","hello, world"');
	});
});
