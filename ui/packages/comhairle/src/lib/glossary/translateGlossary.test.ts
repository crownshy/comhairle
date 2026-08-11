import { describe, it, expect, vi } from 'vitest';
import { translateGlossaryToLocale } from './translateGlossary';

// A fake translator that upper-cases and tags the target locale, so assertions are simple.
const fakeTranslate = (text: string, target: string) => Promise.resolve(`${text} [${target}]`);

describe('translateGlossaryToLocale', () => {
	it('fills missing target terms and tooltip from the primary locale', async () => {
		const glossary = [{ text: { en: ['bus', 'coach'] }, tooltip: { en: 'A vehicle' } }];
		const result = await translateGlossaryToLocale(glossary, 'es', 'en', fakeTranslate);
		expect(result).toEqual([
			{
				text: { en: ['bus', 'coach'], es: ['bus', 'coach [es]'] },
				tooltip: { en: 'A vehicle', es: 'A vehicle [es]' }
			}
		]);
	});

	it('never overwrites existing target-locale text', async () => {
		const glossary = [
			{
				text: { en: ['bus'], es: ['autobús'] },
				tooltip: { en: 'A vehicle', es: 'Un vehículo' }
			}
		];
		const translate = vi.fn(fakeTranslate);
		const result = await translateGlossaryToLocale(glossary, 'es', 'en', translate);
		expect(result).toEqual(glossary);
		expect(translate).not.toHaveBeenCalled();
	});

	it('returns the glossary unchanged when target is the primary locale', async () => {
		const glossary = [{ text: { en: ['bus'] }, tooltip: { en: 'A vehicle' } }];
		const translate = vi.fn(fakeTranslate);
		expect(await translateGlossaryToLocale(glossary, 'en', 'en', translate)).toBe(glossary);
		expect(translate).not.toHaveBeenCalled();
	});

	it('leaves an entry unchanged when its translation throws', async () => {
		const glossary = [{ text: { en: ['bus'] }, tooltip: { en: 'A vehicle' } }];
		const failing = () => Promise.reject(new Error('service down'));
		const result = await translateGlossaryToLocale(glossary, 'es', 'en', failing);
		expect(result).toEqual([{ text: { en: ['bus'] }, tooltip: { en: 'A vehicle' } }]);
	});
});
