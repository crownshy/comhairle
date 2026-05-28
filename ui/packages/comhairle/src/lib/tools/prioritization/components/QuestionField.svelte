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
			class="grid grid-cols-1 gap-2 sm:grid-cols-[repeat(var(--likert-cols),minmax(0,1fr))] sm:gap-3"
			style="--likert-cols: {question.type.categories.length};"
		>
			{#each question.type.categories as cat (cat.value)}
				<Label
					class="group hover:border-primary/60 hover:bg-muted/50 data-[state=checked]:border-primary data-[state=checked]:bg-primary/10 data-[state=checked]:text-primary bg-background flex min-h-[44px] w-full cursor-pointer flex-row items-center gap-3 rounded-lg border p-3 transition-colors sm:min-h-[64px] sm:flex-col sm:justify-center sm:gap-1 sm:p-4 sm:text-center"
					data-state={value === cat.value ? 'checked' : 'unchecked'}
				>
					<RadioGroup.Item value={String(cat.value)} class="sm:sr-only" />
					<span class="text-sm leading-tight group-data-[state=checked]:font-medium">
						{cat.label}
					</span>
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
		</div>
	{:else}
		<Textarea placeholder="Your answer" rows={3} {disabled} />
	{/if}
</div>
