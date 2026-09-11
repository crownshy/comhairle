<script lang="ts">
	import RichTextEditor from '$lib/components/RichTextEditor/RichTextEditor.svelte';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import { Sparkles, Check, MoreHorizontal } from 'lucide-svelte';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { getLanguageName } from '$lib/config/languages';
	import { notifications } from '$lib/notifications.svelte';
	import { type TranslationSource, statusToBadgeVariant } from './translationUtils';
	import type { ComhairleDocument } from '@crownshy/api-client/api';
	import type { Locale } from '$lib/paraglide/runtime';

	type Props = {
		/** The same source the inline field renders; the dialog is just another view over it. */
		source: TranslationSource;
		primaryLocale: Locale;
		supportedLanguages: Locale[];
		editorType?: 'plain' | 'rich';
		minHeight?: string;
		maxHeight?: string;
		initialTargetLang?: Locale;
		availableDocuments?: ComhairleDocument[];
		conversationId?: string;
	};

	let {
		source,
		primaryLocale,
		supportedLanguages,
		editorType = 'plain',
		minHeight = '200px',
		maxHeight,
		initialTargetLang,
		availableDocuments = [],
		conversationId
	}: Props = $props();

	let otherLanguages = $derived(supportedLanguages.filter((l) => l !== primaryLocale));
	let allLanguages = $derived([primaryLocale].concat(otherLanguages));

	// The only genuinely local state here is which tab is open and whether an AI request is in flight;
	// all content and status is read straight from the source.
	let activeTab = $state<Locale | null>(null);
	let isTranslating = $state(false);

	// Not a $derived: activeTab is user-controlled and must persist across dependency changes.
	// We only reseed when it's unset or has fallen out of the language list; a writable $derived
	// would reset the open tab whenever allLanguages/initialTargetLang changed.
	$effect(() => {
		if (allLanguages.length > 0 && (!activeTab || !allLanguages.includes(activeTab))) {
			if (initialTargetLang && otherLanguages.includes(initialTargetLang)) {
				activeTab = initialTargetLang;
			} else {
				activeTab = otherLanguages.length > 0 ? otherLanguages[0] : primaryLocale;
			}
		}
	});

	let isViewingPrimary = $derived(activeTab === primaryLocale);
	let currentTargetLang = $derived(!isViewingPrimary && activeTab ? activeTab : null);
	let sourceContent = $derived(source.contents[primaryLocale] ?? '');
	let currentTargetContent = $derived(
		currentTargetLang ? (source.contents[currentTargetLang] ?? '') : ''
	);
	let currentTargetStatus = $derived(
		currentTargetLang ? (source.statuses[currentTargetLang] ?? 'draft') : 'draft'
	);

	function handleSourceChange(content: string) {
		if (content === source.contents[primaryLocale]) return;
		source.saveSource(content);
	}

	function handleTargetChange(content: string) {
		if (!currentTargetLang) return;
		if (content === source.contents[currentTargetLang]) return;
		source.saveTarget(currentTargetLang, content);
	}

	function handleSourceInput(e: Event) {
		handleSourceChange((e.currentTarget as HTMLTextAreaElement).value);
	}

	function handleTargetInput(e: Event) {
		handleTargetChange((e.currentTarget as HTMLTextAreaElement).value);
	}

	async function selectTab(lang: Locale) {
		// Commit pending edits before leaving the current tab so nothing is lost on switch.
		await source.flush();
		activeTab = lang;
	}

	async function handleAiTranslate() {
		if (isTranslating || !currentTargetLang || !sourceContent) return;
		isTranslating = true;
		try {
			await source.aiTranslate(currentTargetLang, sourceContent);
			notifications.send({ message: 'Translation completed', priority: 'INFO' });
		} catch (e) {
			console.error('AI translation failed:', e);
			notifications.send({ message: 'AI translation failed', priority: 'ERROR' });
		} finally {
			isTranslating = false;
		}
	}

	async function handleApproveClick(lang: string) {
		try {
			await source.approve(lang);
		} catch {
			notifications.send({ message: 'Failed to approve', priority: 'ERROR' });
		}
	}

	async function handleMarkAsDraftClick(lang: string) {
		try {
			await source.markAsDraft(lang);
		} catch {
			notifications.send({ message: 'Failed to update status', priority: 'ERROR' });
		}
	}
</script>

{#if otherLanguages.length > 0 && activeTab}
	<!-- Language tabs -->
	<div class="border-base-border flex items-center overflow-x-auto border-b">
		{#each allLanguages as lang (lang)}
			{@const isPrimary = lang === primaryLocale}
			{@const status = isPrimary ? 'primary' : (source.statuses[lang] ?? 'draft')}
			{@const isActive = lang === activeTab}
			<button
				type="button"
				class="shrink-0 cursor-pointer border-b-[3px] py-1.5 transition-colors {isActive
					? 'border-primary'
					: 'border-transparent'}"
				onclick={() => selectTab(lang)}
			>
				<div class="flex items-center gap-2 rounded-lg px-3 py-2">
					<span class="text-base-foreground text-lg font-semibold"
						>{getLanguageName(lang)}</span
					>
					{#if isPrimary}
						<Badge variant="outline" class="rounded-full shadow-sm">Primary</Badge>
					{:else}
						<Badge
							variant={statusToBadgeVariant[status]}
							class="rounded-full capitalize shadow-sm">{status}</Badge
						>
					{/if}
				</div>
			</button>
		{/each}
	</div>

	{#if isViewingPrimary}
		<!-- Primary editor only (full width) -->
		<div class="pt-6">
			{#if editorType === 'rich'}
				<RichTextEditor
					value={sourceContent}
					onChange={handleSourceChange}
					{minHeight}
					{maxHeight}
					{availableDocuments}
					{conversationId}
				/>
			{:else}
				<div class="dark:bg-input/30 overflow-hidden rounded-lg border bg-white">
					<textarea
						class="w-full resize-none border-none bg-transparent p-4 text-sm leading-5 outline-none"
						style="min-height: {minHeight};"
						value={sourceContent}
						oninput={handleSourceInput}
						placeholder="Primary content..."
					></textarea>
				</div>
			{/if}
		</div>
	{:else if currentTargetLang}
		<!-- Target language heading -->
		<div class="pt-6 pb-2">
			<span class="text-2xl font-semibold">{getLanguageName(currentTargetLang)}</span>
		</div>

		<!-- Side-by-side editors -->
		<div class="flex flex-col gap-12 xl:flex-row">
			<!-- Source (primary) column -->
			<div class="flex min-w-0 flex-col gap-4 xl:w-1/2">
				<div class="flex h-8 items-center gap-2">
					<span class="text-base font-semibold">{getLanguageName(primaryLocale)}</span>
					<Badge variant="outline" class="rounded-full shadow-sm">Primary</Badge>
				</div>
				{#if editorType === 'rich'}
					<RichTextEditor
						value={sourceContent}
						onChange={handleSourceChange}
						{minHeight}
						{maxHeight}
						{availableDocuments}
						{conversationId}
					/>
				{:else}
					<div class="dark:bg-input/30 overflow-hidden rounded-xl border bg-white">
						<textarea
							class="w-full resize-none border-none bg-transparent p-4 text-sm leading-5 outline-none"
							style="min-height: {minHeight};"
							value={sourceContent}
							oninput={handleSourceInput}
							placeholder="Primary content..."
						></textarea>
					</div>
				{/if}
			</div>

			<!-- Target column -->
			<div class="flex min-w-0 flex-col gap-4 xl:w-1/2">
				<div class="flex items-center justify-between">
					<div class="flex items-center gap-2">
						<span class="text-base font-semibold"
							>{getLanguageName(currentTargetLang)}</span
						>
						<Badge
							variant={statusToBadgeVariant[currentTargetStatus]}
							class="rounded-full capitalize shadow-sm"
						>
							{currentTargetStatus}
						</Badge>
					</div>
					<Button
						type="button"
						size="sm"
						class="gap-1.5 rounded-full"
						onclick={handleAiTranslate}
						disabled={isTranslating}
					>
						{#if isTranslating}
							Translating...
						{:else}
							AI translation
							<Sparkles class="h-4 w-4" />
						{/if}
					</Button>
				</div>
				{#if editorType === 'rich'}
					<RichTextEditor
						value={currentTargetContent}
						onChange={handleTargetChange}
						{minHeight}
						{maxHeight}
						{availableDocuments}
						{conversationId}
					/>
				{:else}
					<div class="dark:bg-input/30 overflow-hidden rounded-xl border bg-white">
						<textarea
							class="w-full resize-none border-none bg-transparent p-4 text-sm leading-5 outline-none"
							style="min-height: {minHeight};"
							value={currentTargetContent}
							oninput={handleTargetInput}
							placeholder="Translation content..."
						></textarea>
					</div>
				{/if}

				<!-- Approve / Mark as draft -->
				<div class="flex items-center justify-center gap-3">
					{#if currentTargetStatus === 'approved'}
						<Button
							disabled
							variant="outline"
							size="default"
							class="gap-2 rounded-full"
						>
							<Check class="size-4" />
							Approved
						</Button>
						<DropdownMenu.Root>
							<DropdownMenu.Trigger>
								<Button
									variant="outline"
									size="icon"
									class="h-10 w-10 rounded-full"
								>
									<MoreHorizontal class="size-4" />
								</Button>
							</DropdownMenu.Trigger>
							<DropdownMenu.Content>
								<DropdownMenu.Item
									onclick={() => handleMarkAsDraftClick(currentTargetLang)}
								>
									Mark as draft
								</DropdownMenu.Item>
							</DropdownMenu.Content>
						</DropdownMenu.Root>
					{:else}
						<Button
							type="button"
							size="default"
							class="gap-2 rounded-full"
							onclick={() => handleApproveClick(currentTargetLang)}
							disabled={!currentTargetContent}
						>
							<Check class="size-4" />
							Approve
						</Button>
					{/if}
				</div>
			</div>
		</div>
	{/if}
{:else}
	<!-- Single editor when no translations needed -->
	{#if editorType === 'rich'}
		<RichTextEditor
			value={sourceContent}
			onChange={handleSourceChange}
			{minHeight}
			{maxHeight}
			{availableDocuments}
			{conversationId}
		/>
	{:else}
		<textarea
			class="w-full resize-none rounded-lg border p-3 text-sm"
			style="min-height: {minHeight};"
			value={sourceContent}
			oninput={handleSourceInput}
			placeholder="Content..."
		></textarea>
	{/if}
{/if}
