<script lang="ts">
	import { onMount } from 'svelte';
	import { Button } from '$lib/components/ui/button';
	import {
		Card,
		CardContent,
		CardHeader,
		CardTitle,
		CardDescription
	} from '$lib/components/ui/card';
	import { Plus, Trash2, GripVertical, Pencil, Minus } from 'lucide-svelte';
	import { notifications } from '$lib/notifications.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import DraggableList from '$lib/components/DraggableList.svelte';
	import QuestionEditorDialog from './QuestionEditorDialog.svelte';
	import type {
		ConversationWithTranslations,
		WorkflowStepWithTranslationsDto
	} from '@crownshy/api-client/api';
	import type { QuestionConfig } from './types';
	import { createTextContentSource } from '$lib/components/Translation/translationSource.svelte';
	import TranslatableField from '$lib/components/Translation/TranslatableField.svelte';
	import {
		resolveTranslatableJsonToTextContentIds,
		traverseTranslatableJsonAndCreateTranslations,
		type DraftTranslatableJsonField
	} from '$lib/components/Translation/translationUtils';
	import { invalidate } from '$app/navigation';

	type Props = {
		conversation: ConversationWithTranslations;
		workflowId: string;
		workflowStep: WorkflowStepWithTranslationsDto;
		isLive: boolean;
	};

	let { conversation, workflowId, workflowStep, isLive }: Props = $props();

	let toolConfig = $derived(isLive ? workflowStep.toolConfig : workflowStep.previewToolConfig);

	let topic = $derived<DraftTranslatableJsonField>(
		(toolConfig?.topic as DraftTranslatableJsonField) ?? { localized: '' }
	);
	let config = $state<{
		questions: QuestionConfig<DraftTranslatableJsonField>[];
		followUpRoundsCount: number;
	}>({
		questions: [],
		followUpRoundsCount: 2
	});
	let saving = $state(false);
	let questionEditorOpen = $state(false);
	let editingQuestionId = $state<string | null>(null);
	let editingQuestion = $derived<QuestionConfig<DraftTranslatableJsonField> | null>(
		(toolConfig?.root_questions as QuestionConfig<DraftTranslatableJsonField>[]).find(
			(question) => question.id === editingQuestionId
		) ?? null
	);

	const topicTransSource = $derived.by(() => {
		void toolConfig;
		void workflowStep;
		return createTextContentSource({
			getTranslation: () => topic?.translations,
			getPrimaryLocale: () => conversation.primaryLocale,
			getSupportedLanguages: () => conversation.supportedLanguages,
			getPrimaryFallback: () => topic?.localized ?? ''
		});
	});

	onMount(() => {
		const cfg = toolConfig as
			| {
					topic?: DraftTranslatableJsonField;
					root_questions?: QuestionConfig<DraftTranslatableJsonField>[];
					follow_up_rounds_count?: number;
			  }
			| undefined;
		config = {
			questions:
				cfg?.root_questions?.map((q) => ({
					id: q.id,
					text: q.text,
					intent: q.intent
				})) ?? [],
			followUpRoundsCount:
				typeof cfg?.follow_up_rounds_count === 'number'
					? Math.max(0, Math.min(5, cfg.follow_up_rounds_count))
					: 2
		};
	});

	function openCreateQuestion() {
		editingQuestionId = null;
		questionEditorOpen = true;
	}

	function openEditQuestion(q: QuestionConfig<DraftTranslatableJsonField>) {
		editingQuestionId = q.id;
		questionEditorOpen = true;
	}

	function handleSaveQuestion(q: QuestionConfig<DraftTranslatableJsonField>) {
		const exists = config.questions.some((x) => x.id === q.id);
		config.questions = exists
			? config.questions.map((x) => (x.id === q.id ? q : x))
			: [...config.questions, q];
	}

	function removeQuestion(id: string) {
		config.questions = config.questions.filter((q) => q.id !== id);
	}

	function bumpFollowUps(delta: -1 | 1) {
		config.followUpRoundsCount = Math.max(0, Math.min(5, config.followUpRoundsCount + delta));
	}

	async function saveAll() {
		if (topic.localized.trim().length < 3) {
			notifications.send({
				message: 'Topic must be at least 3 characters.',
				priority: 'ERROR'
			});
			return;
		}
		const missingIntent = config.questions.find((q) => !q.intent.localized.trim());
		if (missingIntent) {
			notifications.send({
				message: `Every question needs an intent. Add one to "${missingIntent.text.localized.trim() || 'Untitled question'}".`,
				priority: 'ERROR'
			});
			return;
		}
		saving = true;
		try {
			const payload = {
				type: 'thinkingspace' as const,
				topic,
				root_questions: config.questions,
				follow_up_rounds_count: config.followUpRoundsCount
			};
			const payloadWithNewlyCreatedTranslations =
				await traverseTranslatableJsonAndCreateTranslations(
					payload,
					conversation.primaryLocale
				);
			const resolvedToolConfig = resolveTranslatableJsonToTextContentIds(
				payloadWithNewlyCreatedTranslations
			);

			const update = isLive
				? { tool_config: resolvedToolConfig }
				: { preview_tool_config: resolvedToolConfig };
			await apiClient.UpdateConversationWorkflowStep(update, {
				params: {
					conversation_id: conversation.id,
					workflow_id: workflowId,
					workflow_step_id: workflowStep.id
				}
			});
			notifications.send({
				message: 'Thinking Space configuration saved.',
				priority: 'INFO'
			});
			await invalidate('conversation:workflow');
		} catch (e) {
			console.error(e);
			notifications.send({
				message: 'Failed to save configuration.',
				priority: 'ERROR'
			});
		} finally {
			saving = false;
		}
	}
</script>

<div class="space-y-6">
	<header class="space-y-2">
		<h1 class="text-2xl font-bold">Thinking Space</h1>
		<p class="text-muted-foreground text-sm">
			Set the topic, the questions participants will answer, and how many AI follow-ups
			they'll be asked per question.
		</p>
	</header>

	<Card>
		<CardHeader>
			<CardTitle>Topic</CardTitle>
			<CardDescription>
				The overall subject participants are reflecting on. Saved to the workflow step.
			</CardDescription>
		</CardHeader>
		<CardContent>
			<TranslatableField
				source={topicTransSource}
				primaryLocale={conversation.primaryLocale}
				supportedLanguages={conversation.supportedLanguages}
			/>
		</CardContent>
	</Card>

	<Card>
		<CardHeader class="flex flex-row items-start justify-between gap-4 space-y-0">
			<div>
				<CardTitle>Minimum follow-ups per main question</CardTitle>
				<CardDescription>
					Participants must answer at least this many AI-generated follow-ups before the
					option to move on appears. They can still answer more if they want. Set to 0 for
					no follow-ups.
				</CardDescription>
			</div>
			<div class="flex shrink-0 items-center gap-2">
				<Button
					variant="outline"
					size="icon"
					onclick={() => bumpFollowUps(-1)}
					disabled={config.followUpRoundsCount <= 0}
					aria-label="Decrease follow-up count"
				>
					<Minus class="size-4" />
				</Button>
				<span class="text-primary w-8 text-center text-xl font-semibold tabular-nums">
					{config.followUpRoundsCount}
				</span>
				<Button
					variant="outline"
					size="icon"
					onclick={() => bumpFollowUps(1)}
					disabled={config.followUpRoundsCount >= 5}
					aria-label="Increase follow-up count"
				>
					<Plus class="size-4" />
				</Button>
			</div>
		</CardHeader>
	</Card>

	<section class="space-y-4">
		<header class="flex items-start justify-between gap-4">
			<div>
				<h2 class="text-xl font-semibold">Main questions</h2>
				<p class="text-muted-foreground text-sm">
					Participants answer these one at a time. Drag to reorder.
				</p>
			</div>
			<Button class="shrink-0" variant="outline" onclick={openCreateQuestion}>
				<Plus class="mr-2 size-4" /> Add question
			</Button>
		</header>

		{#if config.questions.length === 0}
			<Card>
				<CardContent class="py-10 text-center">
					<p class="text-muted-foreground text-sm">
						No questions yet.
						<button
							type="button"
							class="text-primary font-medium underline-offset-4 hover:underline"
							onclick={openCreateQuestion}
						>
							Add the first one
						</button>
						to get started.
					</p>
				</CardContent>
			</Card>
		{:else}
			<DraggableList
				items={config.questions}
				onReorder={(next) => (config.questions = next)}
				dragDisabled={saving}
				class="space-y-3"
			>
				{#snippet children(q: QuestionConfig<DraftTranslatableJsonField>)}
					<Card class="bg-card">
						<CardContent class="flex items-start gap-3 p-4">
							<button
								type="button"
								aria-label="Drag to reorder"
								class="text-muted-foreground hover:text-foreground mt-1 shrink-0 cursor-grab active:cursor-grabbing"
							>
								<GripVertical class="size-4" />
							</button>
							<div class="min-w-0 flex-1 space-y-1">
								<p
									class="text-base leading-relaxed"
									class:text-foreground={q.text.localized.trim()}
									class:text-muted-foreground={!q.text.localized.trim()}
								>
									{q.text.localized.trim() || 'Untitled question'}
								</p>
								{#if q.intent.localized.trim()}
									<p class="text-muted-foreground line-clamp-2 text-xs">
										<span class="font-medium">Intent:</span>
										{q.intent.localized.trim()}
									</p>
								{:else}
									<p class="text-destructive text-xs">
										Intent missing — add one before saving.
									</p>
								{/if}
							</div>
							<div class="flex shrink-0 gap-2">
								<Button
									variant="outline"
									size="sm"
									onclick={() => openEditQuestion(q)}
								>
									<Pencil class="mr-1 size-3.5" /> Edit
								</Button>
								<Button
									variant="ghost"
									size="sm"
									class="text-destructive hover:text-destructive"
									onclick={() => removeQuestion(q.id)}
								>
									<Trash2 class="mr-1 size-3.5" /> Delete
								</Button>
							</div>
						</CardContent>
					</Card>
				{/snippet}
			</DraggableList>
		{/if}
	</section>

	<div class="flex justify-end">
		<Button onclick={saveAll} disabled={saving}>
			{saving ? 'Saving…' : 'Save configuration'}
		</Button>
	</div>
</div>

<QuestionEditorDialog
	open={questionEditorOpen}
	question={editingQuestion}
	onOpenChange={(o) => {
		questionEditorOpen = o;
		if (!o) editingQuestionId = null;
	}}
	onSave={handleSaveQuestion}
	primaryLocale={conversation.primaryLocale}
	supportedLanguages={conversation.supportedLanguages}
/>
