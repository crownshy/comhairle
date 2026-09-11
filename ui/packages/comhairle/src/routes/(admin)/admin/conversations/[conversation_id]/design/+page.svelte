<script lang="ts">
	import type { WorkflowStepWithTranslationsDto } from '@crownshy/api-client/api';
	import { tick } from 'svelte';
	import { invalidate } from '$app/navigation';
	import { apiClient } from '@crownshy/api-client/client';
	import { notifications } from '$lib/notifications.svelte';
	import { saveTranslation } from '$lib/components/Translation/translationUtils';
	import DraggableList from '$lib/components/DraggableList.svelte';
	import { Button } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { toolMeta, toolInfoUrl, type ToolType } from '$lib/tool_meta';
	import type { ConversationTemplate } from '$lib/conversation_templates';
	import TemplatePickerDialog from '$lib/components/TemplatePickerDialog.svelte';
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
	import { resolve } from '$app/paths';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { getAddStepDialogContext } from './context';
	import { key } from '$lib/utils/invalidationKey';
	import { DEFAULT_LOCALE } from '$lib/utils/constants';
	import { tryCatchAsync } from '$lib/utils/errorHandling';

	const { data } = $props();
	const { conversation } = $derived(data);

	let workflow = $derived(data.workflows[0]);

	// Writable derived: seeds from the loaded steps and re-seeds whenever they change (e.g.
	// after `invalidate`), while a drag/reorder can still assign to it locally in between.
	// Using $derived (not $state + $effect) means SSR renders the real order too, so a slow
	// client no longer flashes the empty state before hydration. (See CLAUDE.md.)
	let reorderedSteps = $state<WorkflowStepWithTranslationsDto[] | undefined>(undefined);

	// FIX: Change to use https://svelte.dev/docs/svelte/declaration-tags, once we update to Svelte 5.56
	$effect(() => {
		data.streamedWorkflows.then((result) => {
			if (result.ok !== null) {
				reorderedSteps = result.ok.current.steps;
			}
		});
	});

	// --- Ephemeral UI state ---
	let editingId = $state<string | null>(null);
	let editValue = $state('');
	let boardEl = $state<HTMLDivElement | null>(null);

	// --- Templates (re-seed the whole workflow) ---
	// The workflow doesn't persist which template it came from, and steps drift once
	// the admin edits them, so the trigger reads "Choose from templates" rather than
	// naming a current template it can't actually know.
	type TemplateChoice = { kind: 'blank' } | { kind: 'template'; template: ConversationTemplate };

	let templatePickerOpen = $state(false);
	let pendingChoice = $state<TemplateChoice | null>(null);
	let templateDialogOpen = $state(false);
	let applyingTemplate = $state(false);

	let pendingLabel = $derived(
		pendingChoice?.kind === 'template' ? pendingChoice.template.name : 'Blank'
	);

	/**
	 * Stage a choice. An empty workflow has nothing to destroy, so it skips straight
	 * past the replace-warning instead of asking about steps that don't exist.
	 */
	function chooseTemplate(choice: TemplateChoice) {
		pendingChoice = choice;
		templatePickerOpen = false;
		if (reorderedSteps?.length === 0) {
			applyTemplate();
		} else {
			templateDialogOpen = true;
		}
	}

	async function applyTemplate() {
		const choice = pendingChoice;
		if (!choice) return;
		if (reorderedSteps === undefined) return;

		applyingTemplate = true;

		const result = await tryCatchAsync(async (ok, err) => {
			const deleteSteps = await tryCatchAsync(() =>
				Promise.all(
					reorderedSteps!.map((s) =>
						apiClient.DeleteConversationWorkflowStep(undefined, {
							params: {
								conversation_id: conversation.id,
								workflow_id: workflow.id,
								workflow_step_id: s.id
							}
						})
					)
				)
			);

			if (deleteSteps.err !== null) {
				console.error(deleteSteps.err);
				notifications.send({
					priority: 'ERROR',
					message: 'Failed to delete steps, please try again'
				});
				throw err(deleteSteps.err);
			}

			// Templates may also declare creationEvents, but those belong to conversation
			// creation; re-templating an existing conversation only touches its steps.
			const steps = choice.kind === 'blank' ? [] : choice.template.creationSteps;

			const createConversationWorkflowSteps = await tryCatchAsync(() =>
				Promise.all(
					steps.map((step, i) =>
						apiClient.CreateConversationWorkflowStep(
							{
								name: step.name,
								description: step.description,
								is_offline: step.is_offline,
								activation_rule: 'manual',
								step_order: i + 1,
								// Template configs are structurally looser than the endpoint's
								// zod-inferred union (string vs literal `type`).
								tool_setup: step.tool_setup as Parameters<
									typeof apiClient.CreateConversationWorkflowStep
								>[0]['tool_setup'],
								required: step.required
							},
							{
								params: {
									conversation_id: conversation.id,
									workflow_id: workflow.id
								}
							}
						)
					)
				)
			);

			if (createConversationWorkflowSteps.err !== null) {
				console.error(createConversationWorkflowSteps.err);
				notifications.send({
					priority: 'ERROR',
					message: 'Failed to create steps, please try again'
				});
				throw err(createConversationWorkflowSteps.err);
			}

			return ok(true);
		});

		await invalidate(key('conversation/design/workflow'));
		applyingTemplate = false;
		pendingChoice = null;
		templateDialogOpen = false;

		if (result.err !== null) {
			notifications.send({ priority: 'ERROR', message: 'Failed to apply template' });
			return;
		}

		notifications.send({ priority: 'INFO', message: 'Template applied' });
	}

	function stepType(step: WorkflowStepWithTranslationsDto): ToolType | undefined {
		return step.previewToolConfig?.type ?? step.toolConfig?.type;
	}

	async function patchStep(step: WorkflowStepWithTranslationsDto, body: Record<string, unknown>) {
		await apiClient.UpdateConversationWorkflowStep(body, {
			params: {
				conversation_id: conversation.id,
				workflow_id: workflow.id,
				workflow_step_id: step.id
			}
		});
	}

	// --- Reorder (drag + buttons share the same commit) ---
	function handleReorder(next: WorkflowStepWithTranslationsDto[]) {
		reorderedSteps = next;
	}
	async function handleCommit(next: WorkflowStepWithTranslationsDto[]) {
		for (let i = 0; i < next.length; i++) {
			const step = next[i];
			if (step.stepOrder !== i + 1) {
				try {
					await patchStep(step, { step_order: i + 1 });
				} catch (e) {
					console.error(e);
					notifications.send({ priority: 'ERROR', message: 'Failed to reorder steps' });
					await invalidate(key('conversation/design/workflow'));
					return;
				}
			}
		}
		await invalidate(key('conversation/design/workflow'));
	}
	// Non-drag reorder: keyboard/click alternative for devices where drag is awkward.
	function moveStep(index: number, direction: -1 | 1) {
		if (!reorderedSteps) {
			return;
		}
		const next = moveItem(reorderedSteps, index, direction);
		if (next === reorderedSteps) return;
		reorderedSteps = next;
		handleCommit(next);
	}

	// --- Inline name edit ---
	function startEdit(step: WorkflowStepWithTranslationsDto) {
		editingId = step.id;
		editValue = step.name;
	}
	async function commitEdit(step: WorkflowStepWithTranslationsDto) {
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

		const result = await tryCatchAsync(() =>
			saveTranslation(textContentId, conversation.primaryLocale ?? DEFAULT_LOCALE, name)
		);

		if (result.err !== null) {
			console.error(result.err);
			notifications.send({ priority: 'ERROR', message: 'Failed to rename step' });
			return;
		}

		await invalidate(key('conversation/design/workflow'));
	}

	// --- Delete ---
	async function deleteStep(step: WorkflowStepWithTranslationsDto) {
		const result = await tryCatchAsync(() =>
			apiClient.DeleteConversationWorkflowStep(undefined, {
				params: {
					conversation_id: conversation.id,
					workflow_id: workflow.id,
					workflow_step_id: step.id
				}
			})
		);

		if (result.err !== null) {
			console.error(result.err);
			notifications.send({ priority: 'ERROR', message: 'Failed to delete step' });
			return;
		}

		await invalidate(key('conversation/design/workflow'));
		notifications.send({ priority: 'INFO', message: 'Step deleted' });
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
		if (!reorderedSteps?.some((s) => s.id === pending)) return;
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

	const addStepDialogContext = getAddStepDialogContext();
</script>

<svelte:head>
	<title>Design {conversation.title} - Comhairle Admin</title>
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
						Choose from templates
						<ChevronDown class="size-3" />
					</DropdownMenu.Trigger>
					<DropdownMenu.Content align="end" class="w-56">
						<DropdownMenu.Item onSelect={() => chooseTemplate({ kind: 'blank' })}>
							Start from blank
						</DropdownMenu.Item>
						<DropdownMenu.Item onSelect={() => (templatePickerOpen = true)}>
							Choose from templates
						</DropdownMenu.Item>
					</DropdownMenu.Content>
				</DropdownMenu.Root>
			</div>

			{#await data.streamedWorkflows}
				{#each [1, 2, 3] as i (i)}
					<!-- Brand-tinted shimmer so it reads against the card instead of gray-on-gray. -->
					<div
						class="border-border bg-card flex items-center gap-4 rounded-xl border p-4"
					>
						<Skeleton class="bg-primary/10 size-5 shrink-0 rounded" />
						<Skeleton class="bg-primary/20 size-6 shrink-0 rounded-lg" />
						<div class="flex min-w-0 flex-1 flex-col gap-2">
							<Skeleton class="bg-primary/15 h-5 w-48 max-w-full rounded-md" />
							<Skeleton class="bg-primary/10 h-4 w-24 rounded-md" />
						</div>
						<Skeleton class="bg-primary/10 size-9 shrink-0 rounded-md" />
					</div>
				{/each}
			{:then workflows}
				{#if workflows.err !== null}
					<span class="text-destructive">Could not load workflows, please try again</span>
				{:else if workflows.ok.current.steps.length === 0}
					<div
						class="border-border bg-card text-muted-foreground flex flex-col items-center justify-center gap-3 rounded-xl border border-dashed p-12 text-center"
					>
						<p class="text-sm">No steps yet. Add your first step to get started.</p>
						<Button onclick={() => addStepDialogContext.open()}>
							<Plus class="size-4" />
							Add step
						</Button>
					</div>
				{:else}
					{#if reorderedSteps !== undefined}
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
								<!-- Single-row card. The step name is a stretched link making the whole card navigate to Configure; the actions menu sits above it (z-10) so its clicks win. Pointer cursor + hover lift read as clickable. -->
								<div
									data-step-id={step.id}
									class={cn(
										'bg-card group hover:border-primary/50 border-border relative flex cursor-pointer items-center gap-4 rounded-xl border p-4 transition-all hover:shadow-md',
										highlightedStepId === step.id &&
											'ring-primary ring-offset-muted ring-2 ring-offset-2'
									)}
								>
									<!-- Drag handle. The whole card is draggable; this grip is the affordance that signals it. Sits above the stretched link. -->
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
												href={resolve(
													'/(admin)/admin/conversations/[conversation_id]/design/step/[step_id]',
													{
														conversation_id: conversation.id,
														step_id: step.id
													}
												)}
												class="text-foreground group-hover:text-primary truncate text-lg font-medium transition-colors outline-none after:absolute after:inset-0 after:content-['']"
											>
												{step.name}
											</a>
										{/if}
										<span class="text-primary text-sm font-medium">
											{meta?.displayName ?? type}
										</span>
									</div>

									<!-- Inline reorder arrows: a desktop-only quick path that reveals on card hover / keyboard focus. Duplicates the Move up/down menu items (kept for touch and the menu-driven flow). -->
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
											disabled={index === (reorderedSteps?.length ?? 1) - 1}
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
													disabled={index ===
														(reorderedSteps?.length ?? 1) - 1}
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
					{/if}

					<div>
						<Button variant="outline" onclick={() => addStepDialogContext.open()}>
							<Plus class="size-4" />
							Add step
						</Button>
					</div>
				{/if}
			{/await}
		</div>
	</div>
</div>

<TemplatePickerDialog
	bind:open={templatePickerOpen}
	submitting={applyingTemplate}
	description="Select a workflow template from the options below. Applying it replaces the steps this conversation has now."
	confirmLabel="Apply template"
	onConfirm={(template) => chooseTemplate({ kind: 'template', template })}
/>

<AlertDialog.Root bind:open={templateDialogOpen}>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title>Apply the “{pendingLabel}” template?</AlertDialog.Title>
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
