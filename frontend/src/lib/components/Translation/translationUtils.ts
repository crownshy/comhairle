import { apiClient } from '$lib/api/client';
import { notifications } from '$lib/notifications.svelte';
import { getLanguageName } from '$lib/config/languages';
import { extractTextFromTiptap, translateTiptapContent, isTiptapJson } from '$lib/utils/tiptapUtils';

export type TranslationStatus = 'primary' | 'draft' | 'approved';

export interface TranslationEntry {
	language: string;
	languageName: string;
	status: TranslationStatus;
	content: string;
}

export const statusToBadgeVariant = {
	primary: 'outline',
	draft: 'secondary',
	approved: 'default'
} as const;

export function deriveStatus(
	isPrimary: boolean,
	requiresValidation?: boolean
): TranslationStatus {
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
			requiresValidation: result.requires_validation
		};
	}
}

export async function markOtherTranslationsAsDraft(
	textContentId: string,
	primaryLocale: string,
	translations: TranslationEntry[]
): Promise<void> {
	const otherTranslations = translations.filter(
		t => t.language !== primaryLocale && t.content
	);
	
	await Promise.all(
		otherTranslations.map(t =>
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
		
		return isRichText
			? translateTiptapContent(sourceContent, result.content)
			: result.content;
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


export function createDebouncer(delayMs: number = 500) {
	let timeout: ReturnType<typeof setTimeout>;
	let pendingFn: (() => void | Promise<void>) | null = null;
	
	return {
		debounce(fn: () => void | Promise<void>) {
			clearTimeout(timeout);
			pendingFn = fn;
			timeout = setTimeout(() => {
				pendingFn = null;
				fn();
			}, delayMs);
		},
		cancel() {
			clearTimeout(timeout);
			pendingFn = null;
		},
		async flush() {
			clearTimeout(timeout);
			if (pendingFn) {
				const fn = pendingFn;
				pendingFn = null;
				await fn();
			}
		}
	};
}
