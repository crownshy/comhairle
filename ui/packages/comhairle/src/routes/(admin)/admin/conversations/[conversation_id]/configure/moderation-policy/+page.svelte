<script lang="ts">
	import { onDestroy, tick } from 'svelte';
	import { invalidate } from '$app/navigation';
	import { apiClient } from '@crownshy/api-client/client';
	import { Button } from '$lib/components/ui/button';
	import { Plus, RotateCcw, Trash2 } from 'lucide-svelte';
	import { cn } from '$lib/utils';
	import { guardUnsavedChanges } from '$lib/utils/unsavedChangesGuard.svelte';
	import {
		cleanRejectReasons,
		policyStepUpdates,
		rejectReasonLabelProblems,
		rejectReasonsFromPolicy,
		withSavedReasonIds
	} from '$lib/moderation/moderationPolicy';
	import { Autosave } from './autosave.svelte';
	import SaveStatusPill from './SaveStatusPill.svelte';
	import { notifications } from '$lib/notifications.svelte';
	import type { UpdateModerationPolicyReason } from '@crownshy/api-client/api';

	let { data, params } = $props();

	// `key` is a stable {#each} key: labels can be blank or repeated while editing, and a new
	// reason has no id until its first save lands.
	type Row = {
		key: number;
		label: string;
		description: string;
	};

	const toRows = (reasons: UpdateModerationPolicyReason[]): Row[] =>
		reasons.map((reason, index) => ({
			...reason,
			key: index + 1,
			description: reason.description ?? ''
		}));

	let policyId = $derived<string | null>(data.policies[0]?.id ?? null);

	// $state needed for deep reactivity, should be ok since we're only using the initial value, derived isn't necessary
	let rows = $state<Row[]>(
		toRows(data.policies[0] ? rejectReasonsFromPolicy(data.policies[0]) : data.defaultReasons)
	);

	// While true the conversation has no policy and uses the default reasons, so a later change
	// to the defaults reaches it. Any edit creates the conversation's own policy.
	let usingDefault = $derived(data.policies.length === 0);

	// Why each row's label won't be saved, by position. Blank rows are still being typed, so
	// they aren't flagged.
	const labelProblems = $derived(rejectReasonLabelProblems(rows.map((row) => row.label)));
	const flaggedRows = $derived(
		labelProblems.map((problem) => problem === 'duplicate' || problem === 'contains-separator')
	);

	// Where this editor last pointed each step, since `steps` isn't reloaded until it closes.
	// Only saves read it, never the markup, so it doesn't need to be reactive.
	// eslint-disable-next-line svelte/prefer-svelte-reactivity
	const pointedSteps = new Map<string, string | null>();

	async function pointStepsAt(targetPolicyId: string | null) {
		const workflow = await data.streamedWorkflow;
		if (workflow.err !== null) {
			console.error(workflow.err);
			notifications.addFlash({
				message:
					"Unable to update the conversation because the workflow data wasn't able to load. Please try reloading the page",
				priority: 'ERROR'
			});
			return;
		}
		if (!workflow.ok.id) return;
		for (const { stepId, body } of policyStepUpdates(
			workflow.ok.steps,
			targetPolicyId,
			pointedSteps
		)) {
			await apiClient.UpdateConversationWorkflowStep(body, {
				params: {
					conversation_id: params.conversation_id,
					workflow_id: workflow.ok.id,
					workflow_step_id: stepId
				}
			});
			pointedSteps.set(stepId, targetPolicyId);
		}
	}

	// Steps are pointed at nothing before a delete, because the API refuses to delete a policy
	// a step still uses.
	async function save() {
		if (usingDefault) {
			if (policyId === null) return;
			await pointStepsAt(null);
			await apiClient.DeleteConversationModerationPolicy(undefined, {
				params: { conversation_id: params.conversation_id, moderation_policy_id: policyId }
			});
			policyId = null;
			return;
		}

		const NEW_POLICY_NAME = 'Moderation policy';
		const policyName = data.policies[0]?.name ?? NEW_POLICY_NAME;

		const reasons = cleanRejectReasons(rows);
		const saved =
			policyId === null
				? await apiClient.CreateConversationModerationPolicy(
						{
							name: policyName,
							reasons: reasons.map(({ label, description }) => ({
								label,
								description
							}))
						},
						{ params: { conversation_id: params.conversation_id } }
					)
				: await apiClient.UpdateConversationModerationPolicy(
						{ name: policyName, reasons },
						{
							params: {
								conversation_id: params.conversation_id,
								moderation_policy_id: policyId
							}
						}
					);
		policyId = saved.id;
		rows = withSavedReasonIds(rows, rejectReasonsFromPolicy(saved));
		await pointStepsAt(saved.id);
	}

	const autosave = new Autosave(save);

	guardUnsavedChanges(() => autosave.dirty);

	// The Moderation tab reads the policies and steps from the conversation layout's data, so
	// once a save lands that data is refreshed when this editor goes away. The same load
	// returns the steps, so one key refreshes both.
	onDestroy(() => {
		autosave.flush();
		autosave.settled().then(() => {
			if (autosave.hasSaved) invalidate('conversation:moderation-policy');
		});
	});

	function editRow(key: number, patch: Partial<Pick<Row, 'label' | 'description'>>) {
		rows = rows.map((row) => (row.key === key ? { ...row, ...patch } : row));
		usingDefault = false;
		autosave.schedule();
	}

	async function addRow() {
		const row: Row = { key: rows[rows.length - 1].key + 1, label: '', description: '' };
		rows.push(row);
		await tick();
		document.querySelector<HTMLInputElement>(`#reject-reason-label-${row.key}`)?.focus();
	}

	function removeRow(key: number) {
		const index = rows.findIndex((row) => row.key === key);
		if (index < 0) {
			return;
		}
		rows.splice(index, 1);
		usingDefault = false;
		autosave.schedule();
	}

	function resetToDefault() {
		rows = toRows(data.defaultReasons);
		usingDefault = true;
		autosave.schedule();
	}

	const gridClass = 'grid-cols-[minmax(10rem,16rem)_minmax(12rem,1fr)_auto]';
</script>

<div class="flex flex-col gap-6">
	<div class="flex flex-wrap items-center gap-2">
		<Button variant="outline" onclick={addRow}>
			<Plus class="mr-1.5 size-4" /> Add reason
		</Button>
		{#if !usingDefault}
			<Button
				variant="ghost"
				onclick={resetToDefault}
				title="Replace this list with the default reasons"
			>
				<RotateCcw class="mr-1.5 size-4" /> Reset to default
			</Button>
		{/if}

		<SaveStatusPill status={autosave.status} />
	</div>

	{#if usingDefault}
		<p class="text-muted-foreground text-base">
			This conversation uses the default reasons. Edit, add or remove one and the list becomes
			this conversation's own.
		</p>
	{/if}

	<div class="border-border overflow-hidden rounded-lg border">
		<div class="overflow-x-auto">
			<div class="min-w-120">
				<div
					class={cn(
						'bg-muted/50 text-muted-foreground border-border grid border-b text-sm font-medium',
						gridClass
					)}
				>
					<div class="px-3 py-2">Reason</div>
					<div class="px-3 py-2">What counts (optional)</div>
					<div class="w-10"></div>
				</div>

				<div class="divide-border bg-background divide-y">
					{#each rows as row, index (row.key)}
						<div
							class={cn(
								'focus-within:bg-muted/30 hover:bg-muted/20 grid items-center',
								gridClass
							)}
						>
							<!-- Plain inputs, not the shadcn Input: these are borderless spreadsheet
								cells, matching the glossary editor. -->
							<input
								id="reject-reason-label-{row.key}"
								value={row.label}
								oninput={(event) =>
									editRow(row.key, { label: event.currentTarget.value })}
								placeholder="Off-topic"
								aria-label="Reason"
								aria-invalid={flaggedRows[index]}
								title={row.label}
								class="text-foreground placeholder:text-muted-foreground/60 aria-invalid:text-destructive h-10 truncate bg-transparent px-3 text-base font-medium outline-none"
							/>
							<input
								value={row.description}
								oninput={(event) =>
									editRow(row.key, { description: event.currentTarget.value })}
								placeholder="Shown to moderators when they pick this reason"
								aria-label="What counts under this reason"
								title={row.description}
								class="text-foreground placeholder:text-muted-foreground/60 h-10 truncate bg-transparent px-3 text-base outline-none"
							/>
							<Button
								variant="ghost"
								size="icon"
								onclick={() => removeRow(row.key)}
								aria-label="Remove reason"
								class="text-muted-foreground hover:text-destructive size-10 rounded-none"
							>
								<Trash2 class="size-4" />
							</Button>
						</div>
					{:else}
						<p class="text-muted-foreground px-3 py-8 text-center text-base">
							No reasons. Moderators can still explain a rejection in a note.
						</p>
					{/each}
				</div>
			</div>
		</div>
	</div>

	{#if labelProblems.includes('duplicate')}
		<p class="text-destructive text-base">
			Some reasons appear more than once. Only the first of each is saved.
		</p>
	{/if}
	{#if labelProblems.includes('contains-separator')}
		<p class="text-destructive text-base">
			A reason can't contain a colon followed by a space, because that separates the reason
			from the note. Those reasons aren't saved.
		</p>
	{/if}
</div>
