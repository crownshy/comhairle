import { describe, it, expect } from 'vitest';
import { parseGlossaryCsv } from './glossaryCsv';

describe('parseGlossaryCsv', () => {
	it('maps two columns to a glossary entry', async () => {
		expect(await parseGlossaryCsv('bus,A vehicle that carries people')).toEqual([
			{ text: ['bus'], tooltip: 'A vehicle that carries people' }
		]);
	});

	it('splits synonyms in the first column on ; or |', async () => {
		expect(await parseGlossaryCsv('bus; autobus | coach,A vehicle')).toEqual([
			{ text: ['bus', 'autobus', 'coach'], tooltip: 'A vehicle' }
		]);
	});

	it('skips a header row', async () => {
		const csv = 'Term,Definition\nbus,A vehicle\nreferral,Passed to another team';
		expect(await parseGlossaryCsv(csv)).toEqual([
			{ text: ['bus'], tooltip: 'A vehicle' },
			{ text: ['referral'], tooltip: 'Passed to another team' }
		]);
	});

	it('skips the header row even when the file starts with a UTF-8 BOM', async () => {
		const csv = '\ufeffTerm,Definition\nbus,A vehicle';
		expect(await parseGlossaryCsv(csv)).toEqual([{ text: ['bus'], tooltip: 'A vehicle' }]);
	});

	it('does not treat a data row as a header', async () => {
		expect(await parseGlossaryCsv('bus,A vehicle')).toHaveLength(1);
	});

	it('keeps a definition that itself contains a comma', async () => {
		expect(await parseGlossaryCsv('bus,"A large, shared vehicle"')).toEqual([
			{ text: ['bus'], tooltip: 'A large, shared vehicle' }
		]);
	});

	it('drops rows missing a term or a definition', async () => {
		const csv = 'bus,A vehicle\n,orphan definition\nlonely term,';
		expect(await parseGlossaryCsv(csv)).toEqual([{ text: ['bus'], tooltip: 'A vehicle' }]);
	});

	it('returns [] for empty input', async () => {
		expect(await parseGlossaryCsv('')).toEqual([]);
		expect(await parseGlossaryCsv('\n\n')).toEqual([]);
	});
});
