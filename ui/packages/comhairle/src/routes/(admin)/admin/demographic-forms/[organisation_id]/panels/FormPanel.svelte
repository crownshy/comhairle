<script lang="ts">
	import DraggableList from '$lib/components/DraggableList.svelte';
	import type { QuestionLatestDto } from '@crownshy/api-client/api';
	import { GripVertical, X } from 'lucide-svelte';
	import { SPECIAL_CATEGORY_TAG, type TagName } from '../../demographicModel';
	import Demographics from '$lib/interfaces/Demographics';
	import TagBadge from '../TagBadge.svelte';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';

	type Props = {
		questions: QuestionLatestDto[];
		selectedQuestionId: QuestionLatestDto['id'] | undefined;
		organisationId: string;
	};

	let {
		questions = $bindable(),
		selectedQuestionId = $bindable(),
		organisationId
	}: Props = $props();

	let deleteDialogOpen = $state<boolean>(false);
</script>

{#snippet AddQuestionButtons()}
	<div class="mt-3 flex items-center justify-center gap-x-2">
		<Button>Question bank</Button>
		<Button
			onclick={() => {
				const newQuestion = Demographics.newQuestion(organisationId);
				questions.push(newQuestion);
				selectedQuestionId = newQuestion.id;
			}}
		>
			New
		</Button>
	</div>
{/snippet}

<section>
	<h3 class="mb-2 text-sm font-semibold">Questions</h3>
	{#if questions.length === 0}
		<div
			class="border-border mt-3 flex flex-col items-center gap-y-3 rounded-xl border-2 border-dashed p-5"
		>
			<span class="text-muted-foreground text-sm font-medium">No questions yet</span>
			{@render AddQuestionButtons()}
		</div>
	{:else}
		<DraggableList
			items={questions}
			onReorder={(next) => void (questions = next)}
			class="flex flex-col gap-2"
		>
			{#snippet children(question, index)}
				<div
					class="border-border bg-card flex items-center justify-between rounded-xl border-2 py-3 pr-1 pl-3"
					class:border-primary={question.id === selectedQuestionId}
					aria-selected={question.id === selectedQuestionId}
				>
					<GripVertical class="text-muted-foreground/70 size-4" />
					<button
						type="button"
						class="flex items-center"
						onclickcapture={() => void (selectedQuestionId = question.id)}
					>
						<div class="flex flex-col items-start">
							<div class="flex items-center justify-start gap-1">
								<div class="text-muted-foreground ml-2 text-sm font-semibold">
									{index + 1}.
								</div>
								<h4
									class="w-[20ch] truncate text-left text-sm font-medium"
									title={question.text}
								>
									{question.text}
									{#if question.required && question.text.length < 23}
										<span
											class="text-destructive font-semibold"
											title="Required"
										>
											*
										</span>
										<span class="sr-only">Required</span>
									{/if}
								</h4>
								{#if question.required && question.text.length >= 24}
									<span class="text-destructive font-semibold" title="Required">
										*
									</span>
									<span class="sr-only">Required</span>
								{/if}
							</div>
							<div class="mt-0.5 ml-3">
								{#each question.tags as tag (tag.id)}
									<TagBadge tag={tag.name as TagName} />
								{/each}
								{#if question.specialCategory}
									<TagBadge tag={SPECIAL_CATEGORY_TAG} />
								{/if}
								{#if Demographics.isDefaultQuestion(question)}
									<Badge variant="secondary" class="text-xs">Default</Badge>
								{/if}
							</div>
						</div>
						<Button
							variant="ghost"
							class="text-muted-foreground size-6"
							title="Delete question"
							aria-label="Delete {question.text}"
							onclick={() => void (deleteDialogOpen = true)}
						>
							<X class="size-4" />
						</Button>
					</button>
				</div>
			{/snippet}
		</DraggableList>
		<div>
			{@render AddQuestionButtons()}
		</div>
	{/if}
</section>

<AlertDialog.Root bind:open={deleteDialogOpen}>
	<AlertDialog.Content>
		<AlertDialog.Header>
			Are you sure you want to delete "{questions.find(
				(question) => question.id === selectedQuestionId
			)?.text}"
		</AlertDialog.Header>
		<AlertDialog.Footer>
			<AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
			<AlertDialog.Action
				onclick={() => {
					const index = questions.findIndex(
						(question) => question.id === selectedQuestionId
					);
					if (index < 0) {
						return;
					}
					// Remove currently selected question
					questions.splice(index, 1);

					// Focus on the element above
					if (questions.length > 0) {
						selectedQuestionId = questions[Math.max(0, index - 1)].id;
					}

					deleteDialogOpen = false;
				}}
			>
				Delete
			</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>
