<script lang="ts">
	import * as RadioGroup from '$lib/components/ui/radio-group';
	import { Slider } from '$lib/components/ui/slider';
	import { Input } from '$lib/components/ui/input';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Label } from '$lib/components/ui/label';
	import { Star } from 'lucide-svelte';
	import { letterFor, type AnswerValue, type Question } from './types';

	let {
		question,
		value,
		onChange,
		readonly = false
	}: {
		question: Question;
		value: AnswerValue | undefined;
		onChange: (v: AnswerValue) => void;
		readonly?: boolean;
	} = $props();
</script>

<div class="flex flex-col gap-2">
	<div>
		<div class="text-muted-foreground text-xs">
			Question {question.order}{question.optional ? ' (optional)' : ''}
		</div>
		<div class="font-medium">{question.prompt || 'Untitled question'}</div>
		{#if question.description}
			<div class="text-muted-foreground text-sm">{question.description}</div>
		{/if}
	</div>

	{#if question.type === 'single_line'}
		<Input
			disabled={readonly}
			placeholder="Your answer"
			value={value?.kind === 'text' ? value.value : ''}
			oninput={(e) => {
				const target = e.target as HTMLInputElement;
				onChange({ kind: 'text', value: target.value });
			}}
		/>
	{:else if question.type === 'long_text'}
		<Textarea
			disabled={readonly}
			placeholder="Enter your answer"
			value={value?.kind === 'text' ? value.value : ''}
			oninput={(e) => {
				const target = e.target as HTMLTextAreaElement;
				onChange({ kind: 'text', value: target.value });
			}}
		/>
	{:else if question.type === 'multiple_choice'}
		<RadioGroup.Root
			value={value?.kind === 'choice' ? value.choiceId : ''}
			onValueChange={(v: string) => onChange({ kind: 'choice', choiceId: v })}
			disabled={readonly}
			class="flex flex-col gap-2"
		>
			{#each question.choices as c, i (c.id)}
				<label
					class="hover:bg-muted/50 flex cursor-pointer items-center gap-3 rounded-md border p-3"
				>
					<RadioGroup.Item value={c.id} />
					<span
						class="bg-muted flex size-6 items-center justify-center rounded text-xs font-semibold"
						>{letterFor(i)}</span
					>
					<span>{c.label || `Choice ${i + 1}`}</span>
				</label>
			{/each}
		</RadioGroup.Root>
	{:else if question.type === 'rating_scale'}
		<div class="flex flex-col gap-2">
			<Slider
				type="single"
				value={value?.kind === 'numeric' ? value.value : (question.min + question.max) / 2}
				min={question.min}
				max={question.max}
				step={1}
				disabled={readonly}
				onValueChange={(v: number) => onChange({ kind: 'numeric', value: v })}
			/>
			<div class="text-muted-foreground flex justify-between text-xs">
				<span>{question.minLabel || question.min}</span>
				<span>{question.maxLabel || question.max}</span>
			</div>
		</div>
	{:else if question.type === 'five_star'}
		<div class="flex gap-1" role="radiogroup" aria-label={question.prompt}>
			{#each [1, 2, 3, 4, 5] as n (n)}
				<button
					type="button"
					role="radio"
					aria-checked={value?.kind === 'numeric' && value.value === n}
					aria-label={`${n} out of 5`}
					disabled={readonly}
					onclick={() => onChange({ kind: 'numeric', value: n })}
					class="text-muted-foreground hover:text-amber-500"
				>
					<Star
						class="size-7 {value?.kind === 'numeric' && value.value >= n
							? 'fill-amber-400 text-amber-400'
							: ''}"
					/>
				</button>
			{/each}
		</div>
	{/if}
</div>
