<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { TriangleAlert } from 'lucide-svelte';
	import AiInsight from './AiInsight.svelte';
	import type { StepProgress } from './sampleMonitorData';

	type Props = {
		step: StepProgress;
		typicalMinutes: number;
		notifyHref: string;
		stepHref: string;
		/** AI mode only: a model-written reading of why people stop here. */
		aiExplanation?: string;
	};

	let { step, typicalMinutes, notifyHref, stepHref, aiExplanation }: Props = $props();

	let notFinished = $derived(step.reached - step.finished);
</script>

<section
	aria-label="Biggest drop-off"
	class="flex flex-wrap items-center justify-between gap-6 rounded-xl border border-amber-200 bg-amber-50 p-6"
>
	<div class="flex min-w-0 flex-[1_1_32rem] items-start gap-4">
		<TriangleAlert class="mt-0.5 size-6 shrink-0 text-amber-800" aria-hidden="true" />
		<div class="flex flex-col gap-1.5">
			<h3 class="text-lg font-semibold text-amber-950">Biggest drop-off: {step.name}</h3>
			<p class="text-base leading-relaxed text-amber-950">
				{notFinished} of {step.reached} people started but haven't finished. Median time here
				is
				{step.medianMinutes} min, against {typicalMinutes} min on other steps.
			</p>
			{#if aiExplanation}
				<AiInsight class="mt-2">
					<p class="text-base leading-relaxed text-amber-950">{aiExplanation}</p>
				</AiInsight>
			{/if}
		</div>
	</div>
	<div class="flex flex-wrap gap-3">
		<Button href={notifyHref} class="rounded-full">Message these {notFinished} people</Button>
		<Button href={stepHref} variant="outline" class="rounded-full">Edit step</Button>
	</div>
</section>
