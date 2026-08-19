<script lang="ts">
	import { Label } from '$lib/components/ui/label';
	import * as RadioGroup from '$lib/components/ui/radio-group';
	import { Slider } from '$lib/components/ui/slider';
	import { Textarea } from '$lib/components/ui/textarea';
	import type { Question } from '../types';

	type Props = {
		question: Question<string>;
		value: number | string | null;
		disabled?: boolean;
		/** When true, the question is required but unanswered after a submit attempt. */
		invalid?: boolean;
		onChange: (value: number | string) => void;
	};

	let { question, value, disabled = false, invalid = false, onChange }: Props = $props();

	function handleLikert(raw: string) {
		const n = Number(raw);
		if (!Number.isNaN(n)) onChange(n);
	}

	function handleText(e: Event) {
		const target = e.currentTarget as HTMLTextAreaElement | null;
		if (target) onChange(target.value);
	}

	function handleSlider(v: number) {
		gestureChanged = true;
		onChange(v);
	}

	/** Slider goes from minValue to maxValue with step size derived from subSteps.
	 * `mid` is the neutral resting position for the thumb, `midpoint` the geometric
	 * centre used to decide which end label leans once answered. */
	let sliderRange = $derived.by(() => {
		if (question.type.kind !== 'continuous') return null;
		const { minValue, maxValue, subSteps } = question.type;
		const span = maxValue - minValue;
		const steps = Math.max(2, subSteps);
		const step = span / steps;
		return {
			min: minValue,
			max: maxValue,
			step,
			mid: minValue + step * Math.round(steps / 2),
			midpoint: minValue + span / 2
		};
	});

	/** A continuous answer only counts once it holds a real number. Until then the
	 * thumb just rests at `mid` with no track fill: the "not answered yet" state. */
	let sliderAnswered = $derived(typeof value === 'number');
	let sliderDisplayValue = $derived(typeof value === 'number' ? value : (sliderRange?.mid ?? 0));

	/** Emphasise whichever end the thumb leans toward once the participant answers. */
	let leaningLabel = $derived.by<'min' | 'max' | null>(() => {
		if (!sliderAnswered || !sliderRange || typeof value !== 'number') return null;
		if (value < sliderRange.midpoint) return 'min';
		if (value > sliderRange.midpoint) return 'max';
		return null;
	});

	/** Tracks whether the current pointer gesture moved the thumb. Reset in the
	 * capture phase (before bits-ui updates), so a tap that lands exactly on the
	 * resting thumb (e.g. choosing the middle) still commits an answer on release. */
	let gestureChanged = $state(false);
	function commitRestingIfUntouched() {
		if (disabled) return;
		if (!gestureChanged && !sliderAnswered && sliderRange) onChange(sliderRange.mid);
	}
</script>

<div class="space-y-3" class:opacity-60={disabled}>
	<p class="text-base font-medium" class:text-destructive={invalid}>
		{question.text}
		{#if question.type.kind === 'text'}
			<span class="text-muted-foreground font-normal">(optional)</span>
		{/if}
	</p>

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
			<div class="pref-slider" data-answered={sliderAnswered}>
				<Slider
					type="single"
					value={sliderDisplayValue}
					min={sliderRange.min}
					max={sliderRange.max}
					step={sliderRange.step}
					{disabled}
					onpointerdowncapture={() => (gestureChanged = false)}
					onpointerup={commitRestingIfUntouched}
					onValueChange={(v) => typeof v === 'number' && handleSlider(v)}
				/>
			</div>
			<div class="text-muted-foreground flex justify-between text-xs">
				<span class="font-medium" class:text-primary={leaningLabel === 'min'}>
					{question.type.minLabel ?? ''}
				</span>
				<span class="font-medium" class:text-primary={leaningLabel === 'max'}>
					{question.type.maxLabel ?? ''}
				</span>
			</div>
		</div>
	{:else}
		<Textarea
			placeholder="Your answer"
			rows={3}
			{disabled}
			value={typeof value === 'string' ? value : ''}
			oninput={handleText}
		/>
	{/if}

	{#if invalid}
		<p class="text-destructive text-sm">Please answer this question to continue.</p>
	{/if}
</div>

<style>
	/* Grab affordance while dragging; the track invites a click anywhere. */
	.pref-slider :global([data-slot='slider-thumb']) {
		cursor: grab;
	}
	.pref-slider :global([data-slot='slider-thumb']:active) {
		cursor: grabbing;
	}
	.pref-slider :global([data-slot='slider-track']) {
		cursor: pointer;
	}

	/* The track is the same grey whether or not the question is answered, so the
	 * unfilled remainder after voting matches the unvoted state. */
	.pref-slider :global([data-slot='slider-track']) {
		background-color: var(--border);
	}

	/* Before the participant answers: no track fill, and a blue-outlined resting
	 * thumb so it reads as "interactive but unset" rather than disabled. */
	.pref-slider[data-answered='false'] :global([data-slot='slider-range']) {
		background-color: transparent;
	}
	.pref-slider[data-answered='false'] :global([data-slot='slider-thumb']) {
		border-width: 2px;
		border-color: var(--primary);
		background-color: var(--background);
	}

	/* Once answered, the thumb fills in to match the coloured track range. */
	.pref-slider[data-answered='true'] :global([data-slot='slider-thumb']) {
		border-color: var(--primary);
		background-color: var(--primary);
	}
</style>
