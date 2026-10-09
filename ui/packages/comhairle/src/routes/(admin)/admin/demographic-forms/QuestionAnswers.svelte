<script lang="ts">
	import { Check } from 'lucide-svelte';
	import { Input } from '$lib/components/ui/input';
	import { Textarea } from '$lib/components/ui/textarea';
	import {
		isChoiceDisabled,
		OTHER_CHOICE,
		PREFER_NOT_TO_SAY,
		toggleChoice,
		emptyAnswer,
		type Answer
	} from './demographicAnswers';
	import {
		defaultPlaceholder,
		isChoiceKind,
		selectionHint,
		type DemographicQuestion
	} from './demographicPrototypeData';

	type Props = {
		question: Pick<
			DemographicQuestion,
			'kind' | 'options' | 'allowOther' | 'preferNotToSay' | 'placeholder' | 'maxSelections'
		>;
		answer: Answer;
		onAnswer: (next: Answer) => void;
	};

	let { question, answer, onAnswer }: Props = $props();

	const choices = $derived([
		...question.options.filter((option) => option.trim() !== ''),
		...(question.allowOther ? [OTHER_CHOICE] : []),
		...(question.preferNotToSay ? [PREFER_NOT_TO_SAY] : [])
	]);
	const declined = $derived(answer.selected.includes(PREFER_NOT_TO_SAY));
	const placeholder = $derived(question.placeholder || defaultPlaceholder(question.kind));

	function pick(choice: string) {
		onAnswer(toggleChoice(answer, choice, question.kind, question.maxSelections));
	}

	function toggleDeclined() {
		onAnswer(declined ? emptyAnswer() : { ...emptyAnswer(), selected: [PREFER_NOT_TO_SAY] });
	}

	function setText(event: Event & { currentTarget: HTMLInputElement | HTMLTextAreaElement }) {
		onAnswer({ ...answer, text: event.currentTarget.value });
	}
</script>

{#if isChoiceKind(question.kind)}
	{#if question.kind === 'multiple_choice'}
		<p class="text-muted-foreground text-sm">
			{selectionHint(question.kind, question.maxSelections)}
		</p>
	{/if}
	<ul class="flex flex-col gap-2">
		{#each choices as choice (choice)}
			{@const selected = answer.selected.includes(choice)}
			<li>
				<button
					type="button"
					aria-pressed={selected}
					disabled={isChoiceDisabled(
						answer,
						choice,
						question.kind,
						question.maxSelections
					)}
					onclick={() => pick(choice)}
					class="flex w-full items-center justify-between rounded-lg border px-4 py-3 text-left text-base transition-colors disabled:cursor-not-allowed disabled:opacity-50 {selected
						? 'border-primary bg-primary/10'
						: 'border-border bg-card hover:bg-muted'}"
				>
					<span>{choice}</span>
					{#if selected}
						<Check class="text-primary size-4" aria-hidden="true" />
					{/if}
				</button>
				{#if choice === OTHER_CHOICE && selected}
					<Input
						class="mt-2 h-[3.125rem] px-4 text-base"
						placeholder="Please specify"
						aria-label="Other, please specify"
						value={answer.otherText}
						oninput={(event) =>
							onAnswer({ ...answer, otherText: event.currentTarget.value })}
					/>
				{/if}
			</li>
		{/each}
	</ul>
{:else}
	<div class="flex flex-col gap-3">
		{#if question.kind === 'open_text'}
			<Textarea
				class="min-h-32 px-4 py-3 text-base"
				{placeholder}
				aria-label="Your answer"
				disabled={declined}
				value={answer.text}
				oninput={setText}
			/>
		{:else}
			<Input
				class="h-[3.125rem] px-4 text-base"
				type={question.kind === 'number' ? 'number' : 'text'}
				{placeholder}
				aria-label="Your answer"
				disabled={declined}
				value={answer.text}
				oninput={setText}
			/>
		{/if}
		{#if question.preferNotToSay}
			<button
				type="button"
				aria-pressed={declined}
				onclick={toggleDeclined}
				class="flex w-full items-center justify-between rounded-lg border px-4 py-3 text-left text-base transition-colors {declined
					? 'border-primary bg-primary/10'
					: 'border-border bg-card hover:bg-muted'}"
			>
				<span>{PREFER_NOT_TO_SAY}</span>
				{#if declined}
					<Check class="text-primary size-4" aria-hidden="true" />
				{/if}
			</button>
		{/if}
	</div>
{/if}
