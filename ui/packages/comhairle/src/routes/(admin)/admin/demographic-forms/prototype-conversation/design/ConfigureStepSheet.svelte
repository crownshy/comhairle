<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import * as Sheet from '$lib/components/ui/sheet';
	import { Switch } from '$lib/components/ui/switch';
	import {
		askedCount,
		isOverridden,
		overrideCount,
		requiredCount,
		resetToForm,
		setAsk,
		versionLabel,
		type DemographicStep
	} from '../../conversationStep';
	import type { DemographicForm, DemographicQuestion } from '../../demographicPrototypeData';
	import TagBadge from '../../TagBadge.svelte';

	type Props = {
		step: DemographicStep;
		form: DemographicForm;
		questions: DemographicQuestion[];
		onSave: (step: DemographicStep) => void;
		onClose: () => void;
	};

	let { step, form, questions, onSave, onClose }: Props = $props();

	// svelte-ignore state_referenced_locally
	let draft = $state<DemographicStep>(structuredClone($state.snapshot(step)));

	const overrides = $derived(overrideCount(draft, form));

	function questionFor(id: string) {
		return questions.find((question) => question.id === id);
	}

	function formRequired(id: string) {
		return form.questions.find((question) => question.questionId === id)?.required ?? false;
	}
</script>

<Sheet.Root open onOpenChange={(open) => !open && onClose()}>
	<Sheet.Content
		side="right"
		class="flex w-full flex-col gap-0 p-0 sm:max-w-2xl [&>button.absolute]:hidden"
	>
		<Sheet.Header class="border-border border-b p-6">
			<Sheet.Title class="text-2xl font-semibold">Configure {draft.name}</Sheet.Title>
			<Sheet.Description class="text-base">
				Questions come from {form.name}. Choose which ones this step asks and which it
				requires.
			</Sheet.Description>
		</Sheet.Header>

		<div class="flex min-h-0 flex-1 flex-col gap-4 overflow-y-auto p-6">
			<div class="bg-muted flex flex-wrap items-center justify-between gap-3 rounded-xl p-4">
				<div class="flex flex-col">
					<span class="text-base font-medium">{form.name}</span>
					<span class="text-muted-foreground text-base">
						{versionLabel(draft, form)}
					</span>
				</div>
				<span class="text-base">
					{askedCount(draft)} of {draft.questions.length} asked · {requiredCount(draft)} required
				</span>
			</div>

			<div class="flex items-center justify-between gap-3">
				<p class="text-muted-foreground text-base">
					{#if overrides === 0}
						Everything here matches the form.
					{:else}
						{overrides}
						{overrides === 1 ? 'question differs' : 'questions differ'} from the form.
					{/if}
				</p>
				<Button
					variant="outline"
					disabled={overrides === 0}
					onclick={() => (draft = resetToForm(draft, form))}
				>
					Reset to form defaults
				</Button>
			</div>

			<ul class="border-border divide-y overflow-hidden rounded-xl border">
				<li
					class="bg-muted text-muted-foreground grid grid-cols-[1fr_5rem_6rem] items-center gap-3 px-4 py-2 text-sm font-medium"
				>
					<span>Question</span>
					<span>Ask</span>
					<span>Required</span>
				</li>
				{#each draft.questions as stepQuestion, index (stepQuestion.questionId)}
					{@const question = questionFor(stepQuestion.questionId)}
					{#if question}
						<li class="grid grid-cols-[1fr_5rem_6rem] items-center gap-3 px-4 py-3">
							<div class="flex min-w-0 flex-col gap-1">
								<span
									class="text-base font-medium {stepQuestion.ask
										? ''
										: 'text-muted-foreground line-through'}"
								>
									{question.text}
								</span>
								<span class="flex flex-wrap items-center gap-1">
									{#each question.tags as tag (tag)}
										<TagBadge {tag} />
									{/each}
									{#if question.specialCategory}
										<TagBadge tag="Special category" />
									{/if}
									{#if isOverridden(stepQuestion, form)}
										<Badge variant="primary">Overridden</Badge>
									{:else}
										<span class="text-muted-foreground text-sm">
											From form{formRequired(stepQuestion.questionId)
												? ', required'
												: ''}
										</span>
									{/if}
								</span>
							</div>
							<Switch
								checked={stepQuestion.ask}
								onCheckedChange={(checked) =>
									(draft.questions[index] = setAsk(stepQuestion, checked))}
								aria-label={`Ask ${question.text}`}
							/>
							<Switch
								checked={stepQuestion.required}
								disabled={!stepQuestion.ask}
								onCheckedChange={(checked) =>
									(draft.questions[index] = {
										...stepQuestion,
										required: checked
									})}
								aria-label={`Require ${question.text}`}
							/>
						</li>
					{/if}
				{/each}
			</ul>

			<p class="text-muted-foreground text-base">
				Required asks for an answer, but participants can always choose Prefer not to say.
				{#if draft.policy === 'latest'}
					Questions added to the form later are asked here too, using the form's own
					required setting.
				{/if}
			</p>
		</div>

		<Sheet.Footer class="border-border flex-row justify-end gap-2 border-t p-4">
			<Button variant="outline" onclick={onClose}>Cancel</Button>
			<Button onclick={() => onSave($state.snapshot(draft))}>Save step</Button>
		</Sheet.Footer>
	</Sheet.Content>
</Sheet.Root>
