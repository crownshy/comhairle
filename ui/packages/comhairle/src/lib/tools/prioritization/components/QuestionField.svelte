<script lang="ts">
	import { Label } from '$lib/components/ui/label';
	import * as RadioGroup from '$lib/components/ui/radio-group';
	import { Slider } from '$lib/components/ui/slider';
	import { Textarea } from '$lib/components/ui/textarea';
	import type { Question } from '../types';

	type Props = {
		question: Question;
		value: number | null;
		disabled?: boolean;
		onChange: (value: number) => void;
	};

	let { question, value, disabled = false, onChange }: Props = $props();

	function handleLikert(raw: string) {
		const n = Number(raw);
		if (!Number.isNaN(n)) onChange(n);
	}

	function handleSlider(values: number[]) {
		const v = values[0];
		if (typeof v === 'number') onChange(v);
	}

	/** Slider goes from minValue to maxValue with step size derived from subSteps. */
	let sliderRange = $derived.by(() => {
		if (question.type.kind !== 'continuous') return null;
		const { minValue, maxValue, subSteps } = question.type;
		const span = maxValue - minValue;
		const steps = Math.max(2, subSteps);
		return { min: minValue, max: maxValue, step: span / steps };
	});
</script>

<div class="space-y-3" class:opacity-60={disabled}>
	<p class="text-base font-medium">{question.text}</p>

	{#if question.type.kind === 'likert'}
		<RadioGroup.Root
			value={value !== null ? String(value) : undefined}
			onValueChange={handleLikert}
			{disabled}
			class="flex flex-wrap gap-3"
		>
			{#each question.type.categories as cat (cat.value)}
				<Label
					class="hover:border-primary data-[state=checked]:border-primary flex min-w-[120px] flex-1 cursor-pointer flex-col items-center gap-2 rounded-md border p-3"
					data-state={value === cat.value ? 'checked' : 'unchecked'}
				>
					<RadioGroup.Item value={String(cat.value)} />
					<span class="text-sm">{cat.label}</span>
				</Label>
			{/each}
		</RadioGroup.Root>
	{:else if question.type.kind === 'continuous' && sliderRange}
		<div class="space-y-2">
			<Slider
				type="single"
				value={value ?? sliderRange.min}
				min={sliderRange.min}
				max={sliderRange.max}
				step={sliderRange.step}
				{disabled}
				onValueChange={(v) => typeof v === 'number' && handleSlider([v])}
			/>
			<div class="text-muted-foreground flex justify-between text-xs">
				<span>
					{#if question.type.minLabel}<span class="font-medium"
							>{question.type.minLabel}</span
						>
					{/if}{sliderRange.min}
				</span>
				<span>
					{sliderRange.max}{#if question.type.maxLabel}
						<span class="font-medium">{question.type.maxLabel}</span>{/if}
				</span>
			</div>
			{#if value !== null}
				<p class="text-sm">Selected: {value}</p>
			{/if}
		</div>
	{:else}
		<Textarea
			placeholder="Your answer (text answers are not yet recorded by the backend)"
			rows={3}
			{disabled}
		/>
	{/if}
</div>
