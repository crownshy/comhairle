import { describe, it, expect } from 'vitest';
import {
	parseLocalizedGlossary,
	localizedGlossaryFromMetadata,
	resolveGlossary,
	resolveGlossaryFromMetadata
} from './localizedGlossary';

describe('parseLocalizedGlossary', () => {
	it('keeps per-locale terms and tooltips', () => {
		expect(
			parseLocalizedGlossary(
				[
					{
						text: { en: ['bus'], es: ['autobús'] },
						tooltip: { en: 'A vehicle', es: 'Un vehículo' }
					}
				],
				'en'
			)
		).toEqual([
			{
				text: { en: ['bus'], es: ['autobús'] },
				tooltip: { en: 'A vehicle', es: 'Un vehículo' }
			}
		]);
	});

	it('treats a flat term array and a bare tooltip as the primary locale (back-compat)', () => {
		expect(
			parseLocalizedGlossary([{ text: ['bus', 'autobus'], tooltip: 'A vehicle' }], 'en')
		).toEqual([{ text: { en: ['bus', 'autobus'] }, tooltip: { en: 'A vehicle' } }]);
	});

	it('trims and drops empty terms and tooltips per locale', () => {
		expect(
			parseLocalizedGlossary(
				[
					{
						text: { en: [' bus ', ''], es: ['  '] },
						tooltip: { en: '  A vehicle  ', es: '  ' }
					}
				],
				'en'
			)
		).toEqual([{ text: { en: ['bus'] }, tooltip: { en: 'A vehicle' } }]);
	});

	it('drops entries with no term or no tooltip', () => {
		expect(
			parseLocalizedGlossary(
				[
					{ text: {}, tooltip: { en: 'orphan' } },
					{ text: { en: ['bus'] }, tooltip: {} },
					{ text: { en: ['bus'] }, tooltip: { en: 'ok' } }
				],
				'en'
			)
		).toEqual([{ text: { en: ['bus'] }, tooltip: { en: 'ok' } }]);
	});

	it.each([null, 'x', 5, {}])('returns [] for non-array %p', (value) => {
		expect(parseLocalizedGlossary(value, 'en')).toEqual([]);
	});
});

describe('resolveGlossary', () => {
	const entries = [
		{ text: { en: ['bus'], es: ['autobús'] }, tooltip: { en: 'A vehicle', es: 'Un vehículo' } }
	];

	it('picks the requested locale for both terms and tooltip', () => {
		expect(resolveGlossary(entries, 'es', 'en')).toEqual([
			{ text: ['autobús'], tooltip: 'Un vehículo' }
		]);
	});

	it('falls back to the primary locale terms when the requested locale has none', () => {
		const partial = [
			{ text: { en: ['cookie'] }, tooltip: { en: 'A biscuit', es: 'Una galleta' } }
		];
		// No Spanish terms, so it matches the English term but shows the Spanish tooltip.
		expect(resolveGlossary(partial, 'es', 'en')).toEqual([
			{ text: ['cookie'], tooltip: 'Una galleta' }
		]);
	});

	it('falls back to the primary tooltip when the requested locale has none', () => {
		const partial = [{ text: { en: ['bus'], es: ['autobús'] }, tooltip: { en: 'A vehicle' } }];
		expect(resolveGlossary(partial, 'es', 'en')).toEqual([
			{ text: ['autobús'], tooltip: 'A vehicle' }
		]);
	});
});

describe('metadata helpers', () => {
	it('resolveGlossaryFromMetadata reads and resolves in one step', () => {
		const metadata = {
			glossary: [
				{
					text: { en: ['bus'], es: ['autobús'] },
					tooltip: { en: 'A vehicle', es: 'Un vehículo' }
				}
			]
		};
		expect(resolveGlossaryFromMetadata(metadata, 'es', 'en')).toEqual([
			{ text: ['autobús'], tooltip: 'Un vehículo' }
		]);
	});

	it('resolves legacy flat data against the primary locale', () => {
		const metadata = { glossary: [{ text: ['bus'], tooltip: 'A vehicle' }] };
		expect(resolveGlossaryFromMetadata(metadata, 'en', 'en')).toEqual([
			{ text: ['bus'], tooltip: 'A vehicle' }
		]);
		expect(localizedGlossaryFromMetadata(metadata, 'en')).toEqual([
			{ text: { en: ['bus'] }, tooltip: { en: 'A vehicle' } }
		]);
	});
});
