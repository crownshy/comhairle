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

export function createTranslationManager(getConversation: () => TranslatableConversation) {
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

		return sortedLanguages.map((locale: string) => {
			const existing = existingTranslations.get(locale);
			return {
				language: locale,
				languageName: getLanguageName(locale),
				status: locale === primaryLocale ? 'primary' as const :
					existing?.requires_validation ? 'draft' as const : 
					existing ? 'approved' as const : 'draft' as const,
				content: existing?.content ?? '',
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

	async function handlePrimaryContentChange(field: string) {
		const conversation = getConversation();
		const fieldData = conversation.translations?.[field];
		const textContentId = fieldData?.text_content.id;
		const primaryLocale = conversation.primary_locale ?? 'en';
		
		if (!textContentId || !fieldData?.text_translations) return;

			try {
				// Update all non-primary translations to require validation
				for (const tt of fieldData.text_translations!) {
					if (tt.locale !== primaryLocale) {
						await apiClient.CreateOrUpdateTextTranslation(
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
						);
					}
				}
				await invalidateAll();
			} catch (e) {
				console.error('Failed to mark translations as draft:', e);
			}
			
		// clearTimeout(primaryContentDebounce);
		// primaryContentDebounce = setTimeout(async () => {
		
		// }, 1000); // Debounce for 1 second to avoid too many API calls
	}

	async function openDialog(field: string, language?: string) {
		activeField = field;
		initialLanguage = language ?? null;
		await invalidateAll();
		modalOpen = true;
	}

	async function closeDialog() {
		activeField = null;
		await invalidateAll();
	}

	async function handleSave(updatedTranslations: TranslationEntry[]) {
		if (!activeTextContentId) return;

		try {
			for (const t of updatedTranslations) {
				await apiClient.CreateOrUpdateTextTranslation(
					{
						content: t.content,
						ai_generated: false,
						requires_validation: t.status === 'draft'
					},
					{
						params: {
							text_content_id: activeTextContentId,
							locale: t.language
						}
					}
				);
			}

			await invalidateAll();
			notifications.send({
				message: 'Translations saved successfully',
				priority: 'INFO'
			});
		} catch (e) {
			notifications.send({
				message: 'Failed to save translations',
				priority: 'ERROR'
			});
		}
	}

	async function handleAutoSave(language: string, content: string, status: string) {
		if (!activeTextContentId) return;

		try {
			await apiClient.CreateOrUpdateTextTranslation(
				{
					content,
					ai_generated: false,
					requires_validation: status !== 'approved'
				},
				{
					params: {
						text_content_id: activeTextContentId,
						locale: language
					}
				}
			);
		} catch (e) {
			console.error('Auto-save failed:', e);
		}
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
		handleSave,
		handleAutoSave,
		handleLanguageToggle,
		handleAiTranslate,
		handlePrimaryContentChange
	};
}
