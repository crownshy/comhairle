<script lang="ts">
	import type { LocalisedPage, ToolConfig, WorkflowStep } from '$lib/api/api';
	type Props = {
		pages: Array<LocalisedPage[]>;
		conversation_id: string;
		workflow_step: WorkflowStep;
	};

	import { apiClient } from '$lib/api/client';
	import { Carta, MarkdownEditor } from 'carta-md';
	import { slash } from '@cartamd/plugin-slash';
	import { video } from 'carta-plugin-video';
	import * as Select from '$lib/components/ui/select';

	import 'carta-md/default.css';
	import '@cartamd/plugin-slash/default.css';
	import 'carta-plugin-video/default.css';
	import DOMPurify from 'isomorphic-dompurify';
	import { Button } from '$lib/components/ui/button';

	let { conversation_id, workflow_step, pages }: Props = $props();

	// Selection state
	let currentPageIndex = $state(0);
	let currentLang = $state('en');

	// Content binding
	let content = $state('');

	let debounceTimeout: NodeJS.Timeout;

	const carta = new Carta({
		sanitizer: DOMPurify.sanitize,
		extensions: [slash(), video()]
	});

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
					{ tool_config: { ...workflow_step.tool_config, pages } },
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

	$inspect(content);
	$inspect(pages);

	function deletePage() {
		pages = pages.filter((p, i) => i !== currentPageIndex);
		currentPageIndex = Math.max(currentPageIndex - 1, 0);
	}

	function addPage() {
		pages.push([
			{
				lang: 'en',
				content: '',
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
				content: ''
			});
			content = '';
		}
	}
</script>

<!-- Controls -->
<div class="mb-4 flex items-center justify-between gap-4">
	<div class="flex items-center gap-2">
		<Select.Root
			onSelectedChange={({ value }: { value: string; label: string }) =>
				(currentPageIndex = parseInt(value))}
			selected={{ value: currentPageIndex, label: `Page ${currentPageIndex + 1}` }}
		>
			<Select.Trigger class="w-[180px]">Page {currentPageIndex + 1}</Select.Trigger>
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
		onSelectedChange={({ value }: { value: string; label: string }) => (currentLang = value)}
		selected={{ value: currentLang, label: currentLang === 'en' ? 'English' : 'Gaelic' }}
	>
		<Select.Trigger class="w-[180px]">{currentLang === 'en' ? 'English' : 'Gaelic'}</Select.Trigger>
		<Select.Content>
			<Select.Item value={'en'}>English</Select.Item>
			<Select.Item value={'gd'}>Gaelic</Select.Item>
		</Select.Content>
	</Select.Root>
</div>

<!-- Editor -->
<MarkdownEditor {carta} bind:value={content} />

<style>
	:global(.carta-font-code) {
		font-family: '...', monospace;
		font-size: 1.1rem;
		line-height: 1.1rem;
		letter-spacing: normal;
	}
</style>
