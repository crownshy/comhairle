<script lang="ts">
	import type { LocalisedPage, ToolConfig, WorkflowStep } from '$lib/api/api';
	type Props = {
		conversation_id: string;
		workflow_step: WorkflowStep;
		isLive: boolean;
	};

	import { apiClient } from '$lib/api/client';
	import RichTextEditor from '$lib/components/RichTextEditor/RichTextEditor.svelte';
	import * as Select from '$lib/components/ui/select';
	import { Button } from '$lib/components/ui/button';

	let { conversation_id, workflow_step, isLive }: Props = $props();

	let toolConfig = $derived(
		isLive ? workflow_step.tool_config : workflow_step.preview_tool_config
	);

	let pages = $derived(toolConfig.pages);

	// Selection state
	let currentPageIndex = $state(0);
	let currentLang = $state('en');

	// Content binding
	let content = $state('');

	let debounceTimeout: NodeJS.Timeout;

	// Get current translation
	function getCurrentTranslation() {
		return pages[currentPageIndex].find((p) => p.lang === currentLang);
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
				apiClient.UpdateWorkflowStep(
					isLive
						? { tool_config: { ...toolConfig, pages } }
						: { preview_tool_config: { ...toolConfig, pages } },
					{
						params: {
							workflow_id: workflow_step.workflow_id,
							conversation_id,
							workflow_step_id: workflow_step.id
						}
					}
				);
			}, 500);
		}
	});

	function deletePage() {
		pages = pages.filter((p, i) => i !== currentPageIndex);
		currentPageIndex = Math.max(currentPageIndex - 1, 0);
	}

	function addPage() {
		pages.push([
			{
				lang: 'en',
				content: `# Page ${pages.length + 1}`,
				type: 'markdown'
			},

			{
				lang: 'gd',
				content: `# Duilleag ${pages.length + 1}`,
				type: 'markdown'
			}
		]);
		currentPageIndex = pages.length - 1;
	}

	function addLanguage() {
		if (!getCurrentTranslation()) {
			pages[currentPageIndex].push({
				lang: currentLang,
				type: 'markdown',
				content
			});
			content = '';
		}
	}
</script>

<!-- Controls -->
<div class="flex flex-col">
	<div class="mb-4 flex items-center justify-between gap-4">
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
						<Select.Item value={i}>Page {i + 1}</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>

			<Button onclick={addPage}>+ Add Page</Button>
			<Button variant="destructive" onclick={deletePage}>- Delete Page</Button>
		</div>

		<Select.Root
			type="single"
			onValueChange={(value: string) => (currentLang = value)}
			value={currentLang}
		>
			<Select.Trigger class="w-[180px] bg-white"
				>{currentLang === 'en' ? 'English' : 'Gaelic'}</Select.Trigger
			>
			<Select.Content>
				<Select.Item value={'en'}>English</Select.Item>
				<Select.Item value={'gd'}>Gaelic</Select.Item>
			</Select.Content>
		</Select.Root>
	</div>

	<!-- Editor -->
	<div class="grow">
		<RichTextEditor value={content} onChange={(v) => (content = v)} />
	</div>
</div>
