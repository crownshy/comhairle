<script lang="ts">
	import type { WorkflowStepWithTranslations } from '@crownshy/api-client/api';
	import { tick } from 'svelte';
	import { invalidate } from '$app/navigation';
	import { apiClient } from '@crownshy/api-client/client';
	import { notifications } from '$lib/notifications.svelte.js';
	import { saveTranslation } from '$lib/components/Translation/translationUtils';
	import DraggableList from '$lib/components/DraggableList.svelte';
	import StepListSkeleton from './StepListSkeleton.svelte';
	import { Button } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { toolMeta, toolInfoUrl } from '$lib/tool_meta';
	import { workflow_templates } from '$lib/workflow_templates.js';
	import { addStepDialog } from '$lib/stores/addStepDialog.svelte';
	import { newStepHighlight } from '$lib/stores/newStepHighlight.svelte';
	import { moveItem } from '$lib/utils/reorder';
	import { cn } from '$lib/utils';
	import {
		Pencil,
		Trash2,
		ChevronDown,
		ArrowUp,
		ArrowDown,
		MoreVertical,
		Plus,
		LoaderCircle,
		GripVertical,
		Info
	} from 'lucide-svelte';

	let { data } = $props();

	let conversation = $derived(data.conversation);
	let workflow = $derived(data.workflows[0]);
	let workflowSteps = $derived<WorkflowStepWithTranslations[] | undefined>(data.workflowSteps);

	// `undefined` = steps not loaded yet (show a skeleton); `[]` = genuinely no steps.
	let loadingSteps = $derived(workflowSteps === undefined);

	// Writable derived: seeds from the loaded steps and re-seeds whenever they change (e.g.
	// after `invalidate`), while a drag/reorder can still assign to it locally in between.
	// Using $derived (not $state + $effect) means SSR renders the real order too, so a slow
	// client no longer flashes the empty state before hydration. (See CLAUDE.md.)
	let reorderedSteps = $derived(
		workflowSteps ? [...workflowSteps].sort((a, b) => a.stepOrder - b.stepOrder) : []
	);

	// --- Ephemeral UI state ---
	let editingId = $state<string | null>(null);
	let editValue = $state('');
	let boardEl = $state<HTMLDivElement | null>(null);

	function stepUrl(step: WorkflowStepWithTranslations): string {
		return `/admin/conversations/${conversation.id}/design/step/${step.id}`;
	}

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
						// Template configs are structurally looser than the endpoint's
						// zod-inferred union (string vs literal `type`).
						tool_setup: st.tool_setup as Parameters<
							typeof apiClient.CreateConversationWorkflowStep
						>[0]['tool_setup'],
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

	function stepType(step: WorkflowStepWithTranslations): string | undefined {
		return step.previewToolConfig?.type ?? step.toolConfig?.type;
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

	// --- Reorder (drag + buttons share the same commit) ---
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
	// Non-drag reorder: keyboard/click alternative for devices where drag is awkward.
	function moveStep(index: number, direction: -1 | 1) {
		const next = moveItem(reorderedSteps, index, direction);
		if (next === reorderedSteps) return;
		reorderedSteps = next;
		handleCommit(next);
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

	// --- Scroll a step just created via the AddStepDialog into view and briefly highlight
	//     it, so it's clear which card is the one just added (the dialog is owned by the
	//     layout, so it hands the new id over via `newStepHighlight`). ---
	let highlightedStepId = $state<string | null>(null);
	// Plain handle so the highlight's own timer isn't torn down when this effect re-runs
	// after we clear `newStepHighlight` below.
	let highlightTimer: ReturnType<typeof setTimeout> | undefined;

	$effect(() => {
		const pending = newStepHighlight.id;
		if (!pending) return;
		// Wait until the invalidated steps actually include the new one.
		if (!reorderedSteps.some((s) => s.id === pending)) return;
		newStepHighlight.clear();
		highlightedStepId = pending;
		tick().then(() => {
			boardEl
				?.querySelector(`[data-step-id="${pending}"]`)
				?.scrollIntoView({ behavior: 'smooth', block: 'center' });
		});
		clearTimeout(highlightTimer);
		highlightTimer = setTimeout(() => {
			if (highlightedStepId === pending) highlightedStepId = null;
		}, 2500);
	});

	let pageTitle = $derived(`Design ${conversation.title}`);
</script>

<svelte:head>
	<title>{pageTitle} - Comhairle Admin</title>
</svelte:head>

<!-- Full-bleed surface; the design tab has no page padding. overflow-hidden makes this
	 an independent scroll boundary so the list can't push the sidebar-inset past the
	 viewport. Only the inner column scrolls. -->
<div class="bg-muted flex min-h-0 w-full flex-1 overflow-hidden">
	<div bind:this={boardEl} class="min-h-0 flex-1 overflow-auto">
		<!-- Same gutter column + top spacing as every other admin page; symmetric inset on mobile. -->
		<div class="px-gutter pt-page-top flex w-full max-w-5xl flex-col gap-4 pb-8">
			<!-- Toolbar -->
			<div class="flex shrink-0 flex-col items-center justify-between gap-2 sm:flex-row">
				<div class="flex flex-col">
					<h1 class="self-start text-2xl font-bold sm:self-auto">Process steps</h1>
					<p class="text-muted-foreground text-base">
						Design and configure your engagement, one step at a time.
					</p>
				</div>
				<DropdownMenu.Root>
					<DropdownMenu.Trigger
						class="bg-card border-primary text-primary flex h-8 shrink-0 items-center gap-2 self-end rounded-full border px-3 py-4 text-sm font-medium shadow-sm sm:self-auto"
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
			</div>

			{#if loadingSteps}
				<StepListSkeleton />
			{:else if reorderedSteps.length === 0}
				<div
					class="border-border bg-card text-muted-foreground flex flex-col items-center justify-center gap-3 rounded-xl border border-dashed p-12 text-center"
				>
					<p class="text-sm">No steps yet. Add your first step to get started.</p>
					<Button onclick={() => (addStepDialog.open = true)}>
						<Plus class="size-4" />
						Add step
					</Button>
				</div>
			{:else}
				<DraggableList
					items={reorderedSteps}
					onReorder={handleReorder}
					onCommit={handleCommit}
					dragDisabled={editingId !== null}
					dropTargetStyle={{}}
					class="flex flex-col gap-2.5"
					flipDurationMs={200}
				>
					{#snippet children(step, index)}
						{@const type = stepType(step)}
						{@const meta = toolMeta(type)}
						<!-- Single-row card. The step name is a stretched link making the whole
						     card navigate to Configure; the actions menu sits above it (z-10) so
						     its clicks win. Pointer cursor + hover lift read as clickable. -->
						<div
							data-step-id={step.id}
							class={cn(
								'bg-card group hover:border-primary/50 border-border relative flex cursor-pointer items-center gap-4 rounded-xl border p-4 transition-all hover:shadow-md',
								highlightedStepId === step.id &&
									'ring-primary ring-offset-muted ring-2 ring-offset-2'
							)}
						>
							<!-- Drag handle. The whole card is draggable; this grip is the
							     affordance that signals it. Sits above the stretched link. -->
							<GripVertical
								class="text-muted-foreground group-hover:text-foreground relative z-10 size-5 shrink-0 cursor-grab transition-colors"
								aria-hidden="true"
							/>

							<div
								class="bg-primary text-primary-foreground flex size-6 shrink-0 items-center justify-center rounded-lg text-sm font-bold"
							>
								{index + 1}
							</div>

							<div class="flex min-w-0 flex-1 flex-col">
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
										class="border-input relative z-10 rounded border px-1 text-base outline-none"
									/>
								{:else}
									<a
										href={stepUrl(step)}
										class="text-foreground group-hover:text-primary truncate text-lg font-medium transition-colors outline-none after:absolute after:inset-0 after:content-['']"
									>
										{step.name}
									</a>
								{/if}
								<span class="text-primary text-sm font-medium">
									{meta?.displayName ?? type}
								</span>
							</div>

							<!-- Inline reorder arrows: a desktop-only quick path that reveals on
							     card hover / keyboard focus. Duplicates the Move up/down menu
							     items (kept for touch and the menu-driven flow). -->
							<div
								class="relative z-10 hidden shrink-0 items-center gap-0.5 opacity-0 transition-opacity group-hover:opacity-100 focus-within:opacity-100 sm:flex"
							>
								<button
									type="button"
									aria-label="Move step up"
									disabled={index === 0}
									onclick={() => moveStep(index, -1)}
									class="text-muted-foreground hover:text-foreground hover:bg-accent flex size-8 items-center justify-center rounded-md transition-colors disabled:pointer-events-none disabled:opacity-30"
								>
									<ArrowUp class="size-4" />
								</button>
								<button
									type="button"
									aria-label="Move step down"
									disabled={index === reorderedSteps.length - 1}
									onclick={() => moveStep(index, 1)}
									class="text-muted-foreground hover:text-foreground hover:bg-accent flex size-8 items-center justify-center rounded-md transition-colors disabled:pointer-events-none disabled:opacity-30"
								>
									<ArrowDown class="size-4" />
								</button>
							</div>

							<!-- Actions menu (above the stretched link). All step actions live
								     here to keep the card uncluttered; the whole card is draggable,
								     and Move up/down provide a no-drag reorder path. -->
							<div class="relative z-10 shrink-0">
								<DropdownMenu.Root>
									<DropdownMenu.Trigger
										aria-label="Step actions"
										class="text-muted-foreground hover:text-foreground hover:bg-accent flex size-9 items-center justify-center rounded-md transition-colors"
									>
										<MoreVertical class="size-5" />
									</DropdownMenu.Trigger>
									<DropdownMenu.Content align="end">
										<DropdownMenu.Item
											disabled={index === 0}
											onSelect={() => moveStep(index, -1)}
										>
											<ArrowUp class="size-4" /> Move up
										</DropdownMenu.Item>
										<DropdownMenu.Item
											disabled={index === reorderedSteps.length - 1}
											onSelect={() => moveStep(index, 1)}
										>
											<ArrowDown class="size-4" /> Move down
										</DropdownMenu.Item>
										<DropdownMenu.Item onSelect={() => startEdit(step)}>
											<Pencil class="size-4" /> Rename
										</DropdownMenu.Item>
										<DropdownMenu.Separator />
										<DropdownMenu.Item>
											<a
												href={toolInfoUrl(type)}
												target="_blank"
												class="flex w-full items-center gap-2"
											>
												<Info class="size-4" /> Learn more
											</a>
										</DropdownMenu.Item>
										<DropdownMenu.Separator />
										<DropdownMenu.Item
											class="text-destructive"
											onSelect={() => deleteStep(step)}
										>
											<Trash2 class="size-4" /> Delete
										</DropdownMenu.Item>
									</DropdownMenu.Content>
								</DropdownMenu.Root>
							</div>
						</div>
					{/snippet}
				</DraggableList>

				<div>
					<Button variant="outline" onclick={() => (addStepDialog.open = true)}>
						<Plus class="size-4" />
						Add step
					</Button>
				</div>
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
