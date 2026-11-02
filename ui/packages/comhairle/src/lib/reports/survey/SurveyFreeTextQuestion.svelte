<!--
	@component One free-text heyform question's "Responses" / "Themes" tabs. Themes
	is lazy: its theme collection is only fetched the first time that tab is opened,
	and ThemeExplorer only mounts once that fetch settles, so it always mounts with
	the right `initialThemeCollection` (it has no reactivity of its own for that prop
	changing after mount).
-->
<script lang="ts">
	import Responses from './Responses.svelte';
	import ThemeExplorer from '$lib/components/report/ThemeExplorer.svelte';
	import * as Tabs from '$lib/components/ui/tabs';
	import type { ThemeCollectionDto } from '@crownshy/api-client/api';
	import { apiClient } from '@crownshy/api-client/client';
	import { tryCatchAsync, apiErrorStatus } from '$lib/utils/errorHandling';

	type Props = {
		conversationId: string;
		workflowId: string;
		workflowStepId: string;
		questionId: string;
		questionTitle: string;
		answers: string[];
	};

	let { conversationId, workflowId, workflowStepId, questionId, questionTitle, answers }: Props =
		$props();

	let activeTab = $state('responses');
	let themeCollectionReady = $state(false);
	let loadingThemeCollection = $state(false);
	let initialThemeCollection = $state<ThemeCollectionDto | null>(null);

	async function loadThemeCollection() {
		if (themeCollectionReady || loadingThemeCollection) return;
		loadingThemeCollection = true;

		const res = await tryCatchAsync(() =>
			apiClient.GetLatestThemeCollection({
				params: {
					conversation_id: conversationId,
					workflow_id: workflowId,
					workflow_step_id: workflowStepId
				},
				queries: { source_tool: 'heyform', sub_scope: `question:${questionId}` }
			})
		);

		if (res.err !== null) {
			if (apiErrorStatus(res.err) !== 404) {
				console.error('GetLatestThemeCollection failed', res.err);
			}
		} else {
			initialThemeCollection = res.ok;
		}

		loadingThemeCollection = false;
		themeCollectionReady = true;
	}

	function onTabChange(value: string) {
		activeTab = value;
		if (value === 'themes') loadThemeCollection();
	}
</script>

<Tabs.Root value={activeTab} onValueChange={onTabChange}>
	<Tabs.List class="w-fit">
		<Tabs.Trigger value="responses">Responses</Tabs.Trigger>
		<Tabs.Trigger value="themes">Themes</Tabs.Trigger>
	</Tabs.List>
	{#if activeTab === 'responses'}
		<Tabs.Content value="responses">
			<Responses data={answers} />
		</Tabs.Content>
	{/if}
	{#if activeTab === 'themes'}
		<Tabs.Content value="themes">
			{#if themeCollectionReady}
				<ThemeExplorer
					{conversationId}
					{workflowId}
					{workflowStepId}
					sourceTool="heyform"
					subScope={`question:${questionId}`}
					defaultContext={`Responses to the heyform question "${questionTitle}"`}
					{initialThemeCollection}
				/>
			{/if}
		</Tabs.Content>
	{/if}
</Tabs.Root>
