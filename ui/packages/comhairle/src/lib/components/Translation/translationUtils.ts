import { apiClient } from '@crownshy/api-client/client';
import { notifications } from '$lib/notifications.svelte';
import { getLanguageName } from '$lib/config/languages';
import {
	extractTextFromTiptap,
	translateTiptapContent,
	isTiptapJson
} from '$lib/utils/tiptapUtils';
import type { Translation, Translation2 } from '@crownshy/api-client/api';

export type TranslationStatus = 'primary' | 'draft' | 'approved';

/** Lifecycle of a background save, so the UI can show a truthful indicator instead of guessing. */
export type SaveState = 'idle' | 'saving' | 'saved' | 'error';

export interface TranslationEntry {
	language: string;
	languageName: string;
	status: TranslationStatus;
	content: string;
}

/**
 * The single contract every translatable field is driven by (see ADR-0005). A source owns both
 * reading a field's per-locale content/status and persisting edits, so `TranslatableField` and
 * `TranslationEditor` can be dumb views that never know whether they're backed by a `TextContent`
 * entity or by learn's inline page model.
 *
 * Reads are reactive getters. `contents` always reflects the latest keystroke immediately (the
 * implementation holds an optimistic copy), which is what stops `RichTextEditor` from fighting the
 * cursor. `saveSource`/`saveTarget` are debounced (driven by typing); `aiTranslate`/`approve`/
 * `markAsDraft` are immediate (discrete actions). `flush()` commits any pending debounced save and
 * awaits the in-flight one, so callers can safely leave a page/field.
 */
export interface TranslationSource {
	/** Per-locale content, including the primary locale, latest edit reflected immediately. */
	readonly contents: Record<string, string>;
	/** Per-locale status: the primary locale is `'primary'`, others are `'draft'` / `'approved'`. */
	readonly statuses: Record<string, TranslationStatus>;
	/** Lifecycle of the background save, for a truthful "Saving / Saved" indicator. */
	readonly saveState: SaveState;
	/** Persist the primary-locale content (debounced). */
	saveSource(content: string): void;
	/** Persist a target-locale translation (debounced). */
	saveTarget(locale: string, content: string): void;
	/** Machine-translate `sourceContent` into `locale`, persist it, and return the result. */
	aiTranslate(
		locale: string,
		sourceContent: string
	): Promise<{ content: string; requiresValidation: boolean }>;
	/** Mark a target locale approved (no longer requires validation). */
	approve(locale: string): Promise<void>;
	/** Mark a target locale back to draft (requires validation). */
	markAsDraft(locale: string): Promise<void>;
	/** Run any pending debounced save now and await the in-flight one. */
	flush(): Promise<void>;
}

/**
 * True while a source still has an edit in flight (or a failed save), i.e. there are unsaved changes.
 * Handy for an unsaved-changes guard: `guardUnsavedChanges(() => sources.some(hasUnsavedChanges))`.
 */
export function hasUnsavedChanges(source: TranslationSource): boolean {
	return source.saveState === 'saving' || source.saveState === 'error';
}

export function getTextInLocale(
	translation: Translation | Translation2 | undefined,
	locale: string,
	fallback: string = ''
): string {
	if (!translation?.textTranslations) return fallback;
	const localeTranslation = translation.textTranslations.find((t) => t.locale === locale);
	return localeTranslation?.content ?? fallback;
}

export const statusToBadgeVariant = {
	primary: 'outline',
	draft: 'draft',
	approved: 'default'
} as const;

export function deriveStatus(isPrimary: boolean, requiresValidation?: boolean): TranslationStatus {
	if (isPrimary) return 'primary';
	if (requiresValidation === undefined) return 'draft';
	return requiresValidation ? 'draft' : 'approved';
}

export async function saveTranslation(
	textContentId: string,
	locale: string,
	content: string,
	options: {
		aiGenerated?: boolean;
		requiresValidation?: boolean;
	} = {}
): Promise<void> {
	const { aiGenerated = false, requiresValidation = true } = options;

	await apiClient.CreateOrUpdateTextTranslation(
		{
			content,
			ai_generated: aiGenerated,
			requires_validation: requiresValidation
		},
		{
			params: {
				text_content_id: textContentId,
				locale
			}
		}
	);
}

export async function aiTranslate(
	textContentId: string,
	targetLocale: string,
	sourceContent: string,
	primaryLocale: string = 'en'
): Promise<{ content: string; requiresValidation: boolean }> {
	const isRichText = isTiptapJson(sourceContent);

	// For rich text, we need to:
	// 1. Extract plain text for translation
	// 2. Create a temporary text content for the plain text
	// 3. Translate it
	// 4. Map the translation back to the TipTap structure

	if (isRichText) {
		const plainText = extractTextFromTiptap(sourceContent);

		const tempTextContent = await apiClient.CreateTextContent({
			primary_locale: primaryLocale,
			format: 'plain',
			content: plainText
		});

		try {
			await saveTranslation(tempTextContent.id, targetLocale, '', {
				aiGenerated: true,
				requiresValidation: true
			});

			const result = await apiClient.AutomaticallyGenerateTranslation(undefined, {
				params: {
					text_content_id: tempTextContent.id,
					locale: targetLocale
				}
			});

			const translatedContent = translateTiptapContent(sourceContent, result.content);

			await saveTranslation(textContentId, targetLocale, translatedContent, {
				aiGenerated: true,
				requiresValidation: true
			});

			return {
				content: translatedContent,
				requiresValidation: true
			};
		} finally {
			try {
				await apiClient.DeleteTextContent(undefined, {
					params: { text_content_id: tempTextContent.id }
				});
			} catch (e) {
				console.warn('Failed to delete temporary text content:', e);
			}
		}
	} else {
		// Plain text: use direct translation
		await saveTranslation(textContentId, targetLocale, '', {
			aiGenerated: true,
			requiresValidation: true
		});

		const result = await apiClient.AutomaticallyGenerateTranslation(undefined, {
			params: {
				text_content_id: textContentId,
				locale: targetLocale
			}
		});

		return {
			content: result.content,
			requiresValidation: result.requiresValidation
		};
	}
}

export async function markOtherTranslationsAsDraft(
	textContentId: string,
	primaryLocale: string,
	translations: TranslationEntry[]
): Promise<void> {
	const otherTranslations = translations.filter((t) => t.language !== primaryLocale && t.content);

	await Promise.all(
		otherTranslations.map((t) =>
			saveTranslation(textContentId, t.language, t.content, {
				aiGenerated: false,
				requiresValidation: true
			})
		)
	);
}

export async function aiTranslateContent(
	sourceContent: string,
	targetLocale: string,
	primaryLocale: string = 'en'
): Promise<string> {
	const isRichText = isTiptapJson(sourceContent);
	const plainText = isRichText ? extractTextFromTiptap(sourceContent) : sourceContent;

	const tempTextContent = await apiClient.CreateTextContent({
		primary_locale: primaryLocale,
		format: 'plain',
		content: plainText
	});

	try {
		await saveTranslation(tempTextContent.id, targetLocale, '', {
			aiGenerated: true,
			requiresValidation: true
		});

		const result = await apiClient.AutomaticallyGenerateTranslation(undefined, {
			params: {
				text_content_id: tempTextContent.id,
				locale: targetLocale
			}
		});

		return isRichText ? translateTiptapContent(sourceContent, result.content) : result.content;
	} finally {
		try {
			await apiClient.DeleteTextContent(undefined, {
				params: { text_content_id: tempTextContent.id }
			});
		} catch (e) {
			console.warn('Failed to delete temporary text content:', e);
		}
	}
}

export async function autoTranslateNewLanguage(
	locale: string,
	textContentIds: string[]
): Promise<{ textContentId: string; success: boolean }[]> {
	const results: { textContentId: string; success: boolean }[] = [];

	for (const textContentId of textContentIds) {
		try {
			await saveTranslation(textContentId, locale, '', {
				aiGenerated: true,
				requiresValidation: true
			});

			await aiTranslate(textContentId, locale, '');
			results.push({ textContentId, success: true });
		} catch (error) {
			console.error(`Failed to auto-translate ${textContentId} to ${locale}:`, error);
			results.push({ textContentId, success: false });
		}
	}

	return results;
}
