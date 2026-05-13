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
	import { Plus, Trash2, ArrowUp, ArrowDown, Minus } from 'lucide-svelte';
	import { notifications } from '$lib/notifications.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import type { WorkflowStepWithTranslations } from '@crownshy/api-client/api';
	import { loadConfig, saveConfig, newQuestionId } from './config';
	import type { QuestionConfig, ThinkingSpaceConfig } from './types';

	type Props = {
		conversationId: string;
		workflowId: string;
		workflowStep: WorkflowStepWithTranslations;
		isLive: boolean;
	};

	let { conversationId, workflowId, workflowStep, isLive }: Props = $props();

	let toolConfig = $derived(isLive ? workflowStep.toolConfig : workflowStep.previewToolConfig);

	let topic = $state('');
	let config = $state<ThinkingSpaceConfig>({ questions: [], followUpCount: 2 });
	let saving = $state(false);

	onMount(() => {
		topic = (toolConfig as { topic?: string })?.topic ?? '';
		const loaded = loadConfig(workflowStep.id);
		// Ensure at least one question slot so the UI never shows an empty list
		if (loaded.questions.length === 0) {
			loaded.questions = [{ id: newQuestionId(), text: '' }];
		}
		config = loaded;
	});

	function persistLocal() {
		saveConfig(workflowStep.id, $state.snapshot(config));
	}

	function addQuestion() {
		config.questions = [...config.questions, { id: newQuestionId(), text: '' }];
		persistLocal();
	}

	function removeQuestion(id: string) {
		if (config.questions.length <= 1) return;
		config.questions = config.questions.filter((q) => q.id !== id);
		persistLocal();
	}

	function move(id: string, delta: -1 | 1) {
		const idx = config.questions.findIndex((q) => q.id === id);
		const target = idx + delta;
		if (idx < 0 || target < 0 || target >= config.questions.length) return;
		const next = [...config.questions];
		[next[idx], next[target]] = [next[target], next[idx]];
		config.questions = next;
		persistLocal();
	}

	function bumpFollowUps(delta: -1 | 1) {
		config.followUpCount = Math.max(0, Math.min(5, config.followUpCount + delta));
		persistLocal();
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
			const update = isLive
				? {
						tool_config: {
							...workflowStep.toolConfig,
							type: 'elicitationbot' as const,
							topic
						}
					}
				: {
						preview_tool_config: {
							...workflowStep.previewToolConfig,
							type: 'elicitationbot' as const,
							topic
						}
					};
			await apiClient.UpdateConversationElicitationBotWorkflowStep(update, {
				params: {
					conversation_id: conversationId,
					workflow_id: workflowId,
					workflow_step_id: workflowStep.id
				}
			});
			persistLocal();
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
				<CardTitle>Follow-up questions per main question</CardTitle>
				<CardDescription>
					How many AI-generated follow-ups participants answer before moving on. Set to 0
					for no follow-ups.
				</CardDescription>
			</div>
			<div class="flex shrink-0 items-center gap-2">
				<Button
					variant="outline"
					size="icon"
					onclick={() => bumpFollowUps(-1)}
					disabled={config.followUpCount <= 0}
					aria-label="Decrease follow-up count"
				>
					<Minus class="size-4" />
				</Button>
				<span class="text-primary w-8 text-center text-xl font-semibold tabular-nums">
					{config.followUpCount}
				</span>
				<Button
					variant="outline"
					size="icon"
					onclick={() => bumpFollowUps(1)}
					disabled={config.followUpCount >= 5}
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
			{#each config.questions as q, i (q.id)}
				<div class="border-border bg-card flex items-start gap-3 rounded-lg border p-3">
					<div
						class="bg-primary/10 text-primary mt-1 flex size-7 shrink-0 items-center justify-center rounded-full text-sm font-semibold"
					>
						{i + 1}
					</div>
					<div class="flex-1">
						<Textarea
							bind:value={config.questions[i].text}
							onblur={persistLocal}
							placeholder="Write your question…"
							rows={2}
							class="resize-none"
						/>
					</div>
					<div class="flex flex-col gap-1">
						<Button
							variant="ghost"
							size="icon"
							onclick={() => move(q.id, -1)}
							disabled={i === 0}
							aria-label="Move up"
						>
							<ArrowUp class="size-4" />
						</Button>
						<Button
							variant="ghost"
							size="icon"
							onclick={() => move(q.id, 1)}
							disabled={i === config.questions.length - 1}
							aria-label="Move down"
						>
							<ArrowDown class="size-4" />
						</Button>
						<Button
							variant="ghost"
							size="icon"
							onclick={() => removeQuestion(q.id)}
							disabled={config.questions.length <= 1}
							aria-label="Remove question"
						>
							<Trash2 class="text-destructive size-4" />
						</Button>
					</div>
				</div>
			{/each}

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

	<p class="text-muted-foreground text-xs">
		Questions and follow-up count are stored locally in your browser for this prototype. Only
		the topic is persisted server-side today. See
		<code class="bg-muted rounded px-1 py-0.5">THINKING_SPACE_TODO.md</code> for the planned backend
		wiring.
	</p>
</div>
