<script lang="ts">
	import { invalidate } from '$app/navigation';
	import { onMount } from 'svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import type {
		FormEntryDto,
		PartialWorkflowStep,
		QuestionOverride,
		ToolConfig
	} from '@crownshy/api-client/api';
	import { Button } from '$lib/components/ui/button';
	import { Label } from '$lib/components/ui/label';
	import { Switch } from '$lib/components/ui/switch';
	import SelectableOptionRow from '$lib/components/SelectableOptionRow.svelte';
	import DemographicItemPicker from '../../../DemographicItemPicker.svelte';
	import {
		listPublishedDemographicForms,
		listPublishedDemographicQuestions,
		type PickerLoad,
		type PublishedDemographicItem
	} from '$lib/demographicCatalogue';
	import { notifications } from '$lib/notifications.svelte.js';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { key } from '$lib/utils/invalidationKey';

	type QuestionConfig = Extract<ToolConfig, { type: 'demographicquestion' }>;
	type FormConfig = Extract<ToolConfig, { type: 'demographicform' }>;

	type Props = {
		toolConfig: QuestionConfig | FormConfig;
		conversationId: string;
		organizationId: string | null;
		workflowId: string;
		workflowStepId: string;
		isLive: boolean;
	};
	let { toolConfig, conversationId, organizationId, workflowId, workflowStepId, isLive }: Props =
		$props();

	type EntryState = { enabled: boolean; required: boolean };
	type EntriesLoad =
		| { status: 'idle' }
		| { status: 'loading' }
		| { status: 'ready'; entries: FormEntryDto[] }
		| { status: 'error' };

	const isForm = toolConfig.type === 'demographicform';
	const noun = isForm ? 'form' : 'question';

	// Seeded once from the saved config; the parent keys this component by step id.
	let picker = $state<PickerLoad>({ status: 'loading' });
	let selectedId = $state<string | null>(isForm ? toolConfig.form_id : toolConfig.question_id);
	let pinnedVersionId = $state<string | null>(
		toolConfig.version.type === 'pinned' ? toolConfig.version.version_id : null
	);
	let versionMode = $state<'latest' | 'pinned'>(toolConfig.version.type);
	let entries = $state<EntriesLoad>({ status: 'idle' });
	let entryStates = $state<Record<string, EntryState>>({});
	let saving = $state(false);

	const selectedItem = $derived(
		picker.status === 'ready'
			? (picker.items.find((item) => item.id === selectedId) ?? null)
			: null
	);
	const canSave = $derived(
		selectedId !== null && !saving && (!isForm || entries.status === 'ready')
	);

	onMount(async () => {
		const loadItems = isForm
			? listPublishedDemographicForms
			: listPublishedDemographicQuestions;
		const result = await loadItems(organizationId);
		picker = result.err !== null ? { status: 'error' } : { status: 'ready', items: result.ok };
		if (isForm && selectedId) await loadFormEntries(selectedId, toolConfig.questions);
	});

	async function loadFormEntries(formId: string, overrides: QuestionOverride[] = []) {
		entries = { status: 'loading' };
		const result = await tryCatchAsync(async () => {
			const form = await apiClient.GetDemographicForm({ params: { form_id: formId } });
			return form.latest_published?.entries ?? [];
		});
		if (result.err !== null) {
			entries = { status: 'error' };
			return;
		}
		const loaded = result.ok;
		entryStates = Object.fromEntries(
			loaded.map((entry) => {
				const override = overrides.find((o) => o.question_id === entry.question_id);
				return [
					entry.question_id,
					{
						enabled: override?.enabled ?? true,
						required: override?.required ?? entry.required
					}
				];
			})
		);
		entries = { status: 'ready', entries: loaded };
	}

	function chooseItem(item: PublishedDemographicItem) {
		selectedId = item.id;
		if (versionMode === 'pinned') pinnedVersionId = item.versionId;
		if (isForm) void loadFormEntries(item.id);
	}

	function chooseVersionMode(mode: 'latest' | 'pinned') {
		versionMode = mode;
		if (mode === 'pinned' && selectedItem) pinnedVersionId = selectedItem.versionId;
	}

	function buildToolConfig(): PartialWorkflowStep['tool_config'] {
		if (selectedId === null) return null;
		const version =
			versionMode === 'pinned' && pinnedVersionId
				? { type: 'pinned' as const, version_id: pinnedVersionId }
				: { type: 'latest' as const };
		if (!isForm) {
			return { type: 'demographicquestion', question_id: selectedId, version };
		}
		const questions =
			entries.status === 'ready'
				? entries.entries.map((entry) => ({
						question_id: entry.question_id,
						enabled: entryStates[entry.question_id].enabled,
						required: entryStates[entry.question_id].required
					}))
				: [];
		return { type: 'demographicform', form_id: selectedId, version, questions };
	}

	async function save() {
		const config = buildToolConfig();
		if (config === null) return;
		saving = true;
		const body: PartialWorkflowStep = isLive
			? { tool_config: config }
			: { preview_tool_config: config };
		const result = await tryCatchAsync(async () => {
			await apiClient.UpdateConversationWorkflowStep(body, {
				params: {
					conversation_id: conversationId,
					workflow_id: workflowId,
					workflow_step_id: workflowStepId
				}
			});
		});
		saving = false;
		if (result.err !== null) {
			notifications.send({ priority: 'ERROR', message: `Could not save this ${noun}` });
			return;
		}
		await invalidate(key('admin/conversation/workflow'));
		notifications.send({
			priority: 'INFO',
			message: `${noun[0].toUpperCase()}${noun.slice(1)} saved`
		});
	}
</script>

<div class="flex max-w-3xl flex-col gap-8 p-6">
	<section class="flex flex-col gap-3">
		<h3 class="text-lg font-semibold">Choose a {noun}</h3>
		<DemographicItemPicker {noun} load={picker} {selectedId} onSelect={chooseItem} />
	</section>

	{#if selectedItem}
		<section class="flex flex-col gap-3">
			<h3 class="text-lg font-semibold">Version</h3>
			<SelectableOptionRow
				selected={versionMode === 'latest'}
				name="Always the latest published version"
				description="Participants see the newest published version when they reach this step."
				onSelect={() => chooseVersionMode('latest')}
			/>
			<SelectableOptionRow
				selected={versionMode === 'pinned'}
				name="Keep this version"
				description={`${selectedItem.detail}. Newer versions will not change this step.`}
				onSelect={() => chooseVersionMode('pinned')}
			/>
		</section>
	{/if}

	{#if isForm && selectedId}
		<section class="flex flex-col gap-3">
			<h3 class="text-lg font-semibold">Questions in this form</h3>
			<p class="text-muted-foreground text-base">
				Turn a question off to leave it out of this step, or change whether it is required
				here.
			</p>
			{#if entries.status === 'loading' || entries.status === 'idle'}
				<p class="text-muted-foreground text-base">Loading questions…</p>
			{:else if entries.status === 'error'}
				<p class="text-destructive text-base">Could not load the questions in this form.</p>
			{:else}
				{#each entries.entries as entry (entry.question_id)}
					<div
						class="bg-card flex items-start justify-between gap-6 rounded-lg border p-4"
					>
						<p class="text-base">{entry.question_text}</p>
						<div class="flex shrink-0 flex-col gap-2">
							<Label class="flex items-center gap-2 text-sm font-normal">
								<Switch bind:checked={entryStates[entry.question_id].enabled} />
								Ask
							</Label>
							<Label class="flex items-center gap-2 text-sm font-normal">
								<Switch
									bind:checked={entryStates[entry.question_id].required}
									disabled={!entryStates[entry.question_id].enabled}
								/>
								Required
							</Label>
						</div>
					</div>
				{/each}
			{/if}
		</section>
	{/if}

	<div>
		<Button onclick={save} disabled={!canSave}>Save {noun}</Button>
	</div>
</div>
