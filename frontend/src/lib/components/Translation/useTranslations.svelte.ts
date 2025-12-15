import { invalidateAll } from '$app/navigation';
import { apiClient } from '$lib/api/client';
import { notifications } from '$lib/notifications.svelte';
import { getLanguageName } from '$lib/config/languages';

export type TranslationStatus = 'primary' | 'draft' | 'approved';

export interface TranslationEntry {
	language: string;
	languageName: string;
	status: TranslationStatus;
	content: string;
	lastSaved?: Date;
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

export function createTranslationManager(
	getConversation: () => TranslatableConversation,
	getFormValue?: (field: string) => string | undefined
) {
	let modalOpen = $state(false);
	let activeField = $state<string | null>(null);
	let initialLanguage = $state<string | null>(null);

	function transformToDialogFormat(field: string): TranslationEntry[] {
		const conversation = getConversation();
		const fieldData = conversation.translations?.[field];
		const primaryLocale = conversation.primary_locale ?? 'en';
		const supported = conversation.supported_languages ?? [primaryLocale];

		const sortedLanguages = [
			primaryLocale,
			...supported.filter((l: string) => l !== primaryLocale)
		];

		const existingTranslations = new Map<string, NonNullable<TranslationField['text_translations']>[number]>();
		if (fieldData?.text_translations) {
			for (const tt of fieldData.text_translations) {
				existingTranslations.set(tt.locale, tt);
			}
		}

		// Check if primary content has unsaved changes (for instant UI feedback)
		const primaryExisting = existingTranslations.get(primaryLocale);
		const currentFormValue = getFormValue?.(field);
		const primaryHasUnsavedChanges = currentFormValue !== undefined && 
			currentFormValue !== (primaryExisting?.content ?? '');

		return sortedLanguages.map((locale: string) => {
			const existing = existingTranslations.get(locale);
			const isPrimary = locale === primaryLocale;
			
			// For primary language, use current form value if available
			const content = isPrimary && getFormValue 
				? (getFormValue(field) ?? existing?.content ?? '')
				: (existing?.content ?? '');
			
			// If primary has unsaved changes, show non-primary translations as draft immediately
			let status = deriveTranslationStatus(isPrimary, existing);
			if (!isPrimary && primaryHasUnsavedChanges && status === 'approved') {
				status = 'draft';
			}
			
			return {
				language: locale,
				languageName: getLanguageName(locale),
				status,
				content,
				lastSaved: existing ? new Date(existing.updated_at) : undefined
			};
		});
	}

	const activeTranslations = $derived(
		activeField ? transformToDialogFormat(activeField) : []
	);

	const activeTextContentId = $derived(
		activeField ? getConversation().translations?.[activeField]?.text_content.id : null
	);

	function getFieldTranslations(field: string): TranslationEntry[] {
		return transformToDialogFormat(field).filter(t => t.status !== 'primary');
	}

	/**
	 * Get the content for a specific field in a specific locale.
	 * Returns the translation content if it exists, otherwise returns undefined.
	 */
	function getFieldContentForLocale(field: string, locale: string): string | undefined {
		const conversation = getConversation();
		const fieldData = conversation.translations?.[field];
		if (!fieldData?.text_translations) return undefined;
		
		const translation = fieldData.text_translations.find(tt => tt.locale === locale);
		return translation?.content;
	}

	let primaryContentDebounce: ReturnType<typeof setTimeout>;

	// NOTE: This function is debounced (1s). If the user types and refreshes quickly, changes may be lost.
	function handlePrimaryContentChange(field: string) {
		clearTimeout(primaryContentDebounce);
		primaryContentDebounce = setTimeout(async () => {
			const conversation = getConversation();
			const fieldData = conversation.translations?.[field];
			const textContentId = fieldData?.text_content.id;
			const primaryLocale = conversation.primary_locale ?? 'en';
			
			if (!textContentId) return;

			// Get the current primary content from the form
			const primaryContent = getFormValue?.(field);
			if (primaryContent === undefined) return;

			try {
				// Save the primary content
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

				// Mark other approved translations as needing validation
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

	async function openDialog(field: string, language?: string) {
		activeField = field;
		initialLanguage = language ?? null;
		await invalidateAll();
		modalOpen = true;
	}

	async function closeDialog() {
		modalOpen = false;
		activeField = null;
		await invalidateAll();
	}

	let updateDebounceTimeout: ReturnType<typeof setTimeout>;

	function handleUpdate(language: string, content: string, status: TranslationStatus) {
		if (!activeTextContentId) return;
		
		const conversation = getConversation();
		const isPrimary = language === conversation.primary_locale;

		clearTimeout(updateDebounceTimeout);
		updateDebounceTimeout = setTimeout(async () => {
			try {
				await apiClient.CreateOrUpdateTextTranslation(
					{
						content,
						ai_generated: false,
						requires_validation: status !== 'approved' && status !== 'primary'
					},
					{
						params: {
							text_content_id: activeTextContentId,
							locale: language
						}
					}
				);
				await invalidateAll();
				
				// If primary content changed, mark other translations as needing validation
				if (isPrimary && activeField) {
					handlePrimaryContentChange(activeField);
				}
			} catch (e) {
				console.error('Update failed:', e);
			}
		}, 500);
	}

	async function handleLanguageToggle(
		language: string, 
		enabled: boolean,
		currentSupported: string[],
		onUpdate: (newSupported: string[]) => void
	) {
		const conversation = getConversation();
		const newSupported = enabled 
			? [...currentSupported, language]
			: currentSupported.filter(l => l !== language);
		
		onUpdate(newSupported);
		
		try {
			await apiClient.UpdateConversation(
				{ supported_languages: newSupported },
				{ params: { conversation_id: conversation.id } }
			);
			notifications.send({ 
				message: enabled ? `Added ${language}` : `Removed ${language}`, 
				priority: 'INFO' 
			});
		} catch (e) {
			notifications.send({ message: 'Failed to update languages', priority: 'ERROR' });
		}
	}

	async function handleAiTranslate(
		sourceLanguage: string, 
		targetLanguage: string
	): Promise<string> {
		try {
			const sourceTranslation = activeTranslations.find(t => t.language === sourceLanguage);
			
			if (!sourceTranslation) {
				throw new Error('Source translation not found');
			}

			const response = await fetch('/api/translate', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					source: sourceLanguage,
					target: targetLanguage,
					content: sourceTranslation.content
				})
			});
			
			if (!response.ok) {
				throw new Error('Translation failed');
			}
			
			const { translatedContent } = await response.json();
			
			notifications.send({ 
				message: 'Translation completed', 
				priority: 'INFO' 
			});
			
			return translatedContent;
		} catch (error) {
			notifications.send({ 
				message: 'AI translation failed', 
				priority: 'ERROR' 
			});
			throw error;
		}
	}

	return {
		get modalOpen() { return modalOpen; },
		set modalOpen(v: boolean) { modalOpen = v; },
		get activeField() { return activeField; },
		get activeTranslations() { return activeTranslations; },
		get initialLanguage() { return initialLanguage; },
		getFieldTranslations,
		getFieldContentForLocale,
		openDialog,
		closeDialog,
		handleUpdate,
		handleLanguageToggle,
		handleAiTranslate,
		handlePrimaryContentChange
	};
}
