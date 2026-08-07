import { describe, it, expect } from 'vitest';
import { parseGlossary, glossaryFromMetadata } from './parseGlossary';

describe('parseGlossary', () => {
	it('keeps well-formed entries', () => {
		expect(parseGlossary([{ text: ['bus', 'autobus'], tooltip: 'A vehicle' }])).toEqual([
			{ text: ['bus', 'autobus'], tooltip: 'A vehicle' }
		]);
	});

	it('trims terms and tooltip and drops blank terms', () => {
		expect(parseGlossary([{ text: [' bus ', '', '  '], tooltip: '  A vehicle  ' }])).toEqual([
			{ text: ['bus'], tooltip: 'A vehicle' }
		]);
	});

	it('drops entries with no usable term or no tooltip', () => {
		expect(
			parseGlossary([
				{ text: [], tooltip: 'orphan' },
				{ text: ['bus'], tooltip: '' },
				{ text: ['bus'], tooltip: 'ok' }
			])
		).toEqual([{ text: ['bus'], tooltip: 'ok' }]);
	});

	it('ignores non-string terms and malformed rows', () => {
		expect(
			parseGlossary([{ text: ['bus', 3, null], tooltip: 'A vehicle' }, 'nope', 42, null])
		).toEqual([{ text: ['bus'], tooltip: 'A vehicle' }]);
	});

	it.each([null, undefined, 'string', 42, {}])('returns [] for non-array %p', (value) => {
		expect(parseGlossary(value)).toEqual([]);
	});
});

describe('glossaryFromMetadata', () => {
	it('reads the glossary key out of a metadata object', () => {
		const metadata = { glossary: [{ text: ['bus'], tooltip: 'A vehicle' }], other: 1 };
		expect(glossaryFromMetadata(metadata)).toEqual([{ text: ['bus'], tooltip: 'A vehicle' }]);
	});

	it.each([null, undefined, 'x', 5, {}, { glossary: null }])(
		'returns [] when there is no valid glossary (%p)',
		(metadata) => {
			expect(glossaryFromMetadata(metadata)).toEqual([]);
		}
	);
});
