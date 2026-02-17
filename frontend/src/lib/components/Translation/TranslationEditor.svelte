<script lang="ts">
	import RichTextEditor from '$lib/components/RichTextEditor/RichTextEditor.svelte';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import { Sparkles, Check, MoreHorizontal } from 'lucide-svelte';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { getLanguageName } from '$lib/config/languages';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { notifications } from '$lib/notifications.svelte';
	import { useDebounce } from 'runed';
	import { type TranslationStatus, statusToBadgeVariant } from './translationUtils';

	interface Props {
		initialContents: Record<string, string>;
		initialStatuses: Record<string, TranslationStatus>;
		primaryLocale: string;
		supportedLanguages: string[];
		editorType?: 'plain' | 'rich';
		minHeight?: string;
		maxHeight?: string;
		initialTargetLang?: string;
		isLoading?: boolean;
		onSaveSource?: (content: string) => void | Promise<void>;
		onSaveTarget?: (lang: string, content: string) => void | Promise<void>;
		onAiTranslate?: (targetLang: string, sourceContent: string) => Promise<{ content: string; requiresValidation: boolean }>;
		onApprove?: (lang: string) => void | Promise<void>;
		onMarkAsDraft?: (lang: string) => void | Promise<void>;
		onRegisterFlush?: (flush: () => Promise<void>) => void;
	}

	let {
		initialContents,
		initialStatuses,
		primaryLocale,
		supportedLanguages,
		editorType = 'plain',
		minHeight = '200px',
		maxHeight,
		initialTargetLang,
		isLoading = false,
		onSaveSource,
		onSaveTarget,
		onAiTranslate,
		onApprove,
		onMarkAsDraft,
		onRegisterFlush
	}: Props = $props();

	let otherLanguages = $derived(supportedLanguages.filter((l) => l !== primaryLocale));
	let allLanguages = $derived([primaryLocale, ...otherLanguages]);

	let contents = $state<Record<string, string>>({ ...initialContents });
	let statuses = $state<Record<string, TranslationStatus>>({ ...initialStatuses });
	let activeTab = $state<string | null>(null);
	let isTranslating = $state(false);

	const debouncedSaveSource = useDebounce(async (content: string) => {
		await onSaveSource?.(content);
	}, 500);

	const debouncedSaveTarget = useDebounce(async (lang: string, content: string) => {
		await onSaveTarget?.(lang, content);
	}, 500);

	$effect(() => {
		onRegisterFlush?.(async () => {
			await debouncedSaveSource.runScheduledNow();
			await debouncedSaveTarget.runScheduledNow();
		});
	});

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
	let sourceContent = $derived(contents[primaryLocale] ?? '');
	let currentTargetContent = $derived(currentTargetLang ? (contents[currentTargetLang] ?? '') : '');
	let currentTargetStatus = $derived(
		currentTargetLang ? (statuses[currentTargetLang] ?? 'draft') : 'draft'
	);

	function handleSourceChange(content: string) {
		if (content === contents[primaryLocale]) return;
		contents[primaryLocale] = content;
		contents = { ...contents };

		for (const lang of otherLanguages) {
			if (lang in statuses && statuses[lang] !== 'primary') {
				statuses[lang] = 'draft';
			}
		}
		statuses = { ...statuses };

		debouncedSaveSource(content);
	}

	function handleTargetChange(content: string) {
		if (!currentTargetLang) return;
		if (content === contents[currentTargetLang]) return;
		contents[currentTargetLang] = content;
		contents = { ...contents };

		if (statuses[currentTargetLang] === 'approved') {
			statuses[currentTargetLang] = 'draft';
			statuses = { ...statuses };
		}

		debouncedSaveTarget(currentTargetLang, content);
	}

	function handleSourceInput(e: Event) {
		handleSourceChange((e.currentTarget as HTMLTextAreaElement).value);
	}

	function handleTargetInput(e: Event) {
		handleTargetChange((e.currentTarget as HTMLTextAreaElement).value);
	}

	async function handleAiTranslate() {
		if (isTranslating || !currentTargetLang || !sourceContent || !onAiTranslate) return;
		isTranslating = true;
		try {
			const result = await onAiTranslate(currentTargetLang, sourceContent);
			contents[currentTargetLang] = result.content;
			contents = { ...contents };
			statuses[currentTargetLang] = result.requiresValidation ? 'draft' : 'approved';
			statuses = { ...statuses };
			notifications.send({ message: 'Translation completed', priority: 'INFO' });
		} catch (e) {
			console.error('AI translation failed:', e);
			notifications.send({ message: 'AI translation failed', priority: 'ERROR' });
		} finally {
			isTranslating = false;
		}
	}

	async function handleApproveClick(lang: string) {
		const previousStatus = statuses[lang];
		statuses[lang] = 'approved';
		statuses = { ...statuses };
		try {
			await onApprove?.(lang);
		} catch (e) {
			statuses[lang] = previousStatus;
			statuses = { ...statuses };
			notifications.send({ message: 'Failed to approve', priority: 'ERROR' });
		}
	}

	async function handleMarkAsDraftClick(lang: string) {
		const previousStatus = statuses[lang];
		statuses[lang] = 'draft';
		statuses = { ...statuses };
		try {
			await onMarkAsDraft?.(lang);
		} catch (e) {
			statuses[lang] = previousStatus;
			statuses = { ...statuses };
			notifications.send({ message: 'Failed to update status', priority: 'ERROR' });
		}
	}

</script>

{#if isLoading}
	<!-- Loading skeleton -->
	<div class="flex flex-col gap-4">
		<div class="flex gap-2 border-b pb-2">
			<Skeleton class="h-10 w-28" />
			<Skeleton class="h-10 w-28" />
			<Skeleton class="h-10 w-28" />
		</div>
		<div class="flex flex-col lg:flex-row gap-6 pt-4">
			<div class="flex-1 flex flex-col gap-2">
				<Skeleton class="h-6 w-32" />
				<Skeleton class="w-full rounded-lg" style="min-height: {minHeight};" />
			</div>
			<div class="flex-1 flex flex-col gap-2">
				<Skeleton class="h-6 w-32" />
				<Skeleton class="w-full rounded-lg" style="min-height: {minHeight};" />
			</div>
		</div>
	</div>
{:else if otherLanguages.length > 0 && activeTab}
	<!-- Language tabs -->
	<div class="border-b border-base-border flex items-center overflow-x-auto">
		{#each allLanguages as lang (lang)}
			{@const isPrimary = lang === primaryLocale}
			{@const status = isPrimary ? 'primary' : (statuses[lang] ?? 'draft')}
			{@const isActive = lang === activeTab}
			<button
				type="button"
				class="py-1.5 shrink-0 transition-colors border-b-[3px] cursor-pointer {isActive ? 'border-primary' : 'border-transparent'}"
				onclick={async () => {
					await debouncedSaveSource.runScheduledNow();
					await debouncedSaveTarget.runScheduledNow();
					activeTab = lang;
				}}
			>
				<div class="px-3 py-2 rounded-lg flex items-center gap-2">
					<span class="text-lg font-semibold text-base-foreground">{getLanguageName(lang)}</span>
					{#if isPrimary}
						<Badge variant="outline" class="rounded-full shadow-sm">Primary</Badge>
					{:else}
						<Badge variant={statusToBadgeVariant[status]} class="capitalize rounded-full shadow-sm">{status}</Badge>
					{/if}
				</div>
			</button>
		{/each}
	</div>

	{#if isViewingPrimary}
		<!-- Primary editor only (full width) -->
		<div class="pt-6">
			{#if editorType === 'rich'}
				<RichTextEditor value={sourceContent} onChange={handleSourceChange} {minHeight} {maxHeight} />
			{:else}
				<div class="rounded-lg border bg-white overflow-hidden">
					<textarea
						class="w-full resize-none border-none outline-none p-4 text-sm leading-5 bg-transparent"
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
		<div class="flex flex-col xl:flex-row gap-12">
			<!-- Source (primary) column -->
			<div class="xl:w-1/2 min-w-0 flex flex-col gap-4">
				<div class="flex items-center gap-2 h-8">
					<span class="text-base font-semibold">{getLanguageName(primaryLocale)}</span>
					<Badge variant="outline" class="rounded-full shadow-sm">Primary</Badge>
				</div>
				{#if editorType === 'rich'}
					<RichTextEditor
						value={sourceContent}
						onChange={handleSourceChange}
						{minHeight}
						{maxHeight}
					/>
				{:else}
					<div class="rounded-xl border bg-white overflow-hidden">
						<textarea
							class="w-full resize-none border-none outline-none p-4 text-sm leading-5 bg-transparent"
							style="min-height: {minHeight};"
							value={sourceContent}
							oninput={handleSourceInput}
							placeholder="Primary content..."
						></textarea>
					</div>
				{/if}
			</div>

			<!-- Target column -->
			<div class="xl:w-1/2 min-w-0 flex flex-col gap-4">
				<div class="flex items-center justify-between">
					<div class="flex items-center gap-2">
						<span class="text-base font-semibold">{getLanguageName(currentTargetLang)}</span>
						<Badge
							variant={statusToBadgeVariant[currentTargetStatus]}
							class="capitalize rounded-full shadow-sm"
						>
							{currentTargetStatus}
						</Badge>
					</div>
					<Button
						type="button"
						size="sm"
						class="rounded-full gap-1.5"
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
					/>
				{:else}
					<div class="rounded-xl border-1 bg-white overflow-hidden">
						<textarea
							class="w-full resize-none border-none outline-none p-4 text-sm leading-5 bg-transparent"
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
						<Button disabled variant="outline" size="default" class="rounded-full gap-2">
							<Check class="size-4" />
							Approved
						</Button>
						<DropdownMenu.Root>
							<DropdownMenu.Trigger>
								<Button variant="outline" size="icon" class="rounded-full h-10 w-10">
									<MoreHorizontal class="size-4" />
								</Button>
							</DropdownMenu.Trigger>
							<DropdownMenu.Content>
								<DropdownMenu.Item onclick={() => handleMarkAsDraftClick(currentTargetLang)}>
									Mark as draft
								</DropdownMenu.Item>
							</DropdownMenu.Content>
						</DropdownMenu.Root>
					{:else}
						<Button
							type="button"
							size="default"
							class="rounded-full gap-2"
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
		<RichTextEditor value={sourceContent} onChange={handleSourceChange} {minHeight} {maxHeight} />
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
