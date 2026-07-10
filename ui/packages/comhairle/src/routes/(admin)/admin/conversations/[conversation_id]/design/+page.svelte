<script lang="ts">
	import type { WorkflowStepWithTranslations } from '@crownshy/api-client/api';
	import { tick } from 'svelte';
	import { invalidate } from '$app/navigation';
	import { apiClient } from '@crownshy/api-client/client';
	import { notifications } from '$lib/notifications.svelte.js';
	import { saveTranslation } from '$lib/components/Translation/translationUtils';
	import DraggableList from '$lib/components/DraggableList.svelte';
	import StepPreview from '$lib/components/StepPreview.svelte';
	import ToolPalette from '$lib/components/ToolPalette.svelte';
	import { Button } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import {
		toolMeta,
		DATA_PROTOCOLS,
		protocolFromBool,
		boolFromProtocol,
		type DataProtocol
	} from '$lib/tool_meta';
	import {
		basic_learn_config,
		basic_polis_config,
		basic_survey_config,
		basic_lived_experience_config,
		basic_elicitation_bot_config,
		basic_thinking_space_config,
		basic_prioritization_config,
		defaultStepCreationParams,
		workflow_templates
	} from '$lib/workflow_templates.js';
	import {
		Pencil,
		Trash2,
		GripVertical,
		X,
		Lightbulb,
		Database,
		Clock,
		Minus,
		Plus,
		Lock,
		LockOpen,
		ChevronDown,
		ArrowUpRight,
		Check,
		LoaderCircle
	} from 'lucide-svelte';

	let { data } = $props();

	let conversation = $derived(data.conversation);
	let workflow = $derived(data.workflows[0]);
	let workflowSteps = $derived<WorkflowStepWithTranslations[] | undefined>(data.workflowSteps);

	let reorderedSteps = $state<WorkflowStepWithTranslations[]>([]);
	$effect(() => {
		reorderedSteps = workflowSteps
			? [...workflowSteps].sort((a, b) => a.stepOrder - b.stepOrder)
			: [];
	});

	// --- Toolbar chip toggles (reveal per-card pills) ---
	let showTime = $state(false);
	let showProtocol = $state(false);
	let showLock = $state(false);

	// --- Ephemeral UI state ---
	let editingId = $state<string | null>(null);
	let editValue = $state('');
	let adding = $state(false);
	let boardEl = $state<HTMLDivElement | null>(null);

	// --- Templates (re-seed the whole workflow) ---
	// Keys map to the step arrays in workflow_templates. The workflow doesn't persist
	// which template it came from, so the chip label is session-local (resets on reload).
	const TEMPLATES = [
		{ key: 'empty', label: 'Blank' },
		{ key: 'learn_polis', label: 'Learn + Wiki Poll' },
		{ key: 'learn_survey', label: 'Learn + Survey' },
		{ key: 'learn_survey_polis', label: 'Learn + Survey + Wiki Poll' }
	];
	let currentTemplateLabel = $state('Blank');
	let pendingTemplate = $state<{ key: string; label: string } | null>(null);
	let templateDialogOpen = $state(false);
	let applyingTemplate = $state(false);

	function chooseTemplate(t: { key: string; label: string }) {
		pendingTemplate = t;
		templateDialogOpen = true;
	}

	async function applyTemplate() {
		const t = pendingTemplate;
		if (!t) return;
		applyingTemplate = true;
		const steps = workflow_templates[t.key as keyof typeof workflow_templates] ?? [];
		try {
			// Replace the workflow: delete every existing step, then create the template's.
			for (const s of reorderedSteps) {
				await apiClient.DeleteConversationWorkflowStep(undefined, {
					params: {
						conversation_id: conversation.id,
						workflow_id: workflow.id,
						workflow_step_id: s.id
					}
				});
			}
			let order = 1;
			for (const st of steps) {
				await apiClient.CreateConversationWorkflowStep(
					{
						name: st.name,
						description: st.description,
						is_offline: st.is_offline,
						activation_rule: 'manual',
						step_order: order++,
						tool_setup: st.tool_setup,
						required: st.required
					},
					{ params: { conversation_id: conversation.id, workflow_id: workflow.id } }
				);
			}
			await invalidate('conversation:workflow');
			currentTemplateLabel = t.label;
			notifications.send({ priority: 'INFO', message: 'Template applied' });
		} catch (e) {
			console.error(e);
			notifications.send({ priority: 'ERROR', message: 'Failed to apply template' });
			await invalidate('conversation:workflow');
		} finally {
			applyingTemplate = false;
			pendingTemplate = null;
			templateDialogOpen = false;
		}
	}
	// Estimated time is not persisted yet (hardcoded per tool); edits live client-side.
	let timeOverrides = $state<Record<string, number>>({});
	let dismissed = $state<Record<string, { desc?: boolean; protocol?: boolean }>>({});

	function stepType(step: WorkflowStepWithTranslations): string | undefined {
		return step.previewToolConfig?.type ?? step.toolConfig?.type;
	}
	function estMinutes(step: WorkflowStepWithTranslations): number {
		return timeOverrides[step.id] ?? toolMeta(stepType(step))?.estimatedMinutes ?? 5;
	}
	function protocolOf(step: WorkflowStepWithTranslations): DataProtocol {
		return protocolFromBool(step.requestUserSharePermission);
	}

	async function patchStep(step: WorkflowStepWithTranslations, body: Record<string, unknown>) {
		await apiClient.UpdateConversationWorkflowStep(body, {
			params: {
				conversation_id: conversation.id,
				workflow_id: workflow.id,
				workflow_step_id: step.id
			}
		});
	}

	// --- Reorder ---
	function handleReorder(next: WorkflowStepWithTranslations[]) {
		reorderedSteps = next;
	}
	async function handleCommit(next: WorkflowStepWithTranslations[]) {
		for (let i = 0; i < next.length; i++) {
			const step = next[i];
			if (step.stepOrder !== i + 1) {
				try {
					await patchStep(step, { step_order: i + 1 });
				} catch (e) {
					console.error(e);
					notifications.send({ priority: 'ERROR', message: 'Failed to reorder steps' });
					await invalidate('conversation:workflow');
					return;
				}
			}
		}
		await invalidate('conversation:workflow');
	}

	// --- Inline name edit ---
	function startEdit(step: WorkflowStepWithTranslations) {
		editingId = step.id;
		editValue = step.name;
	}
	async function commitEdit(step: WorkflowStepWithTranslations) {
		const name = editValue.trim();
		editingId = null;
		if (!name || name === step.name) return;
		// The step name is translated text content, saved via the translation endpoint
		// (keyed by its textContent id) — not a plain `name` field on the step.
		const textContentId = step.translations?.name?.textContent?.id;
		if (!textContentId) {
			notifications.send({ priority: 'ERROR', message: 'Failed to rename step' });
			return;
		}
		try {
			await saveTranslation(textContentId, conversation.primaryLocale ?? 'en', name);
			await invalidate('conversation:workflow');
		} catch (e) {
			console.error(e);
			notifications.send({ priority: 'ERROR', message: 'Failed to rename step' });
		}
	}

	// --- Delete ---
	async function deleteStep(step: WorkflowStepWithTranslations) {
		try {
			await apiClient.DeleteConversationWorkflowStep(undefined, {
				params: {
					conversation_id: conversation.id,
					workflow_id: workflow.id,
					workflow_step_id: step.id
				}
			});
			await invalidate('conversation:workflow');
			notifications.send({ priority: 'INFO', message: 'Step deleted' });
		} catch (e) {
			console.error(e);
			notifications.send({ priority: 'ERROR', message: 'Failed to delete step' });
		}
	}

	// --- Data protocol (only confidential/restricted are backed today) ---
	async function setProtocol(step: WorkflowStepWithTranslations, p: DataProtocol) {
		if (protocolOf(step) === p) return;
		try {
			await patchStep(step, { request_user_share_permission: boolFromProtocol(p) });
			await invalidate('conversation:workflow');
		} catch (e) {
			console.error(e);
			notifications.send({ priority: 'ERROR', message: 'Failed to update data protocol' });
		}
	}

	// --- Add step (palette) ---
	async function addStep(creationKey: string) {
		const tool_setup = {
			Polis: basic_polis_config,
			Learn: basic_learn_config,
			Survey: basic_survey_config,
			'Lived Experience': basic_lived_experience_config,
			'Elicitation Bot': basic_elicitation_bot_config(conversation),
			'Thinking Space': basic_thinking_space_config(),
			Prioritization: basic_prioritization_config
		}[creationKey];
		if (!tool_setup) return;

		const step_order =
			reorderedSteps.length > 0 ? Math.max(...reorderedSteps.map((s) => s.stepOrder)) + 1 : 1;
		adding = true;
		try {
			await apiClient.CreateConversationWorkflowStep(
				{
					name: defaultStepCreationParams[creationKey]?.name ?? `New ${creationKey} Step`,
					description:
						defaultStepCreationParams[creationKey]?.description ??
						`A new ${creationKey} Step`,
					is_offline: false,
					activation_rule: 'manual',
					step_order,
					tool_setup,
					required: true
				},
				{ params: { conversation_id: conversation.id, workflow_id: workflow.id } }
			);
			await invalidate('conversation:workflow');
			notifications.send({ priority: 'INFO', message: 'Step added' });
			// Scroll the board to reveal the newly appended step (rightmost).
			await tick();
			boardEl?.scrollTo({ left: boardEl.scrollWidth, behavior: 'smooth' });
		} catch (e) {
			console.error(e);
			notifications.send({ priority: 'ERROR', message: 'Failed to add step' });
		} finally {
			adding = false;
		}
	}

	let pageTitle = $derived(`Design ${conversation.title}`);
</script>

<svelte:head>
	<title>{pageTitle} - Comhairle Admin</title>
</svelte:head>

<!-- Full-bleed white surface (the design tab has no page padding). overflow-hidden also
	 makes this an independent scroll boundary so the wide board can't expand the
	 sidebar-inset past the viewport (the shell's inset is flex-1 without min-w-0); only
	 the board strip below scrolls. -->
<div class="bg-card flex min-h-0 w-full flex-1 overflow-hidden">
	<!-- Left tool palette (fixed-width, independently scrollable rail) -->
	<div class="w-48 shrink-0">
		<ToolPalette onAdd={addStep} {adding} />
	</div>

	<!-- Board -->
	<div class="flex min-h-0 min-w-0 flex-1 flex-col gap-2.5 overflow-hidden p-2.5">
		<!-- Toolbar -->
		<div class="flex shrink-0 flex-wrap items-center justify-end gap-1.5">
			<DropdownMenu.Root>
				<DropdownMenu.Trigger
					class="border-primary text-primary flex h-8 items-center gap-2 rounded-full border px-3 text-xs font-medium shadow-sm"
				>
					Template: {currentTemplateLabel}
					<ChevronDown class="size-3" />
				</DropdownMenu.Trigger>
				<DropdownMenu.Content align="end">
					{#each TEMPLATES as t (t.key)}
						<DropdownMenu.Item onSelect={() => chooseTemplate(t)}>
							{t.label}
						</DropdownMenu.Item>
					{/each}
				</DropdownMenu.Content>
			</DropdownMenu.Root>
			<button
				type="button"
				aria-pressed={showTime}
				onclick={() => (showTime = !showTime)}
				class="border-primary text-primary flex h-8 items-center gap-2 rounded-full border px-3 text-xs font-medium shadow-sm {showTime
					? 'bg-accent'
					: ''}"
			>
				<Clock class="size-3" /> Estimated time
			</button>
			<button
				type="button"
				aria-pressed={showProtocol}
				onclick={() => (showProtocol = !showProtocol)}
				class="border-primary text-primary flex h-8 items-center gap-2 rounded-full border px-3 text-xs font-medium shadow-sm {showProtocol
					? 'bg-accent'
					: ''}"
			>
				<Database class="size-3" /> Data protocol
			</button>
			<button
				type="button"
				aria-pressed={showLock}
				onclick={() => (showLock = !showLock)}
				class="border-primary text-primary flex h-8 items-center gap-2 rounded-full border px-3 text-xs font-medium shadow-sm {showLock
					? 'bg-accent'
					: ''}"
			>
				<LockOpen class="size-3" /> Unlocked
			</button>
		</div>

		<!-- Horizontal step board (the only scroller: x for the strip, y for tall cards) -->
		<div bind:this={boardEl} class="min-h-0 flex-1 overflow-auto">
			{#if reorderedSteps.length === 0}
				<div class="text-muted-foreground flex h-full items-center justify-center text-sm">
					Add a step from the palette to get started.
				</div>
			{:else}
				<DraggableList
					items={reorderedSteps}
					onReorder={handleReorder}
					onCommit={handleCommit}
					dragDisabled={editingId !== null}
					dropTargetStyle={{}}
					class="step-board flex flex-row items-start gap-2.5"
					flipDurationMs={200}
				>
					{#snippet children(step, index)}
						{@const type = stepType(step)}
						{@const meta = toolMeta(type)}
						{@const proto = DATA_PROTOCOLS.find((d) => d.value === protocolOf(step))}
						<div class="flex w-[472px] shrink-0 flex-col gap-2.5">
							<!-- Header bar -->
							<div class="group/head bg-card flex items-center gap-2 px-2 py-2">
								<div class="flex min-w-0 items-center gap-3">
									<div
										class="bg-primary text-primary-foreground flex size-6 shrink-0 items-center justify-center rounded-xl text-xs font-bold"
									>
										{index + 1}
									</div>
									<div class="flex min-w-0 flex-col">
										<div class="flex items-center gap-2">
											{#if editingId === step.id}
												<!-- svelte-ignore a11y_autofocus -->
												<input
													autofocus
													bind:value={editValue}
													onblur={() => commitEdit(step)}
													onkeydown={(e) => {
														if (e.key === 'Enter') commitEdit(step);
														if (e.key === 'Escape') editingId = null;
													}}
													class="border-input rounded border px-1 text-sm text-gray-900 outline-none"
												/>
											{:else}
												<button
													type="button"
													class="flex items-center gap-2 text-left"
													onclick={() => startEdit(step)}
												>
													<span class="truncate text-sm text-gray-900"
														>{step.name}</span
													>
													<Pencil
														class="text-muted-foreground size-3.5 shrink-0"
													/>
												</button>
											{/if}
										</div>
										<span class="text-primary text-xs font-medium">
											{meta?.displayName ?? type}
										</span>
									</div>
								</div>

								<!-- Pills revealed by toolbar toggles -->
								{#if showTime}
									<div
										class="bg-muted text-muted-foreground flex h-5 items-center gap-1 rounded-full px-2 text-xs font-medium"
									>
										<button
											type="button"
											aria-label="Decrease estimated time"
											onclick={() =>
												(timeOverrides[step.id] = Math.max(
													1,
													estMinutes(step) - 1
												))}
										>
											<Minus class="size-3" />
										</button>
										{estMinutes(step)}min
										<button
											type="button"
											aria-label="Increase estimated time"
											onclick={() =>
												(timeOverrides[step.id] = estMinutes(step) + 1)}
										>
											<Plus class="size-3" />
										</button>
									</div>
								{/if}
								{#if showProtocol}
									<DropdownMenu.Root>
										<DropdownMenu.Trigger
											class="bg-muted text-muted-foreground flex h-5 items-center gap-1 rounded-full px-2 text-xs font-medium"
										>
											<Database class="size-2.5" />
											{proto?.label}
											<ChevronDown class="size-2.5" />
										</DropdownMenu.Trigger>
										<DropdownMenu.Content>
											{#each DATA_PROTOCOLS as d (d.value)}
												<DropdownMenu.Item
													disabled={!d.enabled}
													onSelect={() => setProtocol(step, d.value)}
												>
													<span class="flex w-4 justify-center">
														{#if protocolOf(step) === d.value}<Check
																class="size-3"
															/>{/if}
													</span>
													<span class="flex flex-col">
														<span
															>{d.label}{!d.enabled
																? ' (soon)'
																: ''}</span
														>
														<span class="text-muted-foreground text-xs"
															>{d.blurb}</span
														>
													</span>
												</DropdownMenu.Item>
											{/each}
										</DropdownMenu.Content>
									</DropdownMenu.Root>
								{/if}
								{#if showLock}
									<div
										class="bg-muted text-muted-foreground flex h-5 items-center gap-1 rounded-full px-2 text-xs font-medium"
									>
										<Lock class="size-2.5" /> Unlocked
									</div>
								{/if}

								<div class="bg-border h-px flex-1"></div>

								<!-- Hover actions: delete + drag grip -->
								<div
									class="flex items-center gap-1 opacity-0 transition-opacity group-hover/head:opacity-100"
								>
									<button
										type="button"
										aria-label="Delete step"
										class="text-muted-foreground hover:text-destructive"
										onclick={() => deleteStep(step)}
									>
										<Trash2 class="size-4" />
									</button>
									<GripVertical
										class="text-muted-foreground size-4 cursor-grab"
									/>
								</div>
							</div>

							<!-- Description banner -->
							{#if !dismissed[step.id]?.desc}
								<div
									class="bg-accent border-primary/40 mx-2 flex items-center gap-2 rounded-lg border px-3 py-2"
								>
									<Lightbulb class="text-primary size-5 shrink-0" />
									<p class="text-primary flex-1 text-sm font-medium">
										{meta?.tagline ?? step.description}
									</p>
									<button
										type="button"
										aria-label="Dismiss"
										onclick={() =>
											(dismissed[step.id] = {
												...dismissed[step.id],
												desc: true
											})}
									>
										<X class="text-primary/70 size-4" />
									</button>
								</div>
							{/if}

							<!-- Data protocol banner (only when toggle on) -->
							{#if showProtocol && !dismissed[step.id]?.protocol}
								<div
									class="bg-accent border-primary/40 mx-2 flex items-center gap-2 rounded-lg border px-3 py-2"
								>
									<Database class="text-primary size-5 shrink-0" />
									<p class="text-primary flex-1 text-sm font-medium">
										{proto?.label}: {proto?.blurb}
									</p>
									<button
										type="button"
										aria-label="Dismiss"
										onclick={() =>
											(dismissed[step.id] = {
												...dismissed[step.id],
												protocol: true
											})}
									>
										<X class="text-primary/70 size-4" />
									</button>
								</div>
							{/if}

							<!-- Preview + Configure -->
							<div class="relative mx-2">
								<StepPreview {type} />
								<Button
									variant="link"
									href={`/admin/conversations/${conversation.id}/design/step/${step.id}`}
									class="absolute top-2 right-2"
								>
									Configure step
									<ArrowUpRight class="size-3" />
								</Button>
							</div>
						</div>
					{/snippet}
				</DraggableList>
			{/if}
		</div>
	</div>
</div>

<AlertDialog.Root bind:open={templateDialogOpen}>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title>Apply the “{pendingTemplate?.label}” template?</AlertDialog.Title>
			<AlertDialog.Description>
				This replaces the entire workflow. Every current step, its configuration and any
				collected data will be permanently deleted and cannot be recovered.
			</AlertDialog.Description>
		</AlertDialog.Header>
		<AlertDialog.Footer class="flex-col-reverse sm:flex-row">
			<AlertDialog.Cancel class="w-full sm:w-auto" disabled={applyingTemplate}>
				Cancel
			</AlertDialog.Cancel>
			<AlertDialog.Action
				class="bg-destructive hover:bg-destructive/90 w-full text-white sm:w-auto"
				disabled={applyingTemplate}
				onclick={(e) => {
					e.preventDefault();
					applyTemplate();
				}}
			>
				{#if applyingTemplate}
					<LoaderCircle class="mr-2 size-4 animate-spin" />
				{/if}
				Replace workflow
			</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>

<style>
	/* Drop target = a slim blue divider line where the card will land. svelte-dnd-action
	   hides the placeholder slot by default; we reveal it, collapse it to a thin bar and
	   hide the cloned card so neighbouring cards barely shift. Global because the
	   placeholder <li> lives inside DraggableList's markup, not this component's. */
	:global(.step-board [data-is-dnd-shadow-item-internal]) {
		visibility: visible !important;
		align-self: stretch;
		width: 3px !important;
		min-width: 3px !important;
		max-width: 3px !important;
		margin: 0 6px;
		border-radius: 9999px;
		background: var(--primary);
		overflow: hidden !important;
	}
	:global(.step-board [data-is-dnd-shadow-item-internal] > *) {
		display: none !important;
	}
</style>
