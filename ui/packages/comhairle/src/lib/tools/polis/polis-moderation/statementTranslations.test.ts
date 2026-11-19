import { describe, expect, it, vi } from 'vitest';
import type { PolisStatementTranslation } from '@crownshy/api-client/api';

vi.mock('$lib/paraglide/runtime', () => ({ getLocale: () => 'en' }));

const { isForeign, languageName, needsReview, translationForReader } =
	await import('./statementTranslations');

function translation(locale: string, flags: Partial<PolisStatementTranslation> = {}) {
	return {
		id: `t-${locale}`,
		polis_statement_aux_id: 'aux',
		locale,
		content: `in ${locale}`,
		ai_generated: true,
		requires_validation: true,
		created_at: '2026-11-17T00:00:00Z',
		updated_at: '2026-11-17T00:00:00Z',
		...flags
	} satisfies PolisStatementTranslation;
}

describe('translationForReader', () => {
	const translations = [translation('en'), translation('gd')];

	it('returns the reader-language translation for a statement written elsewhere', () => {
		expect(translationForReader('es', translations, 'en')?.locale).toBe('en');
	});

	it('returns null when the statement is already in the reader language', () => {
		expect(translationForReader('en-GB', translations, 'en')).toBeNull();
	});

	it('returns null when the source language is unknown', () => {
		expect(translationForReader(null, translations, 'en')).toBeNull();
	});

	it('returns null when no translation into the reader language exists yet', () => {
		expect(translationForReader('es', [translation('gd')], 'en')).toBeNull();
	});
});

describe('needsReview', () => {
	it('is true only for unverified machine translations', () => {
		expect(needsReview(translation('en'))).toBe(true);
		expect(needsReview(translation('en', { requires_validation: false }))).toBe(false);
		expect(needsReview(translation('en', { ai_generated: false }))).toBe(false);
	});
});

describe('isForeign', () => {
	it('compares base languages', () => {
		expect(isForeign('es', 'en')).toBe(true);
		expect(isForeign('en-GB', 'en')).toBe(false);
		expect(isForeign(undefined, 'en')).toBe(false);
	});
});

describe('languageName', () => {
	it('names the language in the display locale', () => {
		expect(languageName('es', 'en')).toBe('Spanish');
	});
});
