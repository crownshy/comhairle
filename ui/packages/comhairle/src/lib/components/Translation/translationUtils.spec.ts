// @vitest-environment jsdom

import { http, HttpResponse } from 'msw';
import { setupServer } from 'msw/node';
import { describe, it, afterAll, beforeAll, afterEach, vi, expect } from 'vitest';
import {
	resolveTranslatableJsonToTextContentIds,
	traverseTranslatableJsonAndCreateTranslations
} from './translationUtils';

const NIL_UUID = '00000000-0000-0000-0000-000000000000';

const DUMMY_TEXT_CONTENT = {
	id: NIL_UUID,
	primary_locale: 'en',
	format: 'plain'
};

const DUMMY_TRANSLATIONS_DTO = {
	textContent: DUMMY_TEXT_CONTENT,
	textTranslations: [
		{
			id: NIL_UUID,
			contentId: NIL_UUID,
			locale: 'en',
			content: 'DUMMY TEXT',
			aiGenerated: false,
			requiresValidation: false
		}
	]
};

const BASE_URL = 'http://localhost:3000';

export const handlers = [
	http.post(`${BASE_URL}/api/translations`, () => {
		return HttpResponse.json({
			id: NIL_UUID,
			primaryLocale: 'en',
			format: 'plain'
		});
	})
];

const server = setupServer(...handlers);

beforeAll(() => server.listen({ onUnhandledRequest: 'error' }));

afterEach(() => server.resetHandlers());

afterAll(() => server.close());

describe('Translation utils', () => {
	describe('traverseTranslatableJsonAndCreateTranslations', () => {
		it('Creates translations for translatable fields in nested JSON object', async () => {
			const dispatchRequest = vi.fn();
			server.events.on('request:start', dispatchRequest);

			const payload = {
				title: { localized: 'New title' },
				existing: {
					localized: 'Existing field',
					translations: DUMMY_TRANSLATIONS_DTO
				},
				questions: [
					{ label: { localized: 'First question' }, value: 2 },
					{
						label: {
							localized: 'Second question',
							translations: DUMMY_TRANSLATIONS_DTO
						},
						value: 2
					}
				],
				nested: {
					name: {
						localized: 'Nested name'
					}
				}
			};

			const result = await traverseTranslatableJsonAndCreateTranslations(payload, 'en');

			expect(dispatchRequest).toHaveBeenCalledTimes(3);
			expect(typeof result.title).toBe('string');
			expect(typeof result.nested.name).toBe('string');
			expect(typeof result.existing).toBe('object');
			expect(result.existing.localized).toBe('Existing field');
			expect(Array.isArray(result.existing.translations.textTranslations)).toBe(true);
			expect(result.existing.translations).toBeTruthy();
		});

		it('Creates translations for new translatable fields in nested JSON array', async () => {
			const dispatchRequest = vi.fn();
			server.events.on('request:start', dispatchRequest);

			const payload = [
				{
					title: { localized: 'first title' },
					existing: {
						localized: 'existing',
						translations: DUMMY_TRANSLATIONS_DTO
					},
					nested: {
						name: {
							localized: 'first nested name'
						}
					}
				},
				{
					title: { localized: 'second title' },
					existing: {
						localized: 'existing',
						translations: DUMMY_TRANSLATIONS_DTO
					},
					nested: {
						name: {
							localized: 'second nested name'
						}
					}
				},
				{
					title: { localized: 'third title' },
					existing: {
						localized: 'existing',
						translations: DUMMY_TRANSLATIONS_DTO
					},
					nested: {
						name: {
							localized: 'third nested name'
						}
					}
				}
			];

			const result = await traverseTranslatableJsonAndCreateTranslations(payload, 'en');

			expect(dispatchRequest).toHaveBeenCalledTimes(6);

			for (const item of result) {
				expect(typeof item.title).toBe('string');
				expect(typeof item.nested.name).toBe('string');
			}
		});
	});

	describe('resolveTranslatableJsonToTextContentIds', () => {
		it('Resolves all fields with translations to raw textContentId in nested JSON object', () => {
			const payload = {
				title: { localized: 'test title', translations: DUMMY_TRANSLATIONS_DTO },
				otherField: 'other field',
				nestedArr: [
					{
						name: { localized: 'first', translations: DUMMY_TRANSLATIONS_DTO }
					},
					{
						name: { localized: 'second', translations: DUMMY_TRANSLATIONS_DTO }
					}
				]
			};

			const result = resolveTranslatableJsonToTextContentIds(payload);

			expect(result.title).toBe(NIL_UUID);
			expect(result.otherField).toBe('other field');
			expect(result.nestedArr[0].name).toBe(NIL_UUID);
			expect(result.nestedArr[1].name).toBe(NIL_UUID);
		});

		it('Resolves all fields with translations to raw textContentId in nested JSON array', () => {
			const payload = [
				{
					title: { localized: 'first title', translations: DUMMY_TRANSLATIONS_DTO },
					other: 'other',
					nested: {
						name: {
							localized: 'first nested',
							translations: DUMMY_TRANSLATIONS_DTO
						}
					}
				},
				{
					title: { localized: 'second title', translations: DUMMY_TRANSLATIONS_DTO },
					other: 'other',
					nested: {
						name: {
							localized: 'second nested',
							translations: DUMMY_TRANSLATIONS_DTO
						}
					}
				},
				{
					title: { localized: 'third title', translations: DUMMY_TRANSLATIONS_DTO },
					other: 'other',
					nested: {
						name: {
							localized: 'third nested',
							translations: DUMMY_TRANSLATIONS_DTO
						}
					}
				}
			];

			const result = resolveTranslatableJsonToTextContentIds(payload);

			for (const item of result) {
				expect(item.title).toBe(NIL_UUID);
				expect(item.other).toBe('other');
				expect(item.nested.name).toBe(NIL_UUID);
			}
		});
	});
});
