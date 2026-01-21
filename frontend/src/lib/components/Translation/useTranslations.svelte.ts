import { invalidateAll } from '$app/navigation';
import { apiClient } from '$lib/api/client';
import { notifications } from '$lib/notifications.svelte';
import { getLanguageName } from '$lib/config/languages';

export type TranslationStatus = 'primary' | 'draft' | 'approved';

export const statusToBadgeVariant = {
	primary: 'outline',
	draft: 'secondary',
	approved: 'default'
} as const;

export interface TranslationEntry {
	language: string;
	languageName: string;
	status: TranslationStatus;
	content: string;
}

export interface TranslationField {
	text_content: { id: string };
	text_translations?: Array<{
		locale: string;
		content: string;
		requires_validation: boolean;
		updated_at: string;
	}>;
}

export interface TranslatableConversation {
	id: string;
	primary_locale?: string;
	supported_languages?: string[];
	translations?: Record<string, TranslationField>;
}

function deriveTranslationStatus(
	isPrimary: boolean,
	existing?: { requires_validation: boolean }
): TranslationStatus {
	if (isPrimary) return 'primary';
	if (!existing) return 'draft';
	return existing.requires_validation ? 'draft' : 'approved';
}

/**
 * Creates a translation manager with parent-managed state.
 * The dialog receives a working copy that the parent mutates.
 * API calls are debounced and happen in the background.
 */
export function createTranslationManager(
	getConversation: () => TranslatableConversation,
	getFormValue?: (field: string) => string | undefined
) {
	let modalOpen = $state(false);
	let activeField = $state<string | null>(null);
	let activeLanguage = $state<string | null>(null);
	let isTranslating = $state(false);
	
	let workingTranslations = $state<TranslationEntry[]>([]);

	// Debounce timer for API saves
	let saveDebounceTimeout: ReturnType<typeof setTimeout>;

	function buildTranslationsFromServer(field: string): TranslationEntry[] {
		const conversation = getConversation();
		const fieldData = conversation.translations?.[field];
		const primaryLocale = conversation.primary_locale ?? 'en';
		const supported = conversation.supported_languages ?? [primaryLocale];

		const sortedLanguages = [
			primaryLocale,
			...supported.filter((l: string) => l !== primaryLocale)
		];

		const existingMap = new Map<string, NonNullable<TranslationField['text_translations']>[number]>();
		if (fieldData?.text_translations) {
			for (const tt of fieldData.text_translations) {
				existingMap.set(tt.locale, tt);
			}
		}

		return sortedLanguages.map((locale: string) => {
			const existing = existingMap.get(locale);
			const isPrimary = locale === primaryLocale;
			
			const fallback = existing?.content ?? '';
			const content = isPrimary && getFormValue 
				? (getFormValue(field) ?? fallback)
				: fallback;
			
			return {
				language: locale,
				languageName: getLanguageName(locale),
				status: deriveTranslationStatus(isPrimary, existing),
				content
			};
		});
	}

	function getTextContentId(): string | null {
		if (!activeField) return null;
		return getConversation().translations?.[activeField]?.text_content.id ?? null;
	}

	function getPrimaryLocale(): string {
		return getConversation().primary_locale ?? 'en';
	}

	// --- Dialog Actions ---

	function openDialog(field: string, language?: string) {
		activeField = field;
		workingTranslations = buildTranslationsFromServer(field);
		
		// Set initial active language (first non-primary, or provided language)
		if (language && workingTranslations.some(t => t.language === language)) {
			activeLanguage = language;
		} else {
			const nonPrimary = workingTranslations.find(t => t.status !== 'primary');
			activeLanguage = nonPrimary?.language ?? null;
		}
		
		modalOpen = true;
	}

	function closeDialog() {
		modalOpen = false;
		activeField = null;
		activeLanguage = null;
		workingTranslations = [];
	}

	function setActiveLanguage(language: string) {
		const primaryLang = workingTranslations.find(t => t.status === 'primary')?.language;
		if (language !== primaryLang) {
			activeLanguage = language;
		}
	}

	// --- Content & Status Updates ---

	function updateContent(language: string, content: string) {
		const idx = workingTranslations.findIndex(t => t.language === language);
		if (idx === -1) return;

		const translation = workingTranslations[idx];
		const isPrimary = translation.status === 'primary';
		
		// Update local working copy immediately
		workingTranslations[idx] = {
			...translation,
			content,
			status: isPrimary ? 'primary' : 'draft'
		};

		// If primary changed, mark all non-primary as draft
		if (isPrimary) {
			workingTranslations = workingTranslations.map(t => 
				t.status === 'primary' ? t : { ...t, status: 'draft' as TranslationStatus }
			);
		}

		// Debounced save to API
		debouncedSave(language, content, isPrimary ? 'primary' : 'draft');
	}

	function updateStatus(language: string, status: TranslationStatus) {
		const idx = workingTranslations.findIndex(t => t.language === language);
		if (idx === -1) return;

		const translation = workingTranslations[idx];
		if (translation.status === 'primary') return; // Can't change primary status

		// Update local working copy immediately
		workingTranslations[idx] = { ...translation, status };

		// Save to API immediately (no debounce for status changes)
		saveToApi(language, translation.content, status);
	}

	function debouncedSave(language: string, content: string, status: TranslationStatus) {
		clearTimeout(saveDebounceTimeout);
		saveDebounceTimeout = setTimeout(() => {
			saveToApi(language, content, status);
		}, 500);
	}

	async function saveToApi(language: string, content: string, status: TranslationStatus) {
		const textContentId = getTextContentId();
		if (!textContentId) return;

		try {
			await apiClient.CreateOrUpdateTextTranslation(
				{
					content,
					ai_generated: false,
					requires_validation: status === 'draft'
				},
				{
					params: {
						text_content_id: textContentId,
						locale: language
					}
				}
			);

			// If primary changed, mark other translations as needing validation
			if (language === getPrimaryLocale()) {
				await markOtherTranslationsAsDraft(textContentId, language);
			}

			// Refresh server data so parent UI updates
			await invalidateAll();
		} catch (e) {
			console.error('Failed to save translation:', e);
			notifications.send({ message: 'Failed to save translation', priority: 'ERROR' });
		}
	}

	async function markOtherTranslationsAsDraft(textContentId: string, primaryLocale: string) {
		const otherTranslations = workingTranslations.filter(
			t => t.language !== primaryLocale && t.content
		);

		await Promise.all(
			otherTranslations.map(t => 
				apiClient.CreateOrUpdateTextTranslation(
					{
						content: t.content,
						ai_generated: false,
						requires_validation: true
					},
					{
						params: {
							text_content_id: textContentId,
							locale: t.language
						}
					}
				)
			)
		);
	}

	async function handleAiTranslate() {
		if (!activeLanguage || isTranslating) return;

		const textContentId = getTextContentId();
		if (!textContentId) return;

		const primaryTranslation = workingTranslations.find(t => t.status === 'primary');
		if (!primaryTranslation?.content) return;

		isTranslating = true;
		try {
			const activeTranslation = workingTranslations.find(t => t.language === activeLanguage);
			await apiClient.CreateOrUpdateTextTranslation(
				{
					content: activeTranslation?.content ?? '',
					ai_generated: true,
					requires_validation: true
				},
				{
					params: {
						text_content_id: textContentId,
						locale: activeLanguage
					}
				}
			);

			const response = await apiClient.AutomaticallyGenerateTranslation(undefined, {
				params: {
					text_content_id: textContentId,
					locale: activeLanguage
				}
			});
			
			// Update the working copy with translated content
			const idx = workingTranslations.findIndex(t => t.language === activeLanguage);
			if (idx !== -1) {
				workingTranslations[idx] = {
					...workingTranslations[idx],
					content: response.content,
					status: response.requires_validation ? 'draft' : 'approved'
				};
			}

			await invalidateAll();
			
			notifications.send({ message: 'Translation completed', priority: 'INFO' });
		} catch (error) {
			console.error('AI translation failed:', error);
			notifications.send({ message: 'AI translation failed', priority: 'ERROR' });
		} finally {
			isTranslating = false;
		}
	}

	// --- Utilities for parent component ---

	function getFieldTranslations(field: string): TranslationEntry[] {
		return buildTranslationsFromServer(field).filter(t => t.status !== 'primary');
	}

	function getFieldContentForLocale(field: string, locale: string): string | undefined {
		const conversation = getConversation();
		const fieldData = conversation.translations?.[field];
		if (!fieldData?.text_translations) return undefined;
		
		const translation = fieldData.text_translations.find(tt => tt.locale === locale);
		return translation?.content;
	}

	let primaryContentDebounce: ReturnType<typeof setTimeout>;

	function handlePrimaryContentChange(field: string) {
		clearTimeout(primaryContentDebounce);
		primaryContentDebounce = setTimeout(async () => {
			const conversation = getConversation();
			const fieldData = conversation.translations?.[field];
			const textContentId = fieldData?.text_content.id;
			const primaryLocale = conversation.primary_locale ?? 'en';
			
			if (!textContentId) return;

			const primaryContent = getFormValue?.(field);
			if (primaryContent === undefined) return;

			try {
				await apiClient.CreateOrUpdateTextTranslation(
					{
						content: primaryContent,
						ai_generated: false,
						requires_validation: false
					},
					{
						params: {
							text_content_id: textContentId,
							locale: primaryLocale
						}
					}
				);

				if (fieldData?.text_translations) {
					const translationsToUpdate = fieldData.text_translations.filter(
						tt => tt.locale !== primaryLocale && !tt.requires_validation
					);
					
					if (translationsToUpdate.length > 0) {
						await Promise.all(
							translationsToUpdate.map(tt => apiClient.CreateOrUpdateTextTranslation(
								{
									content: tt.content,
									ai_generated: false,
									requires_validation: true
								},
								{
									params: {
										text_content_id: textContentId,
										locale: tt.locale
									}
								}
							))
						);
					}
				}

				await invalidateAll();
			} catch (e) {
				console.error('Failed to save primary content:', e);
			}
		}, 1000);
	}

	// --- Auto-translate for new language ---

	async function autoTranslateNewLanguage(locale: string, textContentIds: string[]) {
		const results: { field: string; success: boolean }[] = [];
		
		for (const textContentId of textContentIds) {
			try {
				// First create an empty translation record (required by backend)
				await apiClient.CreateOrUpdateTextTranslation(
					{
						content: '',
						ai_generated: true,
						requires_validation: true
					},
					{
						params: {
							text_content_id: textContentId,
							locale
						}
					}
				);

				// Then auto-translate it
				await apiClient.AutomaticallyGenerateTranslation(undefined, {
					params: {
						text_content_id: textContentId,
						locale
					}
				});
				results.push({ field: textContentId, success: true });
			} catch (error) {
				console.error(`Failed to auto-translate ${textContentId} to ${locale}:`, error);
				results.push({ field: textContentId, success: false });
			}
		}

		await invalidateAll();
		return results;
	}

	return {
		// Dialog state (reactive)
		get modalOpen() { return modalOpen; },
		set modalOpen(v: boolean) { modalOpen = v; },
		get activeField() { return activeField; },
		get activeLanguage() { return activeLanguage; },
		get isTranslating() { return isTranslating; },
		get workingTranslations() { return workingTranslations; },

		// Dialog actions
		openDialog,
		closeDialog,
		setActiveLanguage,
		updateContent,
		updateStatus,
		handleAiTranslate,

		// Utilities for form fields
		getFieldTranslations,
		getFieldContentForLocale,
		handlePrimaryContentChange,
		autoTranslateNewLanguage
	};
}
