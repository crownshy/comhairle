import { apiClient } from '@crownshy/api-client/client';
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

/**
 * Traverses a nested JSON structure with translatable fields, creating translations
 * for any new translatable fields in draft mode and replacing the value with the
 * resulting `textContentId`, to allow the JSON to be updated with the correct
 * `textContentId` reference.
 *
 * Translatable fields within a nested JSON structure (e.g. within a `workflowStep`'s
 * `toolConfig` / `previewToolConfig`) have the translations colocated with the target
 * field to simplify usage with the [`TranslatableField`] component and the existing
 * [`createTextContentSource`] adaptor.
 *
 * When a new translatable field is added to the existing JSON structure, these
 * fields will be in a draft mode and contain only the localized text minus the
 * translations.
 *
 * @example
 * ```ts
 * {
 *	name: { localized: 'New field' }
 * }
 * ```
 *
 * Vs existing fields with translations:
 *
 * @example
 * ```ts
 * {
 *	 name: {
 *		localized: 'Existing field',
 *		translations: {
 *			textContent: { ... },
 *			textTranslations: [ ... ]
 *		}
 *	 }
 * }
 * ```
 *
 * Translations (i.e. `text_content` + `text_translations` database records)
 * need to be created for these draft fields before the JSON structure can be
 * saved with the resulting `textContentId`.
 */
export async function traverseTranslatableJsonAndCreateTranslations(
	/* eslint-disable-next-line  @typescript-eslint/no-explicit-any */
	value: any,
	primaryLocale: string
	/* eslint-disable-next-line  @typescript-eslint/no-explicit-any */
): Promise<any> {
	if (Array.isArray(value)) {
		return Promise.all(
			value.map((v) => traverseTranslatableJsonAndCreateTranslations(v, primaryLocale))
		);
	}

	if (value !== null && typeof value === 'object') {
		if (isDraftTranslatableField(value)) {
			console.log('Draft translatable field: ', value);
			// create new translation and return textContentId
			const textContentRes = await apiClient.CreateTextContent({
				content: value.localized,
				format: 'plain',
				primary_locale: primaryLocale
			});

			return textContentRes.id;
		}

		/* eslint-disable-next-line  @typescript-eslint/no-explicit-any */
		const fields: Record<string, any> = {};
		for (const [key, val] of Object.entries(value)) {
			fields[key] = await traverseTranslatableJsonAndCreateTranslations(val, primaryLocale);
		}
		return fields;
	}

	return value;
}

/**
 * Determines whether a TranslatableJson field is in a draft mode, eg
 *
 * @example
 * ```ts
 * {
 *	name: { localized: 'New field' }
 * }
 * ```
 *
 * where new translations (text_content + text_translations) need to be created
 * before updating the JSON object as opposed to an existing TranslatableJson
 * field:
 *
 * @example
 * ```ts
 * {
 *	 name: {
 *		localized: 'Existing field',
 *		translations: {
 *			textContent: { ... },
 *			textTranslations: [ ... ]
 *		}
 *	 }
 * }
 * ```
 *
 * where updates can occur via the [`TranslatableField`] component.
 */
/* eslint-disable-next-line  @typescript-eslint/no-explicit-any */
function isDraftTranslatableField(value: any) {
	return (
		value !== null &&
		typeof value === 'object' &&
		typeof value.localized === 'string' &&
		'translations' in (value as object) === false
	);
}

/**
 * Strips translations from nested JSON structures with colocated translatable
 * fields and replaces value with appropriate `textContentId` to allow updates.
 *
 * Read endpoints for translatable JSON structures (e.g. `workflowStep`'s
 * `toolConfig` / `previewToolConfig`) return JSON with translations colocated
 * to target field.
 *
 * @example
 * ```ts
 * {
 *	 name: {
 *		localized: 'Translatable field',
 *		translations: {
 *			textContent: { ... },
 *			textTranslations: [ ... ]
 *		}
 *	 }
 * }
 * ```
 *
 * This provides ease with call sites of the [`createTextContentSource`] adaptor
 * for the [`TranslatableField`] component.
 *
 * Update endpoints for resources with nested translatable JSON fields expect
 * those nested fields to contain only the target `textContentId`.
 *
 * ## Usage
 *
 * @example
 * ```ts
 * const updatePayload = resolveTranslatableJsonToTextContentIds(target);
 * ```
 *
 * Transforms fields to:
 *
 * @example
 * ```ts
 * {
 *	 name: 'abcdefgh-1234-1234-1234-abcdefghijkl'
 * }
 * ```
 */
/* eslint-disable-next-line  @typescript-eslint/no-explicit-any */
export function resolveTranslatableJsonToTextContentIds(value: any): any {
	if (Array.isArray(value)) {
		return value.map(resolveTranslatableJsonToTextContentIds);
	}

	if (value !== null && typeof value === 'object') {
		if (isExistingTranslatableField(value)) {
			return value.translations.textContent.id;
		}

		/* eslint-disable  @typescript-eslint/no-explicit-any */
		const result: Record<string, any> = {};
		for (const [key, val] of Object.entries(value)) {
			result[key] = resolveTranslatableJsonToTextContentIds(val);
		}
		return result;
	}

	// primitives (string, number, boolean, null) pass through unchanged
	return value;
}

function isExistingTranslatableField(value: any) {
	return (
		typeof value.localized === 'string' &&
		value.translations &&
		typeof value.translations === 'object' &&
		value.translations.textContent &&
		typeof value.translations.textContent.id === 'string'
	);
}
