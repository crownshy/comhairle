<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { cn } from '$lib/utils';
	import type { StepProgress } from './sampleMonitorData';

	type Props = {
		steps: StepProgress[];
		joined: number;
		/** Name of the step to flag as the biggest drop-off. */
		worstStep: string;
		showAs: 'people' | 'percent';
	};

	let { steps, joined, worstStep, showAs = $bindable('people') }: Props = $props();

	type Row = {
		label: string;
		value: (step: StepProgress) => string;
		/** Highlight this row's cell in the worst step's column. */
		flagWorst?: boolean;
		/** Draw a small bar behind the value (0–100). */
		bar?: (step: StepProgress) => number;
	};

	function percentOf(value: number, total: number): number {
		return total === 0 ? 0 : Math.round((value / total) * 100);
	}

	function count(value: number): string {
		return showAs === 'people' ? `${value}` : `${percentOf(value, joined)}%`;
	}

	let rows: Row[] = $derived([
		{ label: 'Reached this step', value: (step) => count(step.reached) },
		{ label: 'Completed', value: (step) => count(step.finished) },
		{
			label: 'Completion rate',
			value: (step) => `${percentOf(step.finished, step.reached)}%`,
			bar: (step) => percentOf(step.finished, step.reached)
		},
		{
			label: 'Stopped here',
			value: (step) => count(step.reached - step.finished),
			flagWorst: true
		},
		{ label: 'Inactive 3+ days', value: (step) => `${step.inactiveThreeDays}` },
		{ label: 'Median time', value: (step) => `${step.medianMinutes} min`, flagWorst: true }
	]);

	/**
	 * Diagonal stripes for "started, not finished", so it doesn't read as a lighter
	 * "completed". Set as an inline style rather than a Tailwind arbitrary class so the
	 * pattern doesn't depend on Tailwind picking the class up at build time.
	 */
	const IN_PROGRESS_FILL =
		'repeating-linear-gradient(45deg, color-mix(in oklab, var(--primary) 40%, transparent) 0 4px, color-mix(in oklab, var(--primary) 12%, transparent) 4px 8px)';
	const LABEL_CELL = 'text-muted-foreground flex items-center py-3 pr-4 text-sm';
	const STEP_CELL = 'flex items-center border-l px-4 py-3';
</script>

<!--
	One column per step, one row per measure, so values line up across steps. In each bar the
	dashed box is everyone who joined, the striped fill who reached the step, the solid fill
	who completed it.
-->
<Card.Root>
	<Card.Header class="flex flex-wrap items-center justify-between gap-4">
		<Card.Title>Step funnel</Card.Title>
		<div
			role="group"
			aria-label="Show counts as"
			class="bg-card inline-flex rounded-lg border p-1"
		>
			{#each [{ value: 'people', label: 'People' }, { value: 'percent', label: '% of joiners' }] as option (option.value)}
				<button
					type="button"
					aria-pressed={showAs === option.value}
					onclick={() => (showAs = option.value as 'people' | 'percent')}
					class={cn(
						'min-h-9 rounded-md px-3 text-sm',
						showAs === option.value
							? 'bg-sidebar text-sidebar-foreground'
							: 'text-muted-foreground'
					)}
				>
					{option.label}
				</button>
			{/each}
		</div>
	</Card.Header>
	<Card.Content class="flex flex-col gap-6">
		<ul class="flex flex-wrap gap-5 text-sm" aria-label="Legend">
			<li class="inline-flex items-center gap-2">
				<span class="bg-primary size-3.5 rounded-sm"></span>Completed
			</li>
			<li class="inline-flex items-center gap-2">
				<span class="size-3.5 rounded-sm" style:background-image={IN_PROGRESS_FILL}
				></span>Started, not completed
			</li>
			<li class="inline-flex items-center gap-2">
				<span class="border-muted-foreground/50 size-3.5 rounded-sm border border-dashed"
				></span>Didn't reach this step
			</li>
		</ul>

		<div class="overflow-x-auto">
			<div
				role="table"
				aria-label="Progress through each step"
				class="grid min-w-[860px] grid-cols-[10rem_repeat(5,minmax(9rem,1fr))] tabular-nums"
			>
				<div role="row" class="contents">
					<div role="columnheader" class={LABEL_CELL}>
						<span class="sr-only">Measure</span>
					</div>
					{#each steps as step, index (step.name)}
						<div
							role="columnheader"
							class={cn(STEP_CELL, 'flex-col items-start justify-end gap-1')}
						>
							<span class="text-muted-foreground text-sm">Step {index + 1}</span>
							<span class="text-base font-semibold">{step.name}</span>
							{#if step.name === worstStep}
								<span
									class="rounded-full bg-amber-100 px-2 py-0.5 text-sm font-semibold text-amber-900"
								>
									Biggest drop-off
								</span>
							{/if}
						</div>
					{/each}
				</div>

				<div role="row" class="contents">
					<div role="rowheader" class={cn(LABEL_CELL, 'items-end')}>
						Of {joined} who joined
					</div>
					{#each steps as step (step.name)}
						<div role="cell" class={cn(STEP_CELL, 'items-end')}>
							<div
								class="border-muted-foreground/40 flex h-56 w-full items-end rounded-md border border-dashed"
								role="img"
								aria-label="{step.reached} of {joined} reached {step.name}, {step.finished} completed"
							>
								<div
									class="flex w-full items-end overflow-hidden rounded-md"
									style:background-image={IN_PROGRESS_FILL}
									style:height="{percentOf(step.reached, joined)}%"
								>
									<div
										class="bg-primary w-full rounded-b-md"
										style:height="{percentOf(step.finished, step.reached)}%"
									></div>
								</div>
							</div>
						</div>
					{/each}
				</div>

				{#each rows as row (row.label)}
					<div role="row" class="contents">
						<div role="rowheader" class={cn(LABEL_CELL, 'border-t')}>{row.label}</div>
						{#each steps as step (step.name)}
							{@const flagged = row.flagWorst && step.name === worstStep}
							<div
								role="cell"
								class={cn(
									STEP_CELL,
									'gap-3 border-t text-base',
									flagged && 'bg-amber-50 font-semibold text-amber-900'
								)}
							>
								{#if row.bar}
									<div
										class="bg-muted h-2 flex-1 rounded-full"
										aria-hidden="true"
									>
										<div
											class={cn(
												'h-2 rounded-full',
												step.name === worstStep
													? 'bg-amber-600'
													: 'bg-primary'
											)}
											style:width="{row.bar(step)}%"
										></div>
									</div>
								{/if}
								<span class={cn(row.bar && 'w-10 text-right')}
									>{row.value(step)}</span
								>
							</div>
						{/each}
					</div>
				{/each}
			</div>
		</div>
	</Card.Content>
</Card.Root>
