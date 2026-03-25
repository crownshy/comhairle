<script lang="ts">
	import { cn } from '$lib/utils';
	import { Check } from 'lucide-svelte';

	export type StepStatus = 'completed' | 'completed-locked' | 'current' | 'upcoming';

	export interface StepItem {
		id: string;
		name: string;
		status: StepStatus;
		href?: string;
	}

	interface StepSelectorProps {
		steps: StepItem[];
	}

	let { steps }: StepSelectorProps = $props();

	function getLineColor(stepStatus: StepStatus): string {
		return stepStatus === 'upcoming' ? 'bg-primary/20' : 'bg-primary';
	}
</script>

{#snippet stepIndicator(step: StepItem, index: number, size: 'sm' | 'lg')}
	{@const lg = size === 'lg'}
	{@const outer = lg ? 'h-10 w-10' : 'h-5 w-5'}
	{@const icon = lg ? 'h-5 w-5' : 'h-3 w-3'}
	{@const dot = lg ? 'h-5 w-5' : 'h-2.5 w-2.5'}
	{@const label = `Step ${index + 1}: ${step.name}`}

	{#if step.status === 'completed'}
		{#if step.href}
			<a
				href={step.href}
				class={cn(
					'bg-primary flex shrink-0 items-center justify-center rounded-full',
					outer,
					lg
						? 'transition-all hover:scale-110 hover:shadow-md'
						: 'outline-1 -outline-offset-1 outline-white'
				)}
				aria-label="{label} (completed, click to return)"
				title={lg ? `${label} (completed, click to return)` : undefined}
			>
				<Check class={cn('text-primary-foreground', icon)} />
			</a>
		{:else}
			<div
				class={cn(
					'bg-primary flex shrink-0 items-center justify-center rounded-full',
					outer,
					!lg && 'outline-1 -outline-offset-1 outline-white'
				)}
				aria-label="{label} (completed)"
				title={lg ? `${label} (completed)` : undefined}
			>
				<Check class={cn('text-primary-foreground', icon)} />
			</div>
		{/if}
	{:else if step.status === 'completed-locked'}
		<div
			class={cn(
				'bg-popover outline-ring/40 flex shrink-0 items-center justify-center rounded-full',
				outer,
				lg ? 'outline-2 -outline-offset-2' : 'outline-1 -outline-offset-1'
			)}
			aria-label="{label} (completed, locked)"
			title={lg ? `${label} (completed)` : undefined}
		>
			<Check class={cn(icon, lg ? 'text-muted-foreground' : 'text-primary')} />
		</div>
	{:else if step.status === 'current'}
		<div
			class={cn(
				'bg-primary flex shrink-0 items-center justify-center rounded-full',
				outer,
				lg
					? 'outline-primary outline-1 -outline-offset-1'
					: 'outline-[0.5px] -outline-offset-[0.5px]'
			)}
			aria-current="step"
			aria-label="{label} (current)"
			title={lg ? `${label} (current)` : undefined}
		>
			<div class={cn('bg-background rounded-full', dot)}></div>
		</div>
	{:else}
		<div
			class={cn(
				'flex shrink-0 items-center justify-center rounded-full',
				outer,
				lg
					? '-outline-offset-1 outline-transparent'
					: 'outline-[0.5px] -outline-offset-[0.5px]'
			)}
			aria-label="{label} (upcoming)"
			title={lg ? `${label} (upcoming)` : undefined}
		>
			<div class={cn('bg-primary/20 rounded-full', dot)}></div>
		</div>
	{/if}
{/snippet}

{#snippet stepLabel(step: StepItem, index: number, align: 'center' | 'left')}
	<div
		class={cn(
			'flex flex-col',
			align === 'center' ? 'items-center text-center' : 'items-start text-left'
		)}
	>
		<span
			class={cn(
				'text-xs leading-5 font-medium uppercase',
				step.status === 'completed-locked' ? 'text-secondary-foreground/60' : 'text-ring'
			)}
		>
			Step {index + 1}
		</span>
		{#if step.status === 'completed' && step.href}
			<a
				href={step.href}
				class={cn('text-foreground text-sm leading-5 font-medium hover:underline')}
			>
				{step.name}
			</a>
		{:else}
			<span
				class={cn(
					'px-3 text-sm leading-5 font-medium',
					step.status === 'completed-locked'
						? 'text-muted-foreground/60'
						: 'text-foreground'
				)}
			>
				{step.name}
			</span>
		{/if}
	</div>
{/snippet}

<!-- Desktop: horizontal stepper -->
<nav aria-label="Workflow steps" class="hidden w-full max-w-5xl md:flex">
	<ol class="flex w-full items-start">
		{#each steps as step, index (step.id)}
			{@const isFirst = index === 0}
			{@const isLast = index === steps.length - 1}
			<li class="flex flex-1 flex-col items-center gap-2">
				<!-- Connector line + circle row -->
				<div class="flex w-full items-center gap-1.5">
					<!-- Left line -->
					<div
						class={cn(
							'h-0.5 flex-1',
							isFirst ? 'bg-transparent' : getLineColor(step.status)
						)}
					></div>

					{@render stepIndicator(step, index, 'lg')}

					<!-- Right line -->
					<div
						class={cn(
							'h-0.5 flex-1',
							isLast ? 'bg-transparent' : getLineColor(step.status)
						)}
					></div>
				</div>

				{@render stepLabel(step, index, 'center')}
			</li>
		{/each}
	</ol>
</nav>

<!-- Mobile: compact horizontal bar (dots only, no labels) -->
<nav aria-label="Workflow steps" class="w-full md:hidden">
	<ol class="flex w-full items-center rounded-md">
		{#each steps as step, index (step.id)}
			{@const isFirst = index === 0}
			{@const isLast = index === steps.length - 1}
			<li class="flex flex-1 items-center gap-0.5">
				<div class={cn('h-[1.5px] flex-1', isFirst ? '' : getLineColor(step.status))}></div>
				{@render stepIndicator(step, index, 'sm')}
				<div class={cn('h-[1.5px] flex-1', isLast ? '' : getLineColor(step.status))}></div>
			</li>
		{/each}
	</ol>
</nav>
