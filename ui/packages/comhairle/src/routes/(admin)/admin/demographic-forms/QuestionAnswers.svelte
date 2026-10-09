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
		inputLabel,
		isChoiceKind,
		selectionHint,
		type DemographicQuestion
	} from './demographicPrototypeData';

	type Props = {
		question: Pick<
			DemographicQuestion,
			'kind' | 'options' | 'allowOther' | 'preferNotToSay' | 'placeholder' | 'maxSelections'
		> & { tags?: string[] };
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
	const label = $derived(inputLabel(question.kind, question.tags));
	const placeholder = $derived(question.placeholder || defaultPlaceholder(question.kind));

	function pick(choice: string) {
		onAnswer(toggleChoice(answer, choice, question.kind, question.maxSelections));
	}

	function toggleDeclined() {
		onAnswer(declined ? emptyAnswer() : { ...emptyAnswer(), selected: [PREFER_NOT_TO_SAY] });
	}

	// Dates are typed as day, month and year, and kept together as "DD/MM/YYYY".
	const dateParts = $derived([0, 1, 2].map((i) => answer.text.split('/')[i] ?? ''));

	function setDatePart(index: number, value: string) {
		const parts = [...dateParts];
		parts[index] = value.replace(/\D/g, '');
		onAnswer({ ...answer, text: parts.every((part) => part === '') ? '' : parts.join('/') });
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
					class="flex h-[3.125rem] w-full items-center justify-between rounded-lg border px-4 text-left text-base transition-colors disabled:cursor-not-allowed disabled:opacity-50 {selected
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
		{#if question.kind === 'date_split'}
			<div class="grid grid-cols-2 gap-3">
				{#each [{ name: 'Day', placeholder: 'DD', index: 0 }, { name: 'Month', placeholder: 'MM', index: 1 }, { name: 'Year', placeholder: 'YYYY', index: 2 }] as part (part.name)}
					<label class="flex flex-col gap-1.5 {part.index === 2 ? 'col-span-2' : ''}">
						<span class="text-muted-foreground text-sm">{part.name}</span>
						<Input
							class="h-[3.125rem] px-4 text-base"
							inputmode="numeric"
							placeholder={part.placeholder}
							disabled={declined}
							value={dateParts[part.index]}
							oninput={(event) => setDatePart(part.index, event.currentTarget.value)}
						/>
					</label>
				{/each}
			</div>
		{:else}
			<label class="flex flex-col gap-1.5">
				<span class="text-muted-foreground text-sm">{label}</span>
				{#if question.kind === 'open_text'}
					<Textarea
						class="min-h-32 px-4 py-3 text-base"
						{placeholder}
						disabled={declined}
						value={answer.text}
						oninput={setText}
					/>
				{:else}
					<Input
						class="h-[3.125rem] px-4 text-base"
						type={question.kind === 'number' ? 'number' : 'text'}
						{placeholder}
						disabled={declined}
						value={answer.text}
						oninput={setText}
					/>
				{/if}
			</label>
		{/if}
		{#if question.preferNotToSay}
			<button
				type="button"
				aria-pressed={declined}
				onclick={toggleDeclined}
				class="flex h-[3.125rem] w-full items-center justify-between rounded-lg border px-4 text-left text-sm transition-colors {declined
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
