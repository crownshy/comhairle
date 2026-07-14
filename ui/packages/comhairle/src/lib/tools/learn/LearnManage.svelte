<script lang="ts">
	import type {
		LocalizedPage,
		WorkflowStepWithTranslations,
		ConversationWithTranslations,
		ComhairleDocument
	} from '@crownshy/api-client/api';

	interface ExtendedLocalizedPage extends LocalizedPage {
		lang: string;
		requires_validation: boolean;
	}

	type Props = {
		conversationId: string;
		conversation: ConversationWithTranslations;
		workflowStep: WorkflowStepWithTranslations;
		isLive: boolean;
	};

	import { apiClient } from '@crownshy/api-client/client';
	import { invalidateAll } from '$app/navigation';
	import { notifications } from '$lib/notifications.svelte';
	import * as Select from '$lib/components/ui/select';
	import { Button } from '$lib/components/ui/button';
	import TranslatableField from '$lib/components/Translation/TranslatableField.svelte';
	import {
		aiTranslateContent,
		type TranslationStatus
	} from '$lib/components/Translation/translationUtils';
	import { Skeleton } from '$lib/components/ui/skeleton';

	let { conversationId, conversation, workflowStep, isLive }: Props = $props();

	let isInitialLoad = $state(true);

	let primaryLocale = $derived(conversation.primaryLocale ?? 'en');
	let supportedLanguages = $derived(conversation.supportedLanguages ?? ['en']);

	type LearnToolConfig = { type: 'learn'; pages: ExtendedLocalizedPage[][] };

	let sourceConfig = $derived(
		(isLive ? workflowStep.toolConfig : workflowStep.previewToolConfig) as LearnToolConfig
	);

	let pages = $state<ExtendedLocalizedPage[][]>([]);
	let hasLocalChanges = $state(false);

	let lastPropsConfig = $state<string>('');
	$effect(() => {
		const propsConfig = JSON.stringify({
			pages: sourceConfig?.pages
		});
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
		lastPropsConfig = JSON.stringify({
			pages: sourceConfig?.pages
		});
	}

	let currentPageIndex = $state(0);

	function getTranslation(lang: string): ExtendedLocalizedPage | undefined {
		return pages[currentPageIndex]?.find((p) => p.lang === lang);
	}

	/**
	 * Insert or update the `lang` translation on the current page.
	 *
	 * Finds the existing entry and mutates it in place, or appends a new one if
	 * none exists yet. `pages` is `$state`, so the in-place mutation is reactive on
	 * its own; no reassignment is needed. Does nothing if the current page is absent.
	 *
	 * @param lang - locale of the translation to upsert
	 * @param fields - `content` to set, and `requiresValidation` for the entry. On
	 *   update, an omitted field is left untouched; on insert it defaults (empty
	 *   content, `requires_validation: false`).
	 */
	function upsertTranslation(
		lang: string,
		fields: { content?: string; requiresValidation?: boolean }
	) {
		const page = pages[currentPageIndex];
		if (!page) return;
		const existing = getTranslation(lang);
		if (existing) {
			if (fields.content !== undefined) existing.content = fields.content;
			existing.type = 'markdown';
			if (fields.requiresValidation !== undefined)
				existing.requires_validation = fields.requiresValidation;
		} else {
			page.push({
				lang,
				type: 'markdown',
				content: fields.content ?? '',
				requires_validation: fields.requiresValidation ?? false
			});
		}
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
		pages = pages.filter((_: ExtendedLocalizedPage[], i: number) => i !== currentPageIndex);
		currentPageIndex = Math.max(currentPageIndex - 1, 0);
		saveToServer();
	}

	function addPage() {
		markLocalChanges();
		const newPage: ExtendedLocalizedPage[] = [
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
			await apiClient.UpdateConversationWorkflowStep(
				isLive ? { tool_config: configToSave } : { preview_tool_config: configToSave },
				{
					params: {
						workflow_id: workflowStep.workflowId,
						conversation_id: conversationId,
						workflow_step_id: workflowStep.id
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
		upsertTranslation(primaryLocale, { content });
		// Editing the source text invalidates every existing translation of this page.
		for (const t of pages[currentPageIndex] ?? []) {
			if (t.lang !== primaryLocale) t.requires_validation = true;
		}
		saveToServer({ invalidate: false });
	}

	function handleSaveTarget(lang: string, content: string) {
		markLocalChanges();
		upsertTranslation(lang, { content, requiresValidation: true });
		saveToServer({ invalidate: false });
	}

	async function handleAiTranslate(
		targetLang: string,
		sContent: string
	): Promise<{ content: string; requiresValidation: boolean }> {
		const translatedContent = await aiTranslateContent(sContent, targetLang, primaryLocale);
		upsertTranslation(targetLang, { content: translatedContent, requiresValidation: true });
		await saveToServer({ invalidate: false });
		return { content: translatedContent, requiresValidation: true };
	}

	async function handleApprove(lang: string) {
		const t = getTranslation(lang);
		if (!t) return;
		markLocalChanges();
		t.requires_validation = false;
		await saveToServer({ invalidate: false });
	}

	async function handleMarkAsDraft(lang: string) {
		const t = getTranslation(lang);
		if (!t) return;
		markLocalChanges();
		t.requires_validation = true;
		await saveToServer({ invalidate: false });
	}

	// --- Document list for inline source document picker ---
	let availableDocuments = $state<ComhairleDocument[]>([]);

	$effect(() => {
		if (!conversationId) return;
		apiClient
			.ListDocuments({ params: { conversation_id: conversationId } })
			.then((docs) => {
				availableDocuments = docs.filter((d) => d.parse_status === 'DONE');
			})
			.catch(() => {
				availableDocuments = [];
			});
	});
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

				<Button class="rounded-md" onclick={addPage}>+ Add Page</Button>
				<Button
					variant="destructive"
					class="rounded-md"
					onclick={deletePage}
					disabled={pages.length <= 1}>- Delete Page</Button
				>
			{/if}
		</div>
	</div>

	<!-- Primary content editor + translation badges + dialog -->
	{#if isInitialLoad}
		<div class="flex flex-col gap-2">
			<div class="overflow-hidden rounded-lg border">
				<div class="bg-muted/30 flex items-center gap-1 border-b p-2">
					<Skeleton class="h-8 w-8 rounded" />
					<Skeleton class="h-8 w-8 rounded" />
					<Skeleton class="h-8 w-8 rounded" />
				</div>
				<div class="p-4" style="min-height: 300px;">
					<Skeleton class="mb-3 h-4 w-3/4" />
					<Skeleton class="mb-3 h-4 w-full" />
					<Skeleton class="mb-3 h-4 w-5/6" />
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
			{availableDocuments}
			{conversationId}
			onSaveSource={handleSaveSource}
			onSaveTarget={handleSaveTarget}
			onAiTranslate={handleAiTranslate}
			onApprove={handleApprove}
			onMarkAsDraft={handleMarkAsDraft}
		/>
	{/if}
</div>
