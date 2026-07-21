<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Button } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';
	import RichTextEditor from '$lib/components/RichTextEditor/RichTextEditor.svelte';
	import LanguageStatusBadge from './LanguageStatusBadge.svelte';
	import TranslationEditor from './TranslationEditor.svelte';
	import type { Translation, Translation2 } from '@crownshy/api-client/api';
	import { Languages, X, Check, LoaderCircle, TriangleAlert } from 'lucide-svelte';
	import { getLanguageName } from '$lib/config/languages';
	import { invalidateAll } from '$app/navigation';
	import { useDebounce } from 'runed';
	import {
		type TranslationStatus,
		type TranslationEntry,
		type SaveState,
		deriveStatus,
		saveTranslation,
		aiTranslate,
		markOtherTranslationsAsDraft
	} from './translationUtils';
	import type { ComhairleDocument } from '@crownshy/api-client/api';

	type TranslationData = Translation | Translation2;

	interface Props {
		value: string | null;
		onValueChange: (value: string) => void;
		primaryLocale: string;
		supportedLanguages: string[];
		editorType?: 'plain' | 'rich';
		inputType?: 'input' | 'textarea';
		placeholder?: string;
		minHeight?: string;
		maxHeight?: string;
		dialogMinHeight?: string;
		dialogTitle?: string;
		inputProps?: Record<string, any>;
		/**
		 * Optional guard run against the primary-locale value before autosaving. Return `false`
		 * to skip the save (e.g. the value fails validation, such as a required field left blank).
		 * Omit it and every change autosaves, which is correct for optional fields.
		 */
		canSave?: (value: string) => boolean;
		/**
		 * Externally-owned save status, shown in place of the field's own indicator. Pass this in
		 * callback mode (no `translation`) where the parent owns persistence, so the "Saving/Saved"
		 * state reflects the real request rather than the field's local guess. Omit it in textContent
		 * mode; the field drives its own indicator there.
		 */
		saveStatus?: SaveState;
		translation?: TranslationData;
		initialContents?: Record<string, string>;
		initialStatuses?: Record<string, TranslationStatus>;
		availableDocuments?: ComhairleDocument[];
		conversationId?: string;
		onSaveSource?: (content: string) => void | Promise<void>;
		onSaveTarget?: (lang: string, content: string) => void | Promise<void>;
		onAiTranslate?: (
			targetLang: string,
			sourceContent: string
		) => Promise<{ content: string; requiresValidation: boolean }>;
		onApprove?: (lang: string) => void | Promise<void>;
		onMarkAsDraft?: (lang: string) => void | Promise<void>;
	}

	let {
		value,
		onValueChange,
		primaryLocale,
		supportedLanguages,
		editorType = 'plain',
		inputType = 'input',
		placeholder = '',
		minHeight = '100px',
		maxHeight,
		dialogMinHeight = '200px',
		dialogTitle = 'Content Translation',
		inputProps = {},
		canSave,
		saveStatus: externalSaveStatus,
		translation,
		initialContents,
		initialStatuses,
		availableDocuments = [],
		conversationId,
		onSaveSource,
		onSaveTarget,
		onAiTranslate: onAiTranslateProp,
		onApprove: onApproveProp,
		onMarkAsDraft: onMarkAsDraftProp
	}: Props = $props();

	let dialogOpen = $state(false);
	let clickedLang = $state<string | undefined>(undefined);
	let internalSaveStatus = $state<'idle' | 'saving' | 'saved'>('idle');
	let savedTimer: ReturnType<typeof setTimeout> | undefined;

	// An externally-supplied status wins; otherwise the field shows the one it drives itself (textContent mode).
	let displaySaveStatus = $derived(externalSaveStatus ?? internalSaveStatus);

	function setSaveStatus(status: 'idle' | 'saving' | 'saved') {
		clearTimeout(savedTimer);
		internalSaveStatus = status;
		if (status === 'saved') {
			savedTimer = setTimeout(() => (internalSaveStatus = 'idle'), 2000);
		}
	}

	// Inline autosave for textContent mode only. In callback mode the parent owns persistence and
	// receives edits through `onValueChange`, so this never runs there (see handleRichChange below).
	const debouncedSaveInline = useDebounce(async (content: string) => {
		// Never persist a value the parent has rejected (e.g. a required field cleared to blank).
		// The debounce fires with the latest content, so a value typed then cleared is dropped here.
		if (canSave && !canSave(content)) {
			setSaveStatus('idle');
			return;
		}
		if (!textContentId) return;
		const id = textContentId;
		setSaveStatus('saving');
		try {
			await saveTranslation(id, primaryLocale, content, {
				requiresValidation: false
			});
			const approved = badges.filter((t) => t.status === 'approved' && t.content);
			if (approved.length > 0) {
				await markOtherTranslationsAsDraft(id, primaryLocale, approved);
			}
			setSaveStatus('saved');

			await invalidateAll();
		} catch (e) {
			console.error('Failed to save primary content:', e);
			setSaveStatus('idle');
		}
	}, 1000);
	let editorFlush: (() => Promise<void>) | null = null;

	let isTextContentMode = $derived(!!translation?.textContent?.id);
	let textContentId = $derived(translation?.textContent?.id);
	let otherLanguages = $derived(supportedLanguages.filter((l) => l !== primaryLocale));

	let badges = $derived.by((): TranslationEntry[] => {
		if (otherLanguages.length === 0) return [];

		if (isTextContentMode && translation?.textTranslations) {
			return otherLanguages.map((locale) => {
				const existing = translation!.textTranslations.find((t) => t.locale === locale);
				return {
					language: locale,
					languageName: getLanguageName(locale),
					status: deriveStatus(false, existing?.requiresValidation),
					content: existing?.content ?? ''
				};
			});
		}

		return otherLanguages.map((locale) => ({
			language: locale,
			languageName: getLanguageName(locale),
			status: initialStatuses?.[locale] ?? ('draft' as TranslationStatus),
			content: initialContents?.[locale] ?? ''
		}));
	});

	let hasTranslations = $derived(badges.length > 0);

	function handlePlainInput(e: Event) {
		const newValue = (e.currentTarget as HTMLInputElement | HTMLTextAreaElement).value;
		onValueChange(newValue);
		if (isTextContentMode) saveInlinePrimary(newValue);
	}

	function handleRichChange(content: string) {
		if (content === value) return;
		onValueChange(content);
		if (isTextContentMode) saveInlinePrimary(content);
	}

	function saveInlinePrimary(content: string) {
		// Still reset the debounce timer on every keystroke, but don't show "Saving" for a value
		// that the guard below will drop; the debounced callback repeats the check before saving.
		if (!canSave || canSave(content)) setSaveStatus('saving');
		debouncedSaveInline(content);
	}

	let editorContents = $derived.by((): Record<string, string> => {
		if (!value) return {} as Record<string, string>;

		if (initialContents) {
			return { ...initialContents, [primaryLocale]: value };
		}
		const c: Record<string, string> = {};
		c[primaryLocale] = value;
		if (translation?.textTranslations) {
			for (const t of translation.textTranslations) {
				if (t.locale !== primaryLocale) c[t.locale] = t.content;
			}
		}
		for (const locale of otherLanguages) {
			if (!(locale in c)) c[locale] = '';
		}
		return c;
	});

	let editorStatuses = $derived.by((): Record<string, TranslationStatus> => {
		if (initialStatuses) {
			return { ...initialStatuses, [primaryLocale]: 'primary' };
		}
		const s: Record<string, TranslationStatus> = {};
		s[primaryLocale] = 'primary';
		if (translation?.textTranslations) {
			for (const t of translation.textTranslations) {
				if (t.locale !== primaryLocale) {
					s[t.locale] = deriveStatus(false, t.requiresValidation);
				}
			}
		}
		for (const locale of otherLanguages) {
			if (!(locale in s)) s[locale] = 'draft';
		}
		return s;
	});

	function handleEditorSaveSource(content: string) {
		onValueChange(content);
		if (isTextContentMode && textContentId) {
			const id = textContentId;
			return saveTranslation(id, primaryLocale, content, { requiresValidation: false }).then(
				async () => {
					const entries = otherLanguages
						.map((l) => ({
							language: l,
							languageName: getLanguageName(l),
							status: 'draft' as TranslationStatus,
							content: editorContents[l] ?? ''
						}))
						.filter((e) => e.content);
					if (entries.length > 0)
						await markOtherTranslationsAsDraft(id, primaryLocale, entries);
				}
			);
		}
		return onSaveSource?.(content);
	}

	function handleEditorSaveTarget(lang: string, content: string) {
		if (isTextContentMode && textContentId) {
			return saveTranslation(textContentId, lang, content, { requiresValidation: true });
		}
		return onSaveTarget?.(lang, content);
	}

	async function handleEditorAiTranslate(targetLang: string, sourceContent: string) {
		if (isTextContentMode && textContentId) {
			return aiTranslate(textContentId, targetLang, sourceContent, primaryLocale);
		}
		if (onAiTranslateProp) return onAiTranslateProp(targetLang, sourceContent);
		throw new Error('No AI translate handler configured');
	}

	async function handleEditorApprove(lang: string) {
		if (isTextContentMode && textContentId) {
			return saveTranslation(textContentId, lang, editorContents[lang] ?? '', {
				requiresValidation: false
			});
		}
		return onApproveProp?.(lang);
	}

	async function handleEditorMarkAsDraft(lang: string) {
		if (isTextContentMode && textContentId) {
			return saveTranslation(textContentId, lang, editorContents[lang] ?? '', {
				requiresValidation: true
			});
		}
		return onMarkAsDraftProp?.(lang);
	}

	function openDialog(lang?: string) {
		clickedLang = lang;
		dialogOpen = true;
	}

	async function closeDialog() {
		if (!dialogOpen) return;
		if (editorFlush) {
			await editorFlush();
		}
		dialogOpen = false;
		editorFlush = null;
		if (isTextContentMode) await invalidateAll();
	}
</script>

<!-- Inline field -->
<div class="flex flex-col gap-2">
	{#if editorType === 'rich'}
		<div class="relative">
			<RichTextEditor
				{value}
				onChange={handleRichChange}
				{placeholder}
				{minHeight}
				{maxHeight}
				{availableDocuments}
				{conversationId}
			/>
			{#if hasTranslations}
				<Button
					type="button"
					variant="link"
					class="absolute top-2 right-2 z-10"
					onclick={() => openDialog()}
				>
					<Languages class="h-4 w-4" />
				</Button>
			{/if}
		</div>
	{:else}
		<div class="relative">
			{#if inputType === 'textarea'}
				<Textarea
					class="pr-12"
					{value}
					oninput={handlePlainInput}
					{placeholder}
					{...inputProps}
				/>
			{:else}
				<Input
					class="pr-12"
					{value}
					oninput={handlePlainInput}
					{placeholder}
					{...inputProps}
				/>
			{/if}
			{#if hasTranslations}
				<Button
					type="button"
					variant="link"
					class="absolute top-0 right-0"
					onclick={() => openDialog()}
				>
					<Languages />
				</Button>
			{/if}
		</div>
	{/if}

	{#if hasTranslations || displaySaveStatus !== 'idle'}
		<div class="flex flex-wrap items-center gap-2">
			{#if displaySaveStatus === 'saving'}
				<span class="text-muted-foreground inline-flex items-center gap-1 text-xs">
					<LoaderCircle class="h-3 w-3 animate-spin" />
					Saving
				</span>
			{:else if displaySaveStatus === 'saved'}
				<span class="inline-flex items-center gap-1 text-xs text-green-600">
					<Check class="h-3 w-3" />
					Saved
				</span>
			{:else if displaySaveStatus === 'error'}
				<span class="text-destructive inline-flex items-center gap-1 text-xs">
					<TriangleAlert class="h-3 w-3" />
					Not saved
				</span>
			{/if}
			{#each badges as badge (badge.language)}
				<LanguageStatusBadge {...badge} onclick={(lang) => openDialog(lang)} />
			{/each}
		</div>
	{/if}
</div>

<!-- Translation dialog -->
{#if hasTranslations}
	<Dialog.Root open={dialogOpen}>
		<Dialog.Content
			class="max-h-[90vh] min-w-[70vw] rounded-xl p-12"
			showCloseButton={false}
			onInteractOutside={(e) => {
				e.preventDefault();
				closeDialog();
			}}
			onEscapeKeydown={(e) => {
				e.preventDefault();
				closeDialog();
			}}
		>
			<Dialog.Header class="flex flex-row items-center justify-between pr-0">
				<Dialog.Title
					class="text-foreground justify-start text-3xl leading-8 font-semibold"
				>
					{dialogTitle}
				</Dialog.Title>
				<button
					type="button"
					onclick={closeDialog}
					class="rounded-sm opacity-70 transition-opacity hover:opacity-100"
				>
					<X />
					<span class="sr-only">Close</span>
				</button>
			</Dialog.Header>
			<div class="max-h-[calc(90vh-120px)] overflow-y-auto pt-4">
				{#if dialogOpen}
					<TranslationEditor
						initialContents={editorContents}
						initialStatuses={editorStatuses}
						{primaryLocale}
						{supportedLanguages}
						{editorType}
						minHeight={dialogMinHeight}
						initialTargetLang={clickedLang}
						{availableDocuments}
						{conversationId}
						onSaveSource={handleEditorSaveSource}
						onSaveTarget={handleEditorSaveTarget}
						onAiTranslate={handleEditorAiTranslate}
						onApprove={handleEditorApprove}
						onMarkAsDraft={handleEditorMarkAsDraft}
						onRegisterFlush={(flush) => {
							editorFlush = flush;
						}}
					/>
				{/if}
			</div>
		</Dialog.Content>
	</Dialog.Root>
{/if}
