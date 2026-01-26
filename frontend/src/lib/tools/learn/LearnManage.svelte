<script lang="ts">
	import type { LocalisedPage, ToolConfig, WorkflowStep, ConversationWithTranslations, Page } from '$lib/api/api';
	
	interface ExtendedLocalisedPage extends LocalisedPage {
		lang: string;
		requires_validation?: boolean;
	}
	
	type Props = {
		conversation_id: string;
		conversation: ConversationWithTranslations;
		workflow_step: WorkflowStep;
		isLive: boolean;
	};

	import { apiClient } from '$lib/api/client';
	import { invalidateAll } from '$app/navigation';
	import { notifications } from '$lib/notifications.svelte';
	import RichTextEditor from '$lib/components/RichTextEditor/RichTextEditor.svelte';
	import * as Select from '$lib/components/ui/select';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { Sparkles, Check, MoreHorizontal } from 'lucide-svelte';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { getLanguageName } from '$lib/config/languages';

	let { conversation_id, conversation, workflow_step, isLive }: Props = $props();

	let primaryLocale = $derived(conversation.primary_locale ?? 'en');
	let supportedLanguages = $derived(conversation.supported_languages ?? ['en']);

	type LearnToolConfig = { type: 'learn'; pages: ExtendedLocalisedPage[][] };
	
	let toolConfig = $derived(
		(isLive ? workflow_step.tool_config : workflow_step.preview_tool_config) as LearnToolConfig
	);

	let pages = $derived<ExtendedLocalisedPage[][]>(toolConfig?.pages ?? []);

	// Selection state
	let currentPageIndex = $state(0);
	let currentLang = $state('en');

	$effect(() => {
		if (primaryLocale && !supportedLanguages.includes(currentLang)) {
			currentLang = primaryLocale;
		}
	});

	// Content binding
	let content = $state('');
	let isTranslating = $state(false);
	let statusUpdateTrigger = $state(0);

	let debounceTimeout: NodeJS.Timeout;

	// Get current translation
	function getCurrentTranslation(): ExtendedLocalisedPage | undefined {
		return pages[currentPageIndex]?.find((p) => p.lang === currentLang);
	}

	// Sync content when switching page/lang
	$effect(() => {
		const t = getCurrentTranslation();
		content = t ? t.content : '';
	});

	$effect(() => {
		if (content !== undefined) {
			const t = getCurrentTranslation();
			if (t) {
				t.content = content;
				// Keep type as markdown - TipTap handles conversion
				t.type = 'markdown';
			} else {
				// Add new language to current page if not present
				pages[currentPageIndex].push({
					lang: currentLang,
					type: 'markdown',
					content
				});
			}

			// Debounced sync to server
			clearTimeout(debounceTimeout);
			debounceTimeout = setTimeout(() => {
				saveToServer();
			}, 500);
		}
	});

	function deletePage() {
		const newPages = pages.filter((_: ExtendedLocalisedPage[], i: number) => i !== currentPageIndex);
		toolConfig.pages = newPages;
		currentPageIndex = Math.max(currentPageIndex - 1, 0);
		saveToServer();
	}

	function addPage() {
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

	function addLanguage() {
		if (!getCurrentTranslation()) {
			pages[currentPageIndex].push({
				lang: currentLang,
				type: 'markdown',
				content,
				requires_validation: currentLang !== primaryLocale
			});
			content = '';
		}
	}

	let translationStatus = $derived.by(() => {
		void statusUpdateTrigger;
		if (currentLang === primaryLocale) return 'primary';
		const t = getCurrentTranslation();
		if (!t) return 'draft';
		return t.requires_validation === false ? 'approved' : 'draft';
	});

	async function approveTranslation() {
		const t = getCurrentTranslation();
		if (!t || currentLang === primaryLocale) return;

		t.requires_validation = false;
		statusUpdateTrigger++;

		await saveToServerQuiet();
		notifications.send({ message: 'Translation approved', priority: 'INFO' });
	}

	async function markAsDraft() {
		const t = getCurrentTranslation();
		if (!t || currentLang === primaryLocale) return;

		t.requires_validation = true;
		statusUpdateTrigger++;

		await saveToServerQuiet();
		notifications.send({ message: 'Translation marked as draft', priority: 'INFO' });
	}

	async function saveToServer() {
		clearTimeout(debounceTimeout);
		try {
			await apiClient.UpdateWorkflowStep(
				isLive
					? { tool_config: toolConfig }
					: { preview_tool_config: toolConfig },
				{
					params: {
						workflow_id: workflow_step.workflow_id,
						conversation_id,
						workflow_step_id: workflow_step.id
					}
				}
			);
			await invalidateAll();
		} catch (e) {
			notifications.send({ message: 'Failed to save changes', priority: 'ERROR' });
		}
	}

	async function saveToServerQuiet() {
		clearTimeout(debounceTimeout);
		try {
			await apiClient.UpdateWorkflowStep(
				isLive
					? { tool_config: toolConfig }
					: { preview_tool_config: toolConfig },
				{
					params: {
						workflow_id: workflow_step.workflow_id,
						conversation_id,
						workflow_step_id: workflow_step.id
					}
				}
			);
		} catch (e) {
			notifications.send({ message: 'Failed to save changes', priority: 'ERROR' });
		}
	}

	async function handleAiTranslate() {
		if (isTranslating || currentLang === primaryLocale) return;

		const primaryTranslation = pages[currentPageIndex]?.find((p) => p.lang === primaryLocale);
		if (!primaryTranslation?.content) {
			notifications.send({ message: 'No primary content to translate from', priority: 'WARNING' });
			return;
		}

		isTranslating = true;
		try {
			const textContent = await apiClient.CreateTextContent({
				primary_locale: primaryLocale,
				format: 'markdown',
				content: primaryTranslation.content
			});

			await apiClient.CreateOrUpdateTextTranslation(
				{
					content: '',
					ai_generated: true,
					requires_validation: true
				},
				{
					params: {
						text_content_id: textContent.id,
						locale: currentLang
					}
				}
			);

			const translation = await apiClient.AutomaticallyGenerateTranslation(undefined, {
				params: {
					text_content_id: textContent.id,
					locale: currentLang
				}
			});

			let t = getCurrentTranslation();
			if (t) {
				t.content = translation.content;
				t.requires_validation = true;
			} else {
				pages[currentPageIndex].push({
					lang: currentLang,
					type: 'markdown',
					content: translation.content,
					requires_validation: true
				});
			}
			content = translation.content;

			await saveToServer();

			await apiClient.DeleteTextContent(undefined, { params: { text_content_id: textContent.id } });

			notifications.send({ message: 'Translation completed', priority: 'INFO' });
		} catch (error) {
			console.error('AI translation failed:', error);
			notifications.send({ message: 'AI translation failed', priority: 'ERROR' });
		} finally {
			isTranslating = false;
		}
	}
</script>

<!-- Controls -->
<div class="flex flex-col gap-4">
	<!-- Top row: Page controls -->
	<div class="flex items-center justify-between gap-4">
		<div class="flex items-center gap-2">
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
		</div>
	</div>

	<!-- Second row: Language selector with status and actions -->
	<div class="flex items-center justify-between gap-4 rounded-lg bg-muted/50">
		<div class="flex items-center gap-3">
			<Select.Root
				type="single"
				onValueChange={(value: string) => (currentLang = value)}
				value={currentLang}
			>
				<Select.Trigger class="w-[180px] bg-white">
					{getLanguageName(currentLang)}
				</Select.Trigger>
				<Select.Content>
					{#each supportedLanguages as lang}
						<Select.Item value={lang}>
							{getLanguageName(lang)}
							{#if lang === primaryLocale}
								<span class="ml-2 text-xs text-muted-foreground">(Primary)</span>
							{/if}
						</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>

			<Badge variant={translationStatus === 'primary' ? 'outline' : translationStatus === 'approved' ? 'default' : 'secondary'}>
				{translationStatus === 'primary' ? 'Primary' : translationStatus === 'approved' ? 'Approved' : 'Draft'}
			</Badge>
		</div>

		<!-- Actions for non-primary languages -->
		{#if currentLang !== primaryLocale}
			<div class="flex items-center gap-2">
				<Button onclick={handleAiTranslate} disabled={isTranslating} variant="outline" size="sm">
					{#if isTranslating}
						Translating...
					{:else}
						<Sparkles class="mr-1 size-4" />
						AI Translate
					{/if}
				</Button>

				{#if translationStatus === 'approved'}
					<Button disabled variant="outline" size="sm" class="gap-1">
						<Check class="size-4" />
						Approved
					</Button>
					<DropdownMenu.Root>
						<DropdownMenu.Trigger>
							<Button variant="outline" size="icon" class="h-8 w-8">
								<MoreHorizontal class="size-4" />
							</Button>
						</DropdownMenu.Trigger>
						<DropdownMenu.Content>
							<DropdownMenu.Item onclick={markAsDraft}>
								Mark as draft
							</DropdownMenu.Item>
						</DropdownMenu.Content>
					</DropdownMenu.Root>
				{:else}
					<Button 
						onclick={approveTranslation}
						disabled={!content}
						size="sm"
						class="gap-1"
					>
						<Check class="size-4" />
						Approve
					</Button>
				{/if}
			</div>
		{/if}
	</div>

	<!-- Editor -->
	<div class="grow">
		<RichTextEditor value={content} onChange={(v) => (content = v)} />
	</div>
</div>
