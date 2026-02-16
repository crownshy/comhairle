<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Button } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';
	import RichTextEditor from '$lib/components/RichTextEditor/RichTextEditor.svelte';
	import LanguageStatusBadge from './LanguageStatusBadge.svelte';
	import TranslationEditor from './TranslationEditor.svelte';
	import type { Translation, Translation2 } from '$lib/api/api';
	import { Languages, X } from 'lucide-svelte';
	import { getLanguageName } from '$lib/config/languages';
	import { invalidateAll } from '$app/navigation';
	import { useDebounce } from 'runed';
	import {
		type TranslationStatus,
		type TranslationEntry,
		deriveStatus,
		saveTranslation,
		aiTranslate,
		markOtherTranslationsAsDraft
	} from './translationUtils';

	type TranslationData = Translation | Translation2;

	interface Props {
		value: string;
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
		translation?: TranslationData;
		initialContents?: Record<string, string>;
		initialStatuses?: Record<string, TranslationStatus>;
		onSaveSource?: (content: string) => void | Promise<void>;
		onSaveTarget?: (lang: string, content: string) => void | Promise<void>;
		onAiTranslate?: (targetLang: string, sourceContent: string) => Promise<{ content: string; requiresValidation: boolean }>;
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
		translation,
		initialContents,
		initialStatuses,
		onSaveSource,
		onSaveTarget,
		onAiTranslate: onAiTranslateProp,
		onApprove: onApproveProp,
		onMarkAsDraft: onMarkAsDraftProp
	}: Props = $props();

	let dialogOpen = $state(false);
	let clickedLang = $state<string | undefined>(undefined);
	const debouncedSaveInline = useDebounce(async (content: string) => {
		if (isTextContentMode && textContentId) {
			const id = textContentId;
			try {
				await saveTranslation(id, primaryLocale, content, {
					requiresValidation: false
				});
				const approved = badges.filter(t => t.status === 'approved' && t.content);
				if (approved.length > 0) {
					await markOtherTranslationsAsDraft(id, primaryLocale, approved);
				}
				await invalidateAll();
			} catch (e) {
				console.error('Failed to save primary content:', e);
			}
		} else if (onSaveSource) {
			onSaveSource(content);
		}
	}, 1000);
	let editorFlush: (() => Promise<void>) | null = null;

	let isTextContentMode = $derived(!!translation?.textContent?.id);
	let textContentId = $derived(translation?.textContent?.id);
	let otherLanguages = $derived(supportedLanguages.filter(l => l !== primaryLocale));

	let badges = $derived.by((): TranslationEntry[] => {
		if (otherLanguages.length === 0) return [];

		if (isTextContentMode && translation?.textTranslations) {
			return otherLanguages.map(locale => {
				const existing = translation!.textTranslations.find(t => t.locale === locale);
				return {
					language: locale,
					languageName: getLanguageName(locale),
					status: deriveStatus(false, existing?.requiresValidation),
					content: existing?.content ?? ''
				};
			});
		}

		return otherLanguages.map(locale => ({
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
		saveInlinePrimary(newValue);
	}

	function handleRichChange(content: string) {
		if (content === value) return;
		onValueChange(content);
		saveInlinePrimary(content);
	}

	function saveInlinePrimary(content: string) {
		debouncedSaveInline(content);
	}

	let editorContents = $derived.by((): Record<string, string> => {
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
			return saveTranslation(id, primaryLocale, content, { requiresValidation: false }).then(async () => {
				const entries = otherLanguages
					.map(l => ({ language: l, languageName: getLanguageName(l), status: 'draft' as TranslationStatus, content: editorContents[l] ?? '' }))
					.filter(e => e.content);
				if (entries.length > 0) await markOtherTranslationsAsDraft(id, primaryLocale, entries);
			});
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
			return saveTranslation(textContentId, lang, editorContents[lang] ?? '', { requiresValidation: false });
		}
		return onApproveProp?.(lang);
	}

	async function handleEditorMarkAsDraft(lang: string) {
		if (isTextContentMode && textContentId) {
			return saveTranslation(textContentId, lang, editorContents[lang] ?? '', { requiresValidation: true });
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
			/>
			{#if hasTranslations}
				<Button type="button" variant="link" class="absolute right-2 top-2 z-10" onclick={() => openDialog()}>
					<Languages class="h-4 w-4" />
				</Button>
			{/if}
		</div>
	{:else}
		<div class="relative">
			{#if inputType === 'textarea'}
				<Textarea class="bg-white pr-12" {value} oninput={handlePlainInput} {placeholder} {...inputProps} />
			{:else}
				<Input class="pr-12" {value} oninput={handlePlainInput} {placeholder} {...inputProps} />
			{/if}
			{#if hasTranslations}
				<Button type="button" variant="link" class="absolute right-0 top-0" onclick={() => openDialog()}>
					<Languages />
				</Button>
			{/if}
		</div>
	{/if}

	{#if hasTranslations}
		<div class="flex items-center gap-2 flex-wrap">
			{#each badges as badge (badge.language)}
				<LanguageStatusBadge {...badge} onclick={(lang) => openDialog(lang)} />
			{/each}
		</div>
	{/if}
</div>

<!-- Translation dialog -->
{#if hasTranslations}
	<Dialog.Root open={dialogOpen} onOpenChange={(open) => { if (!open) closeDialog(); }}>
		<Dialog.Content class="scot-gov max-h-[90vh] min-w-[70vw] p-12 rounded-[12px]" showCloseButton={false}>
			<Dialog.Header class="flex items-center justify-between pr-0 flex-row">
				<Dialog.Title class="justify-start text-black text-3xl font-semibold leading-8">
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
			<div class="overflow-y-auto max-h-[calc(90vh-120px)] pt-4">
				{#if dialogOpen}
					<TranslationEditor
						initialContents={editorContents}
						initialStatuses={editorStatuses}
						{primaryLocale}
						{supportedLanguages}
						{editorType}
						minHeight={dialogMinHeight}
						initialTargetLang={clickedLang}
						onSaveSource={handleEditorSaveSource}
						onSaveTarget={handleEditorSaveTarget}
						onAiTranslate={handleEditorAiTranslate}
						onApprove={handleEditorApprove}
						onMarkAsDraft={handleEditorMarkAsDraft}
						onRegisterFlush={(flush) => { editorFlush = flush; }}
					/>
				{/if}
			</div>
		</Dialog.Content>
	</Dialog.Root>
{/if}
