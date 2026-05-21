<script lang="ts">
	import { onMount } from 'svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Textarea } from '$lib/components/ui/textarea';
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
	import type { WorkflowStepWithTranslations } from '@crownshy/api-client/api';
	import type { QuestionConfig } from './types';

	type Props = {
		conversationId: string;
		workflowId: string;
		workflowStep: WorkflowStepWithTranslations;
		isLive: boolean;
	};

	let { conversationId, workflowId, workflowStep, isLive }: Props = $props();

	let toolConfig = $derived(isLive ? workflowStep.toolConfig : workflowStep.previewToolConfig);

	let topic = $state('');
	let config = $state<{ questions: QuestionConfig[]; followUpRoundsCount: number }>({
		questions: [],
		followUpRoundsCount: 2
	});
	let saving = $state(false);
	let editingId = $state<string | null>(null);

	onMount(() => {
		const cfg = toolConfig as
			| { topic?: string; root_questions?: QuestionConfig[]; follow_up_rounds_count?: number }
			| undefined;
		topic = cfg?.topic ?? '';
		// Ensure at least one question slot so the UI never shows an empty list
		const questions =
			cfg?.root_questions && cfg.root_questions.length > 0
				? cfg.root_questions.map((q) => ({ id: q.id, text: q.text }))
				: [{ id: crypto.randomUUID(), text: '' }];
		config = {
			questions,
			followUpRoundsCount:
				typeof cfg?.follow_up_rounds_count === 'number'
					? Math.max(0, Math.min(5, cfg.follow_up_rounds_count))
					: 2
		};
		// Open the first question for editing if it's still blank
		if (questions.length === 1 && !questions[0].text.trim()) {
			editingId = questions[0].id;
		}
	});

	function addQuestion() {
		const q = { id: crypto.randomUUID(), text: '' };
		config.questions = [...config.questions, q];
		editingId = q.id;
	}

	function removeQuestion(id: string) {
		if (config.questions.length <= 1) return;
		config.questions = config.questions.filter((q) => q.id !== id);
		if (editingId === id) editingId = null;
	}

	function bumpFollowUps(delta: -1 | 1) {
		config.followUpRoundsCount = Math.max(0, Math.min(5, config.followUpRoundsCount + delta));
	}

	async function saveAll() {
		if (topic.trim().length < 3) {
			notifications.send({
				message: 'Topic must be at least 3 characters.',
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
			const update = isLive ? { tool_config: payload } : { preview_tool_config: payload };
			await apiClient.UpdateConversationWorkflowStep(update, {
				params: {
					conversation_id: conversationId,
					workflow_id: workflowId,
					workflow_step_id: workflowStep.id
				}
			});
			notifications.send({
				message: 'Thinking Space configuration saved.',
				priority: 'INFO'
			});
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
			<Input bind:value={topic} placeholder="e.g. Farmers & Agriculture in Scotland" />
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

	<Card>
		<CardHeader>
			<CardTitle>Main questions</CardTitle>
			<CardDescription>
				Participants answer these one at a time. Reorder or remove any you don't need.
			</CardDescription>
		</CardHeader>
		<CardContent class="space-y-3">
			<DraggableList
				items={config.questions}
				onReorder={(next) => (config.questions = next)}
				dragDisabled={editingId !== null || saving}
				class="space-y-3"
			>
				{#snippet children(q: QuestionConfig, i: number)}
					<Card class="bg-card">
						<CardContent class="flex items-start gap-3 p-4">
							<button
								type="button"
								aria-label="Drag to reorder"
								class="text-muted-foreground hover:text-foreground mt-1 shrink-0 cursor-grab active:cursor-grabbing"
							>
								<GripVertical class="size-4" />
							</button>
							<div class="min-w-0 flex-1">
								{#if editingId === q.id}
									<Textarea
										bind:value={config.questions[i].text}
										placeholder="Write your question…"
										rows={2}
										class="resize-none"
									/>
									<div class="mt-2 flex justify-end">
										<Button size="sm" onclick={() => (editingId = null)}>
											Done
										</Button>
									</div>
								{:else}
									<p
										class="text-base leading-relaxed"
										class:text-foreground={q.text.trim()}
										class:text-muted-foreground={!q.text.trim()}
									>
										{q.text.trim() || 'Untitled question'}
									</p>
								{/if}
							</div>
							{#if editingId !== q.id}
								<div class="flex shrink-0 gap-2">
									<Button
										variant="outline"
										size="sm"
										onclick={() => (editingId = q.id)}
									>
										<Pencil class="mr-1 size-3.5" /> Edit
									</Button>
									<Button
										variant="ghost"
										size="sm"
										class="text-destructive hover:text-destructive"
										onclick={() => removeQuestion(q.id)}
										disabled={config.questions.length <= 1}
									>
										<Trash2 class="mr-1 size-3.5" /> Delete
									</Button>
								</div>
							{/if}
						</CardContent>
					</Card>
				{/snippet}
			</DraggableList>

			<Button variant="outline" class="w-full" onclick={addQuestion}>
				<Plus class="size-4" />
				Add question
			</Button>
		</CardContent>
	</Card>

	<div class="flex justify-end">
		<Button onclick={saveAll} disabled={saving}>
			{saving ? 'Saving…' : 'Save configuration'}
		</Button>
	</div>
</div>
