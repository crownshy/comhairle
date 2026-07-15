<script lang="ts">
	import {
		type LocalizedPage,
		type WorkflowStepWithTranslations,
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
	import { SvelteMap } from 'svelte/reactivity';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import type { ComponentProps } from 'svelte';

	interface ExtendedLocalizedPage extends LocalizedPage {
		lang: string;
		requires_validation: boolean;
	}

	interface Props {
		conversationId: string;
		conversation: ConversationWithTranslations;
		workflowStep: WorkflowStepWithTranslations;
		isLive: boolean;
	}

	let { conversationId, conversation, workflowStep, isLive }: Props = $props();

	let isInitialLoad = $state(false);

	let primaryLocale = $derived(conversation.primaryLocale ?? 'en');
	let supportedLanguages = $derived(conversation.supportedLanguages ?? ['en']);

	type Id = number;
	type Language = string;
	type Pages = Record<Id, Record<Language, ExtendedLocalizedPage>>;

	let sourceConfig = $derived(isLive ? workflowStep.toolConfig : workflowStep.previewToolConfig);

	// svelte-ignore non_reactive_update
	let pages = new SvelteMap<keyof Pages, Pages[keyof Pages]>();
	let hasLocalChanges = $state(false);

	let list: ComponentProps<typeof DraggableList>['items'] = $derived.by(() => {
		const list = [];
		for (const id of pages.keys()) {
			list.push({ id: id.toString() });
		}
		return list;
	});

	let lastPropsConfig = $state<string>('');
	$effect(() => {
		const propsConfig = JSON.stringify({
			pages: sourceConfig?.pages
		});
		if (propsConfig !== lastPropsConfig && !hasLocalChanges) {
			(sourceConfig?.pages as ExtendedLocalizedPage[][] | undefined)?.forEach((page, i) => {
				const extendedLocalizedPage: Record<Language, ExtendedLocalizedPage> = {};
				page.forEach((p) => {
					extendedLocalizedPage[p.lang] = p;
				});
				pages.set(i, extendedLocalizedPage);
			});
			lastPropsConfig = propsConfig;
			if (isInitialLoad && pages.size > 0) {
				isInitialLoad = false;
			}
		}
	});

	const localChanges = {
		dirty: () => (hasLocalChanges = true),
		clear: () => {
			hasLocalChanges = false;
			lastPropsConfig = JSON.stringify({
				pages: sourceConfig?.pages
			});
		}
	};

	let id = $state<number>(0);

	let sourceContent = $derived(pages.get(id)?.[primaryLocale]?.content ?? '');

	let targetLanguages = $derived(
		supportedLanguages.filter((lang: string) => lang !== primaryLocale)
	);

	interface PageData {
		initialContents: Record<Language, string>;
		statuses: Record<Language, TranslationStatus>;
	}
	let pageData = $derived.by((): PageData => {
		const pageData: PageData = {
			initialContents: { [primaryLocale]: pages.get(id)?.[primaryLocale]?.content ?? '' },
			statuses: {}
		};
		for (const lang of targetLanguages) {
			const translation = pages.get(id)?.[lang];
			pageData.initialContents[lang] = translation?.content ?? '';
			pageData.statuses[lang] =
				translation?.requires_validation === false ? 'approved' : 'draft';
		}
		return pageData;
	});

	async function syncPages(callback: () => Promise<void> | void, options?: SaveToServerOptions) {
		localChanges.dirty();
		await callback();
		return saveToServer(options);
	}

	function deletePage() {
		syncPages(() => {
			pages.delete(id);
		});
	}

	function addPage() {
		syncPages(() => {
			const newId =
				pages
					.entries()
					.drop(pages.size - 1)
					.next().value?.[0] ?? 0;
			const newPage: ExtendedLocalizedPage = {
				lang: primaryLocale,
				content: `# Page ${pages.size + 1}`,
				type: 'markdown',
				requires_validation: false
			};
			pages.set(newId, { [primaryLocale]: newPage });
		});
	}

	type From = 'source' | 'target';
	function upsertContent(
		from: From,
		lang: string,
		content: ExtendedLocalizedPage['content'] | undefined
	) {
		const requires_validation = from === 'target';
		return syncPages(
			() => {
				const page = pages.get(id);
				if (!page) return;
				page[lang] = {
					lang,
					type: 'markdown',
					content: content ?? page[lang]?.content ?? '',
					requires_validation
				};
				switch (from) {
					case 'source':
						for (const translation in page) {
							if (page[translation].lang !== primaryLocale) {
								page[translation].requires_validation = true;
							}
						}
						break;
					case 'target':
						break;
				}
				pages.set(id, page);
			},
			{ invalidate: false }
		);
	}

	type SaveToServerOptions = { invalidate?: boolean };
	async function saveToServer({ invalidate = true }: SaveToServerOptions = {}) {
		const configToSave: Props['workflowStep']['toolConfig'] = {
			type: 'learn',
			pages: Object.entries(pages).map((p) => p[1][1])
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
		localChanges.clear();
	}

	async function modifyValidation(lang: string, validation: boolean): Promise<void> {
		return syncPages(
			() => {
				const page = pages.get(id);
				if (!page || !page[lang]) return;
				page[lang].requires_validation = validation;
				pages.set(id, page);
			},
			{ invalidate: false }
		);
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
				<Skeleton class="h-10 w-45" />
				<Skeleton class="h-10 w-24" />
				<Skeleton class="h-10 w-28" />
			{:else}
				<DraggableList items={list} onReorder={(next) => (list = next)}>
					{#snippet children(item)}
						<Button onclick={() => (id = Number(item.id))}>Page {item.id}</Button>
					{/snippet}
				</DraggableList>
				<Button class="rounded-md" onclick={addPage}>+ Add Page</Button>
				<Button
					variant="destructive"
					class="rounded-md"
					onclick={deletePage}
					disabled={pages.size <= 1}>- Delete Page</Button
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
			onValueChange={(content) => upsertContent('source', primaryLocale, content)}
			{primaryLocale}
			{supportedLanguages}
			editorType="rich"
			minHeight="300px"
			dialogMinHeight="250px"
			dialogTitle="Translate: Page {id + 1}"
			initialContents={pageData.initialContents}
			initialStatuses={pageData.statuses}
			{availableDocuments}
			{conversationId}
			onSaveSource={(content) => upsertContent('source', primaryLocale, content)}
			onSaveTarget={(lang, content) => upsertContent('target', lang, content)}
			onAiTranslate={async (targetLang, sContent) => {
				const translatedContent = await aiTranslateContent(
					sContent,
					targetLang,
					primaryLocale
				);
				await upsertContent('target', targetLang, translatedContent);
				return { content: translatedContent, requiresValidation: true };
			}}
			onApprove={(lang) => modifyValidation(lang, true)}
			onMarkAsDraft={(lang) => modifyValidation(lang, false)}
		/>
	{/if}
</div>
