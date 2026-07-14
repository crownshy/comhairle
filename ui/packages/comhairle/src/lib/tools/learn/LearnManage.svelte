<script lang="ts">
	import {
		type LocalizedPage,
		type WorkflowStepWithTranslations,
		type ConversationWithTranslations,
		type ComhairleDocument,
		type ToolConfig
	} from '@crownshy/api-client/api';
	import { apiClient } from '@crownshy/api-client/client';
	import { invalidateAll } from '$app/navigation';
	import { notifications } from '$lib/notifications.svelte';
	import { Button } from '$lib/components/ui/button';
	import TranslatableField from '$lib/components/Translation/TranslatableField.svelte';
	import {
		aiTranslateContent,
		type TranslationStatus
	} from '$lib/components/Translation/translationUtils';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import DraggableList from '$lib/components/DraggableList.svelte';
	import { SvelteMap } from 'svelte/reactivity';

	interface ExtendedLocalizedPage extends LocalizedPage {
		lang: string;
		requires_validation: boolean;
	}

	type Props = {
		conversationId: string;
		conversation: ConversationWithTranslations;
		workflowStep: Omit<WorkflowStepWithTranslations, 'toolConfig'> &
			Extract<ToolConfig, { type: 'learn' }>;
		isLive: boolean;
	};

	let { conversationId, conversation, workflowStep, isLive }: Props = $props();

	let isInitialLoad = $state(true);

	let primaryLocale = $derived(conversation.primaryLocale ?? 'en');
	let supportedLanguages = $derived(conversation.supportedLanguages ?? ['en']);

	type Pages = Record<number, ExtendedLocalizedPage[]>;

	let sourceConfig = $derived(
		(isLive ? workflowStep.toolConfig : workflowStep.previewToolConfig)
	);

	// svelte-ignore non_reactive_update
	let pages = new SvelteMap<keyof Pages, Pages[keyof Pages]>();
	let hasLocalChanges = $state(false);

	let lastPropsConfig = $state<string>('');
	$effect(() => {
		const propsConfig = JSON.stringify({
			pages: sourceConfig?.pages
		});
		if (propsConfig !== lastPropsConfig && !hasLocalChanges) {
			pages = structuredClone(sourceConfig?.pages ?? []);
			lastPropsConfig = propsConfig;
			if (isInitialLoad && pages.size > 0) {
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

	let id = $state<number>(0);

	function getTranslation(lang: string): ExtendedLocalizedPage | undefined {
		return pages.get(id)?.find((p) => p.lang === lang);
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
		syncPage(() => {
			pages.delete(id);
		});
	}

	function syncPage(callback: () => void, options?: SaveToServerOptions) {
		markLocalChanges();
		callback();
		saveToServer(options);
	}

	function addPage() {
		syncPage(() => {
			const newId =
				pages
					.entries()
					.drop(pages.size - 1)
					.next().value?.[0] ?? 0;
			const newPage: ExtendedLocalizedPage[] = [
				{
					lang: primaryLocale,
					content: `# Page ${pages.size + 1}`,
					type: 'markdown',
					requires_validation: false
				}
			];
			pages.set(newId, newPage);
		});
	}

	function upsertPage(
		lang: string,
		options?: Partial<Pick<ExtendedLocalizedPage, 'content' | 'requires_validation'>>
	) {
		syncPage(() => {
			const page = pages.get(id);
			if (!page) return;
			const existing = getTranslation(lang);
			if (!existing) {
				pages.set(
					id,
					page.concat([
						{
							lang,
							type: 'markdown',
							content: options?.content ?? '',
							requires_validation: options?.requires_validation ?? false
						}
					])
				);
				return;
			}
			if (!!options?.content) {
				existing.content = options.content;
			}
		});
	}

	type SaveToServerOptions = { invalidate?: boolean };
	async function saveToServer({ invalidate = true }: SaveToServerOptions = {}) {
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
		const source = getTranslation(primaryLocale);
		if (source) {
			source.content = content;
			source.type = 'markdown';
		} else if (!!pages.get(id)?.length) {
			pages.set(id, [
				{
					lang: primaryLocale,
					type: 'markdown',
					content,
					requires_validation: false
				}
			]);
		}
		pages.set(
			id,
			pages.get(id)?.map((p) => {
				if (p.lang !== primaryLocale) {
					p.requires_validation = true;
				}
				return p;
			}) ?? []
		);
		saveToServer({ invalidate: false });
	}

	function handleSaveTarget(lang: string, content: string) {
		markLocalChanges();
		const target = getTranslation(lang);
		if (target) {
			target.content = content;
			target.type = 'markdown';
			target.requires_validation = true;
		} else if (!!pages.get(id)?.length) {
			pages.set(
				id,
				pages.get(id).concat({
					lang,
					type: 'markdown',
					content,
					requires_validation: true
				})
			);
		}
		pages = [...pages];
		saveToServer({ invalidate: false });
	}

	async function handleAiTranslate(
		targetLang: string,
		sContent: string
	): Promise<{ content: string; requiresValidation: boolean }> {
		const translatedContent = await aiTranslateContent(sContent, targetLang, primaryLocale);
		let t = getTranslation(targetLang);
		if (t) {
			t.content = translatedContent;
			t.requires_validation = true;
		} else {
			pages[currentPageIndex].push({
				lang: targetLang,
				type: 'markdown',
				content: translatedContent,
				requires_validation: true
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
				<DraggableList items={pages}>
					{#snippet children(_, i)}
						<div>Page {i}</div>
					{/snippet}
				</DraggableList>
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
			dialogTitle="Translate: Page {id + 1}"
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
