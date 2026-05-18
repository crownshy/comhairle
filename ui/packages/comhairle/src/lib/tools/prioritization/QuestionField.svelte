<script lang="ts">
	import * as RadioGroup from '$lib/components/ui/radio-group';
	import { Slider } from '$lib/components/ui/slider';
	import { Textarea } from '$lib/components/ui/textarea';
	import type { AnswerValue, Question } from './types';

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

	function continuousMidpoint(min: number, max: number): number {
		return min + (max - min) / 2;
	}
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

	{#if question.type === 'text'}
		<Textarea
			disabled={readonly}
			placeholder="Enter your answer"
			value={value?.kind === 'text' ? value.value : ''}
			oninput={(e) => {
				const target = e.target as HTMLTextAreaElement;
				onChange({ kind: 'text', value: target.value });
			}}
		/>
	{:else if question.type === 'likert_scale'}
		<RadioGroup.Root
			value={value?.kind === 'numeric' ? String(value.value) : ''}
			onValueChange={(v: string) => onChange({ kind: 'numeric', value: Number(v) })}
			disabled={readonly}
			class="grid auto-cols-fr grid-flow-col gap-2"
		>
			{#each question.categories as c (c.value)}
				{@const selected = value?.kind === 'numeric' && value.value === c.value}
				<label
					class="hover:bg-muted/50 flex cursor-pointer flex-col items-center justify-center gap-1 rounded-md border p-3 text-center text-sm {selected
						? 'border-primary bg-primary/5'
						: ''}"
				>
					<RadioGroup.Item value={String(c.value)} class="sr-only" />
					<span>{c.label || `Option ${c.value}`}</span>
				</label>
			{/each}
		</RadioGroup.Root>
	{:else if question.type === 'continuous'}
		<div class="flex flex-col gap-2">
			<Slider
				type="single"
				value={value?.kind === 'numeric'
					? value.value
					: continuousMidpoint(question.minValue, question.maxValue)}
				min={question.minValue}
				max={question.maxValue}
				step={1}
				disabled={readonly}
				onValueChange={(v: number) => onChange({ kind: 'numeric', value: v })}
			/>
			{#if question.minLabel || question.maxLabel}
				<div class="text-muted-foreground flex justify-between text-xs">
					<span>{question.minLabel}</span>
					<span>{question.maxLabel}</span>
				</div>
			{/if}
		</div>
	{/if}
</div>
