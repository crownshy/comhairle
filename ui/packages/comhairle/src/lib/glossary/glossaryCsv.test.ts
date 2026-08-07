import { describe, it, expect } from 'vitest';
import { parseCsvRows, parseGlossaryCsv } from './glossaryCsv';

describe('parseCsvRows', () => {
	it('splits simple rows and columns', () => {
		expect(parseCsvRows('a,b\nc,d')).toEqual([
			['a', 'b'],
			['c', 'd']
		]);
	});

	it('keeps commas inside quoted fields', () => {
		expect(parseCsvRows('bus,"A vehicle, large"')).toEqual([['bus', 'A vehicle, large']]);
	});

	it('unescapes doubled quotes inside a quoted field', () => {
		expect(parseCsvRows('bus,"A ""big"" vehicle"')).toEqual([['bus', 'A "big" vehicle']]);
	});

	it('keeps newlines inside quoted fields', () => {
		expect(parseCsvRows('bus,"line one\nline two"')).toEqual([['bus', 'line one\nline two']]);
	});

	it('handles CRLF line endings and a trailing newline', () => {
		expect(parseCsvRows('a,b\r\nc,d\r\n')).toEqual([
			['a', 'b'],
			['c', 'd']
		]);
	});

	it('strips a UTF-8 BOM', () => {
		expect(parseCsvRows('﻿a,b')).toEqual([['a', 'b']]);
	});
});

describe('parseGlossaryCsv', () => {
	it('maps two columns to a glossary entry', () => {
		expect(parseGlossaryCsv('bus,A vehicle that carries people')).toEqual([
			{ text: ['bus'], tooltip: 'A vehicle that carries people' }
		]);
	});

	it('splits synonyms in the first column on ; or |', () => {
		expect(parseGlossaryCsv('bus; autobus | coach,A vehicle')).toEqual([
			{ text: ['bus', 'autobus', 'coach'], tooltip: 'A vehicle' }
		]);
	});

	it('skips a header row', () => {
		const csv = 'Term,Definition\nbus,A vehicle\nreferral,Passed to another team';
		expect(parseGlossaryCsv(csv)).toEqual([
			{ text: ['bus'], tooltip: 'A vehicle' },
			{ text: ['referral'], tooltip: 'Passed to another team' }
		]);
	});

	it('does not treat a data row as a header', () => {
		expect(parseGlossaryCsv('bus,A vehicle')).toHaveLength(1);
	});

	it('keeps a definition that itself contains a comma', () => {
		expect(parseGlossaryCsv('bus,"A large, shared vehicle"')).toEqual([
			{ text: ['bus'], tooltip: 'A large, shared vehicle' }
		]);
	});

	it('drops rows missing a term or a definition', () => {
		const csv = 'bus,A vehicle\n,orphan definition\nlonely term,';
		expect(parseGlossaryCsv(csv)).toEqual([{ text: ['bus'], tooltip: 'A vehicle' }]);
	});

	it('returns [] for empty input', () => {
		expect(parseGlossaryCsv('')).toEqual([]);
		expect(parseGlossaryCsv('\n\n')).toEqual([]);
	});
});
