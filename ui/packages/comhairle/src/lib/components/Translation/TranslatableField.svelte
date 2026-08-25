<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Button } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';
	import RichTextEditor from '$lib/components/RichTextEditor/RichTextEditor.svelte';
	import LanguageStatusBadge from './LanguageStatusBadge.svelte';
	import TranslationEditor from './TranslationEditor.svelte';
	import { Languages, X, Check, LoaderCircle, TriangleAlert } from 'lucide-svelte';
	import { getLanguageName } from '$lib/config/languages';
	import type { ComponentProps } from 'svelte';
	import type { TranslationSource, TranslationEntry } from './translationUtils';
	import type { ComhairleDocument } from '@crownshy/api-client/api';
	import type { EmbeddableStep } from '$lib/components/RichTextEditor/ReportEmbedControls.svelte';
	import type { Locale } from '$lib/paraglide/runtime';

	type BaseProps = {
		/** The single persistence + read contract this field renders. See ADR-0005. */
		source: TranslationSource;
		primaryLocale: Locale;
		supportedLanguages: Locale[];
		editorType?: 'plain' | 'rich';
		placeholder?: string;
		minHeight?: string;
		maxHeight?: string;
		dialogMinHeight?: string;
		dialogTitle?: string;
		/**
		 * Optional guard run against the primary-locale value before saving. Return `false` to skip
		 * the save (e.g. a required field cleared to blank). Omit it and every change saves, which is
		 * correct for optional fields.
		 */
		canSave?: (value: string) => boolean;
		availableDocuments?: ComhairleDocument[];
		conversationId?: string;
		/** Forwarded to the inline rich editor to enable the "Embed report component" control. */
		reportEmbedSteps?: EmbeddableStep[];
	};

	// `inputProps` is typed against whichever underlying element `inputType` selects, so callers
	// get element-correct autocomplete/checking at the call site. TS can't carry that correlation
	// through the `$props()` destructure (see the cast at the spread below).
	type Props = BaseProps &
		(
			| { inputType?: 'input'; inputProps?: ComponentProps<typeof Input> }
			| { inputType?: 'textarea'; inputProps?: ComponentProps<typeof Textarea> }
		);

	let {
		source,
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
		availableDocuments = [],
		conversationId,
		reportEmbedSteps = []
	}: Props = $props();

	let dialogOpen = $state(false);
	let clickedLang = $state<Locale | undefined>(undefined);

	let value = $derived(source.contents[primaryLocale] ?? '');
	let otherLanguages = $derived(supportedLanguages.filter((l) => l !== primaryLocale));
	let hasTranslations = $derived(otherLanguages.length > 0);
	let saveState = $derived(source.saveState);

	let badges = $derived.by((): TranslationEntry[] =>
		otherLanguages.map((locale) => ({
			language: locale,
			languageName: getLanguageName(locale),
			status: source.statuses[locale] ?? 'draft',
			content: source.contents[locale] ?? ''
		}))
	);

	function saveSource(content: string) {
		if (!canSave || canSave(content)) source.saveSource(content);
	}

	function handlePlainInput(e: Event) {
		saveSource((e.currentTarget as HTMLInputElement | HTMLTextAreaElement).value);
	}

	function handleRichChange(content: string) {
		if (content === value) return;
		saveSource(content);
	}

	function openDialog(lang?: Locale) {
		clickedLang = lang;
		dialogOpen = true;
	}

	async function closeDialog() {
		if (!dialogOpen) return;
		// Commit any pending debounced edit before the dialog goes away.
		await source.flush();
		dialogOpen = false;
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
				{reportEmbedSteps}
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
					{...inputProps as ComponentProps<typeof Textarea>}
				/>
			{:else}
				<Input
					class="pr-12"
					{value}
					oninput={handlePlainInput}
					{placeholder}
					{...inputProps as ComponentProps<typeof Input>}
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

	{#if hasTranslations || saveState !== 'idle'}
		<div class="flex flex-wrap items-center gap-2">
			{#if saveState === 'saving'}
				<span class="text-muted-foreground inline-flex items-center gap-1 text-xs">
					<LoaderCircle class="h-3 w-3 animate-spin" />
					Saving
				</span>
			{:else if saveState === 'saved'}
				<span class="inline-flex items-center gap-1 text-xs text-green-600">
					<Check class="h-3 w-3" />
					Saved
				</span>
			{:else if saveState === 'error'}
				<span class="text-destructive inline-flex items-center gap-1 text-xs">
					<TriangleAlert class="h-3 w-3" />
					Not saved
				</span>
			{/if}
			{#each badges as badge (badge.language)}
				<LanguageStatusBadge
					{...badge}
					language={badge.language as Locale}
					onclick={(lang) => openDialog(lang)}
				/>
			{/each}
		</div>
	{/if}
</div>

<!-- Translation dialog -->
{#if hasTranslations}
	<Dialog.Root open={dialogOpen}>
		<Dialog.Content
			class="h-[90vh] min-w-[80vw] grid-rows-[auto_1fr] rounded-xl p-12 pt-5"
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
					class="text-muted-foreground justify-start text-xl leading-8 font-semibold"
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
			<div class="max-h-[calc(90vh-120px)] overflow-y-auto">
				<TranslationEditor
					{source}
					{primaryLocale}
					{supportedLanguages}
					{editorType}
					minHeight={dialogMinHeight}
					initialTargetLang={clickedLang}
					{availableDocuments}
					{conversationId}
				/>
			</div>
		</Dialog.Content>
	</Dialog.Root>
{/if}
