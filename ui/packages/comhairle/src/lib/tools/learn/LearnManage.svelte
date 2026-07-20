<script lang="ts">
	import {
		type ConversationWithTranslations,
		type ComhairleDocument
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
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { onMount } from 'svelte';
	import { GripVertical } from 'lucide-svelte';
	import { Badge } from '$lib/components/ui/badge';
	import Pages, { type ExtendedLocalizedPage, type Language } from './Pages.svelte';
	import type {
		InstancedToolConfig,
		WorkflowStepWithTranslationsAndTool
	} from '$lib/tools/types';

	interface Props {
		conversationId: string;
		conversation: ConversationWithTranslations;
		workflowStep: WorkflowStepWithTranslationsAndTool<'learn'>;
		isLive: boolean;
	}

	let { conversationId, conversation, workflowStep, isLive }: Props = $props();

	let isInitialLoad = $state(false);
	let primaryLocale = $derived(conversation.primaryLocale ?? 'en');
	let supportedLanguages = $derived(conversation.supportedLanguages ?? ['en']);

	// FIX: Remove this after the types have been fixed on the backend
	type LearnToolConfig = Exclude<InstancedToolConfig<'learn'>, 'pages'> & {
		pages: ExtendedLocalizedPage[][];
	};
	let sourceConfig: LearnToolConfig = $derived(
		(isLive ? workflowStep.toolConfig : workflowStep.previewToolConfig) as LearnToolConfig
	);

	const pages = new Pages();

	type SaveToServerOptions = { invalidate?: boolean };
	async function save(
		pagesToSave: ExtendedLocalizedPage[][],
		{ invalidate = true }: SaveToServerOptions = {}
	) {
		const configToSave: Props['workflowStep']['toolConfig'] = {
			type: 'learn',
			pages: pagesToSave
		};

		const response = await tryCatchAsync(() =>
			apiClient.UpdateConversationWorkflowStep(
				isLive ? { tool_config: configToSave } : { preview_tool_config: configToSave },
				{
					params: {
						workflow_id: workflowStep.workflowId,
						conversation_id: conversationId,
						workflow_step_id: workflowStep.id
					}
				}
			)
		);

		if (response.err !== null) {
			notifications.send({ message: 'Failed to save changes', priority: 'ERROR' });
		}

		if (invalidate) await invalidateAll();
		pages.restore();
	}

	pages.onChange((options) => {
		pages.dirty();
		return save(pages.toLocalizedPages(), { invalidate: options?.invalidate ?? true });
	});

	pages.onRestore(() => {
		lastPropsConfig = JSON.stringify({
			pages: sourceConfig?.pages
		});
	});

	onMount(() => {
		pages.load(sourceConfig?.pages ?? []);
	});

	let lastPropsConfig = $state<string>('');
	$effect(() => {
		const propsConfig = JSON.stringify({
			pages: sourceConfig.pages
		});
		if (propsConfig !== lastPropsConfig && !pages.areDirty) {
			pages.load(sourceConfig.pages);
			lastPropsConfig = propsConfig;
			if (isInitialLoad && Object.keys(pages).length > 0) {
				isInitialLoad = false;
			}
		}
	});

	let sourceContent = $derived(pages.items[pages.currentId]?.[primaryLocale]?.content ?? '');
	let targetLanguages = $derived(
		supportedLanguages.filter((lang: string) => lang !== primaryLocale)
	);

	interface PageData {
		initialContents: Record<Language, string>;
		statuses: Record<Language, TranslationStatus>;
	}
	let pageData = $derived.by((): PageData => {
		const pageData: PageData = {
			initialContents: {
				[primaryLocale]: pages.items[pages.currentId]?.[primaryLocale]?.content ?? ''
			},
			statuses: {}
		};
		for (const lang of targetLanguages) {
			const translation = pages.items[pages.currentId]?.[lang];
			pageData.initialContents[lang] = translation?.content ?? '';
			pageData.statuses[lang] =
				translation?.requires_validation === false ? 'approved' : 'draft';
		}
		return pageData;
	});

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
				<Skeleton class="h-10 w-45" />
				<Skeleton class="h-10 w-24" />
				<Skeleton class="h-10 w-28" />
			{:else}
				<DraggableList
					items={pages.order}
					onReorder={(order) => pages.reorder(order)}
					class="bg-muted flex flex-row items-center gap-2"
				>
					{#snippet children(item, i)}
						<Badge
							class="flex flex-row items-center px-4 py-2"
							variant={Number(item.id) === pages.currentId ? 'primary' : 'outline'}
						>
							<GripVertical
								class="text-muted-foreground cursor-grab {Number(item.id) ===
								pages.currentId
									? 'text-primary-foreground'
									: ''}"
								size={16}
							/>
							<label class="cursor-pointer"
								>Page {i + 1}
								<input
									class="hidden"
									type="radio"
									name="currentId"
									value={Number(item.id)}
									bind:group={pages.currentId}
								/>
							</label>
						</Badge>
					{/snippet}
				</DraggableList>
				<Button variant="ghost" onclick={() => pages.new(primaryLocale)}>+ Add Page</Button>
				<Button
					variant="destructive"
					class="rounded-md"
					onclick={() => pages.current.delete()}
					disabled={pages.count <= 1}>- Delete Page</Button
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
			onValueChange={(content) =>
				pages.current.upsertContent('source', primaryLocale, content)}
			{primaryLocale}
			{supportedLanguages}
			editorType="rich"
			minHeight="300px"
			dialogMinHeight="250px"
			dialogTitle="Translate: Page {pages.currentId + 1}"
			translation={workflowStep.translations.description}
			initialContents={pageData.initialContents}
			initialStatuses={pageData.statuses}
			{availableDocuments}
			{conversationId}
			onSaveSource={(content) =>
				pages.current.upsertContent('source', primaryLocale, content)}
			onSaveTarget={(lang, content) => pages.current.upsertContent('target', lang, content)}
			onAiTranslate={async (targetLang, sContent) => {
				const translatedContent = await aiTranslateContent(
					sContent,
					targetLang,
					primaryLocale
				);
				await pages.current.upsertContent('target', targetLang, translatedContent);
				return { content: translatedContent, requiresValidation: true };
			}}
			onApprove={(lang) => pages.current.modifyValidation(lang, true)}
			onMarkAsDraft={(lang) => pages.current.modifyValidation(lang, false)}
		/>
	{/if}
</div>
