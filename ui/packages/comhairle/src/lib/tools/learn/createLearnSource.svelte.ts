import Pages from './Pages.svelte';
import {
	aiTranslateContent,
	type TranslationSource,
	type TranslationStatus
} from '$lib/components/Translation/translationUtils';

type LearnSourceOptions = {
	pages: Pages;
	getPrimaryLocale: () => string;
	getSupportedLanguages: () => string[];
};

/**
 * Projects the **current page** of a {@link Pages} collection onto the shared {@link TranslationSource}
 * contract, so learn's inline-JSON page model plugs into `TranslatableField` exactly like a
 * `TextContent`-backed field does (see ADR-0005).
 *
 * `Pages` already holds an optimistic in-memory model and an observable/flushable save state, so this
 * adapter is thin: reads project the current page; writes and `flush` delegate straight through.
 * Construct it during `LearnManage` init and pass the result to `TranslatableField source={...}`.
 */
export function createLearnSource(options: LearnSourceOptions): TranslationSource {
	const { pages, getPrimaryLocale, getSupportedLanguages } = options;

	const otherLanguages = () => getSupportedLanguages().filter((l) => l !== getPrimaryLocale());

	const contents = $derived.by((): Record<string, string> => {
		const primaryLocale = getPrimaryLocale();
		const page = pages.items[pages.currentId] ?? {};
		const result: Record<string, string> = {
			[primaryLocale]: page[primaryLocale]?.content ?? ''
		};
		for (const locale of otherLanguages()) {
			result[locale] = page[locale]?.content ?? '';
		}
		return result;
	});

	const statuses = $derived.by((): Record<string, TranslationStatus> => {
		const primaryLocale = getPrimaryLocale();
		const page = pages.items[pages.currentId] ?? {};
		const result: Record<string, TranslationStatus> = { [primaryLocale]: 'primary' };
		for (const locale of otherLanguages()) {
			const translation = page[locale];
			result[locale] =
				translation && translation.requires_validation === false ? 'approved' : 'draft';
		}
		return result;
	});

	return {
		get contents() {
			return contents;
		},
		get statuses() {
			return statuses;
		},
		get saveState() {
			return pages.saveState;
		},

		saveSource(content: string) {
			pages.current.upsertContent('source', getPrimaryLocale(), content);
		},

		saveTarget(locale: string, content: string) {
			pages.current.upsertContent('target', locale, content);
		},

		async aiTranslate(locale: string, sourceContent: string) {
			const translated = await aiTranslateContent(sourceContent, locale, getPrimaryLocale());
			await pages.current.upsertContent('target', locale, translated);
			return { content: translated, requiresValidation: true };
		},

		async approve(locale: string) {
			await pages.current.approve(locale, true);
		},

		async markAsDraft(locale: string) {
			await pages.current.approve(locale, false);
		},

		flush() {
			return pages.flush();
		}
	};
}
