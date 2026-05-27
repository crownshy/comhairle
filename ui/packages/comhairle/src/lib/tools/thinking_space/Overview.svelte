<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Card, CardContent } from '$lib/components/ui/card';
	import { Pencil, CornerDownRight } from 'lucide-svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import { notifications } from '$lib/notifications.svelte';
	import type { QuestionConfig, QuestionAnswers } from './types';

	type Props = {
		topic: string;
		questions: QuestionConfig[];
		answers: QuestionAnswers[];
		onDone?: () => void;
	};

	let { topic, questions, answers, onDone }: Props = $props();

	// Editable local copy — edits are persisted to the backend on save.
	let items = $state<QuestionAnswers[]>(
		answers.map((qa) => ({ ...qa, followUps: qa.followUps.map((f) => ({ ...f })) }))
	);

	let editingId = $state<string | null>(null);
	let draft = $state('');
	let saving = $state(false);

	function startEdit(id: string, current: string) {
		editingId = id;
		draft = current;
	}

	function cancelEdit() {
		editingId = null;
		draft = '';
	}

	async function saveEdit(id: string) {
		const value = draft.trim();
		if (!value || saving) return;
		saving = true;
		try {
			await apiClient.UpdateThinkingSpaceAnswer(
				{ answer: value },
				{ params: { answer_id: id } }
			);
			items = items.map((qa) => ({
				...qa,
				rootAnswer: qa.rootAnswerId === id ? value : qa.rootAnswer,
				followUps: qa.followUps.map((f) => (f.id === id ? { ...f, answer: value } : f))
			}));
			editingId = null;
			draft = '';
		} catch (e) {
			console.error(e);
			notifications.send({
				message: 'Could not save your edit. Please try again.',
				priority: 'ERROR'
			});
		} finally {
			saving = false;
		}
	}
</script>

<div class="mx-auto w-full max-w-2xl px-6 py-10">
	<header class="mb-8 text-center">
		{#if topic}
			<p class="text-muted-foreground text-xs font-medium tracking-wide uppercase">
				{topic}
			</p>
		{/if}
		<h2 class="text-foreground mt-1 text-3xl font-semibold tracking-tight">Your responses</h2>
		<p class="text-muted-foreground mx-auto mt-2 max-w-md text-sm leading-relaxed">
			Review what you shared. Edit anything that doesn't sound right.
		</p>
	</header>

	<div class="space-y-8">
		{#each questions as q (q.id)}
			{@const item = items.find((x) => x.questionId === q.id)}
			{#if item}
				<section class="space-y-3">
					<h3 class="text-foreground text-lg font-semibold">{q.text}</h3>
					{@render answerBlock(item.rootAnswerId, item.rootAnswer)}
					{#each item.followUps as fu (fu.id)}
						<div class="space-y-2 pl-4">
							<p
								class="text-muted-foreground flex items-center gap-1.5 text-sm leading-snug italic"
							>
								<CornerDownRight class="size-3.5 shrink-0" />
								{fu.question}
							</p>
							{@render answerBlock(fu.id, fu.answer)}
						</div>
					{/each}
				</section>
			{/if}
		{/each}
	</div>

	{#if onDone}
		<div class="mt-10">
			<Button size="lg" class="w-full" onclick={onDone}>Continue</Button>
		</div>
	{/if}
</div>

{#snippet answerBlock(id: string | null, text: string)}
	<Card>
		<CardContent class="space-y-3 px-4 py-3">
			{#if id && editingId === id}
				<Textarea bind:value={draft} rows={3} class="text-sm" />
				<div class="flex gap-2">
					<Button
						size="sm"
						onclick={() => saveEdit(id)}
						disabled={!draft.trim() || saving}
					>
						{saving ? 'Saving…' : 'Save'}
					</Button>
					<Button size="sm" variant="ghost" onclick={cancelEdit} disabled={saving}>
						Cancel
					</Button>
				</div>
			{:else}
				<div class="flex items-start justify-between gap-3">
					<p class="text-foreground text-sm leading-relaxed whitespace-pre-wrap">
						{text}
					</p>
					{#if id}
						<Button
							size="sm"
							variant="outline"
							class="shrink-0"
							onclick={() => startEdit(id, text)}
						>
							<Pencil class="size-3.5" />
							Edit
						</Button>
					{/if}
				</div>
			{/if}
		</CardContent>
	</Card>
{/snippet}
