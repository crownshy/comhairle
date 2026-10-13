import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { Translation } from '@crownshy/api-client/api';
import type { Locale } from '$lib/paraglide/runtime';
import { createTextContentSource } from './translationSource.svelte';
import { saveTranslation } from './translationUtils';
import { notifications } from '$lib/notifications.svelte';

vi.mock('$app/navigation', () => ({ invalidateAll: vi.fn() }));
vi.mock('$lib/notifications.svelte', () => ({ notifications: { send: vi.fn() } }));
vi.mock('./translationUtils', async (importOriginal) => ({
	...(await importOriginal<typeof import('./translationUtils')>()),
	saveTranslation: vi.fn(async () => {}),
	markOtherTranslationsAsDraft: vi.fn(async () => {})
}));

function translationWith(id: string, content: string) {
	return {
		textContent: { id },
		textTranslations: [{ locale: 'en', content }]
	} as unknown as Translation;
}

function createSource(options: Partial<Parameters<typeof createTextContentSource>[0]> = {}) {
	const refresh = vi.fn(async () => {});
	const source = createTextContentSource({
		getTranslation: () => translationWith('text-1', 'original'),
		getPrimaryLocale: () => 'en' as Locale,
		getSupportedLanguages: () => ['en'] as Locale[],
		refresh,
		...options
	});
	return { source, refresh };
}

describe('createTextContentSource', () => {
	beforeEach(() => {
		vi.mocked(saveTranslation).mockReset().mockResolvedValue(undefined);
		vi.mocked(notifications.send).mockReset();
	});

	it('saves without reloading, and keeps showing what was typed', async () => {
		const { source, refresh } = createSource();

		source.saveSource('edited');
		await source.flush();

		expect(saveTranslation).toHaveBeenCalledWith('text-1', 'en', 'edited', {
			requiresValidation: false
		});
		expect(refresh).not.toHaveBeenCalled();
		expect(source.contents.en).toBe('edited');
		expect(source.stale).toBe(true);
	});

	it('reloads once on sync, then is no longer stale', async () => {
		const { source, refresh } = createSource();

		source.saveSource('first');
		source.saveSource('second');
		await source.sync();

		expect(saveTranslation).toHaveBeenCalledTimes(1);
		expect(refresh).toHaveBeenCalledTimes(1);
		expect(source.stale).toBe(false);
	});

	it('does not reload on sync when nothing was saved', async () => {
		const { source, refresh } = createSource();

		await source.sync();

		expect(refresh).not.toHaveBeenCalled();
	});

	it('reports a failed save once, keeps the text, and saves it again on retry', async () => {
		vi.mocked(saveTranslation).mockRejectedValueOnce(new Error('offline'));
		const { source, refresh } = createSource();

		source.saveSource('edited');
		await source.flush();

		expect(source.saveState).toBe('error');
		expect(source.contents.en).toBe('edited');
		expect(notifications.send).toHaveBeenCalledTimes(1);

		await source.sync();
		expect(refresh).not.toHaveBeenCalled();

		await source.retry();

		expect(saveTranslation).toHaveBeenCalledTimes(2);
		expect(source.saveState).toBe('saved');
	});

	it('uses the id from ensureTextContentId for later saves', async () => {
		const ensureTextContentId = vi.fn(async () => 'created-1');
		const { source } = createSource({
			getTranslation: () => undefined,
			ensureTextContentId
		});

		source.saveSource('first');
		await source.flush();
		source.saveSource('second');
		await source.flush();

		expect(ensureTextContentId).toHaveBeenCalledTimes(1);
		expect(saveTranslation).toHaveBeenCalledWith('created-1', 'en', 'second', {
			requiresValidation: false
		});
	});
});
