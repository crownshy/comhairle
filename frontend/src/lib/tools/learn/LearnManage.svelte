<script lang="ts">
	import type { LocalisedPage, WorkflowStepWithTranslations, ConversationWithTranslations } from '$lib/api/api';

	interface ExtendedLocalisedPage extends LocalisedPage {
		lang: string;
		requires_validation: boolean;
	}

	type Props = {
		conversation_id: string;
		conversation: ConversationWithTranslations;
		workflow_step: WorkflowStepWithTranslations;
		isLive: boolean;
	};

	import { apiClient } from '$lib/api/client';
	import { invalidateAll } from '$app/navigation';
	import { notifications } from '$lib/notifications.svelte';
	import * as Select from '$lib/components/ui/select';
	import { Button } from '$lib/components/ui/button';
	import TranslatableField from '$lib/components/Translation/TranslatableField.svelte';
	import { aiTranslateContent, type TranslationStatus } from '$lib/components/Translation/translationUtils';
	import { Skeleton } from '$lib/components/ui/skeleton';

	let { conversation_id, conversation, workflow_step, isLive }: Props = $props();
	
	let isInitialLoad = $state(true);

	let primaryLocale = $derived(conversation.primaryLocale ?? 'en');
	let supportedLanguages = $derived(conversation.supportedLanguages ?? ['en']);

	type LearnToolConfig = { type: 'learn'; pages: ExtendedLocalisedPage[][] };
		
	let sourceConfig = $derived(
		(isLive ? workflow_step.toolConfig : workflow_step.previewToolConfig) as LearnToolConfig
	);

	let pages = $state<ExtendedLocalisedPage[][]>([]);
	let hasLocalChanges = $state(false);
	
	let lastPropsConfig = $state<string>('');
	$effect(() => {
		const propsConfig = JSON.stringify(sourceConfig?.pages);
		if (propsConfig !== lastPropsConfig && !hasLocalChanges) {
			pages = structuredClone(sourceConfig?.pages ?? []);
			lastPropsConfig = propsConfig;
			if (isInitialLoad && pages.length > 0) {
				isInitialLoad = false;
			}
		}
	});

	function getToolConfigForSave(): LearnToolConfig {
		return { type: 'learn', pages };
	}
	
	function markLocalChanges() {
		hasLocalChanges = true;
	}
	
	function clearLocalChanges() {
		hasLocalChanges = false;
		lastPropsConfig = JSON.stringify(sourceConfig?.pages);
	}

	let currentPageIndex = $state(0);

	function getTranslation(lang: string): ExtendedLocalisedPage | undefined {
		return pages[currentPageIndex]?.find((p) => p.lang === lang);
	}

	let sourceContent = $derived.by(() => {
		const source = getTranslation(primaryLocale);
		return source?.content ?? '';
	});

	let targetLanguages = $derived(
		supportedLanguages.filter((lang: string) => lang !== primaryLocale)
	);

	let pageContents = $derived.by((): Record<string, string> => {
		const c: Record<string, string> = {};
		c[primaryLocale] = sourceContent;
		for (const lang of targetLanguages) {
			const t = getTranslation(lang);
			c[lang] = t?.content ?? '';
		}
		return c;
	});

	let pageStatuses = $derived.by((): Record<string, TranslationStatus> => {
		const s: Record<string, TranslationStatus> = {};
		s[primaryLocale] = 'primary';
		for (const lang of targetLanguages) {
			const t = getTranslation(lang);
			s[lang] = t && t.requires_validation === false ? 'approved' : 'draft';
		}
		return s;
	});

	function deletePage() {
		markLocalChanges();
		pages = pages.filter((_: ExtendedLocalisedPage[], i: number) => i !== currentPageIndex);
		currentPageIndex = Math.max(currentPageIndex - 1, 0);
		saveToServer();
	}

	function addPage() {
		markLocalChanges();
		const newPage: ExtendedLocalisedPage[] = [
			{
				lang: primaryLocale,
				content: `# Page ${pages.length + 1}`,
				type: 'markdown',
				requires_validation: false
			}
		];
		pages.push(newPage);
		currentPageIndex = pages.length - 1;
		saveToServer();
	}

	async function saveToServer({ invalidate = true }: { invalidate?: boolean } = {}) {
		try {
			const configToSave = getToolConfigForSave();
			await apiClient.UpdateWorkflowStep(
				isLive
					? { tool_config: configToSave }
					: { preview_tool_config: configToSave },
				{
					params: {
						workflow_id: workflow_step.workflowId,
						conversation_id,
						workflow_step_id: workflow_step.id
					}
				}
			);
			if (invalidate) await invalidateAll();
			clearLocalChanges();
		} catch (e) {
			notifications.send({ message: 'Failed to save changes', priority: 'ERROR' });
		}
	}

	function handleSaveSource(content: string) {
		markLocalChanges();
		const source = getTranslation(primaryLocale);
		if (source) {
			source.content = content;
			source.type = 'markdown';
		} else if (pages[currentPageIndex]) {
			pages[currentPageIndex].push({
				lang: primaryLocale, type: 'markdown', content, requires_validation: false
			});
		}
		for (const t of pages[currentPageIndex] ?? []) {
			if (t.lang !== primaryLocale) t.requires_validation = true;
		}
		pages = [...pages];
		saveToServer({ invalidate: false });
	}

	function handleSaveTarget(lang: string, content: string) {
		markLocalChanges();
		const target = getTranslation(lang);
		if (target) {
			target.content = content;
			target.type = 'markdown';
			target.requires_validation = true;
		} else if (pages[currentPageIndex]) {
			pages[currentPageIndex].push({
				lang, type: 'markdown', content, requires_validation: true
			});
		}
		pages = [...pages];
		saveToServer({ invalidate: false });
	}

	async function handleAiTranslate(targetLang: string, sContent: string): Promise<{ content: string; requiresValidation: boolean }> {
		const translatedContent = await aiTranslateContent(sContent, targetLang, primaryLocale);
		let t = getTranslation(targetLang);
		if (t) {
			t.content = translatedContent;
			t.requires_validation = true;
		} else {
			pages[currentPageIndex].push({
				lang: targetLang, type: 'markdown', content: translatedContent, requires_validation: true
			});
		}
		pages = [...pages];
		await saveToServer({ invalidate: false });
		return { content: translatedContent, requiresValidation: true };
	}

	async function handleApprove(lang: string) {
		const t = getTranslation(lang);
		if (!t) return;
		markLocalChanges();
		t.requires_validation = false;
		pages = [...pages];
		await saveToServer({ invalidate: false });
	}

	async function handleMarkAsDraft(lang: string) {
		const t = getTranslation(lang);
		if (!t) return;
		markLocalChanges();
		t.requires_validation = true;
		pages = [...pages];
		await saveToServer({ invalidate: false });
	}
</script>

<!-- Controls -->
<div class="flex flex-col gap-4">
	<!-- Top row: Page controls -->
	<div class="flex items-center justify-between gap-4">
		<div class="flex items-center gap-2">
			{#if isInitialLoad}
				<Skeleton class="h-10 w-[180px]" />
				<Skeleton class="h-10 w-24" />
				<Skeleton class="h-10 w-28" />
			{:else}
				<Select.Root
					type="single"
					value={currentPageIndex.toString()}
					onValueChange={(value: string) => (currentPageIndex = parseInt(value))}
				>
					<Select.Trigger class="w-[180px] bg-white"
						>Page {currentPageIndex + 1}</Select.Trigger
					>
					<Select.Content>
						{#each pages as _, i}
							<Select.Item value={i.toString()}>Page {i + 1}</Select.Item>
						{/each}
					</Select.Content>
				</Select.Root>

				<Button onclick={addPage}>+ Add Page</Button>
				<Button variant="destructive" onclick={deletePage} disabled={pages.length <= 1}>- Delete Page</Button>
			{/if}
		</div>
	</div>

	<!-- Primary content editor + translation badges + dialog -->
	{#if isInitialLoad}
		<div class="flex flex-col gap-2">
			<div class="border rounded-lg overflow-hidden">
				<div class="flex items-center gap-1 p-2 border-b bg-muted/30">
					<Skeleton class="h-8 w-8 rounded" />
					<Skeleton class="h-8 w-8 rounded" />
					<Skeleton class="h-8 w-8 rounded" />
				</div>
				<div class="p-4" style="min-height: 300px;">
					<Skeleton class="h-4 w-3/4 mb-3" />
					<Skeleton class="h-4 w-full mb-3" />
					<Skeleton class="h-4 w-5/6 mb-3" />
					<Skeleton class="h-4 w-2/3" />
				</div>
			</div>
			<div class="flex gap-2">
				<Skeleton class="h-7 w-24 rounded-full" />
				<Skeleton class="h-7 w-24 rounded-full" />
			</div>
		</div>
	{:else}
		<TranslatableField
			value={sourceContent}
			onValueChange={handleSaveSource}
			{primaryLocale}
			{supportedLanguages}
			editorType="rich"
			minHeight="300px"
			dialogMinHeight="250px"
			dialogTitle="Translate: Page {currentPageIndex + 1}"
			initialContents={pageContents}
			initialStatuses={pageStatuses}
			onSaveSource={handleSaveSource}
			onSaveTarget={handleSaveTarget}
			onAiTranslate={handleAiTranslate}
			onApprove={handleApprove}
			onMarkAsDraft={handleMarkAsDraft}
		/>
	{/if}
</div>
