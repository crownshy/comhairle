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

	function getLineColor(
		stepStatus: StepStatus,
		side: 'left' | 'right',
		nextStatus?: StepStatus
	): string {
		if (stepStatus === 'completed' || stepStatus === 'completed-locked') {
			return 'bg-primary';
		}
		if (stepStatus === 'current') {
			return 'bg-primary';
		}
		// upcoming
		return 'bg-primary/20';
	}
</script>

{#snippet circleIndicator(step: StepItem, index: number)}
	{#if step.status === 'completed'}
		{#if step.href}
			<a
				href={step.href}
				class="bg-primary flex h-10 w-10 shrink-0 items-center justify-center rounded-full transition-all hover:scale-110 hover:shadow-md"
				aria-label="Step {index + 1}: {step.name} (completed, click to return)"
				title="Step {index + 1}: {step.name} (completed, click to return)"
			>
				<Check class="text-primary-foreground h-5 w-5" />
			</a>
		{:else}
			<div
				class="bg-primary flex h-10 w-10 shrink-0 items-center justify-center rounded-full"
				aria-label="Step {index + 1}: {step.name} (completed)"
				title="Step {index + 1}: {step.name} (completed)"
			>
				<Check class="text-primary-foreground h-5 w-5" />
			</div>
		{/if}
	{:else if step.status === 'completed-locked'}
		<div
			class="bg-popover outline-ring/40 flex h-10 w-10 shrink-0 items-center justify-center rounded-full outline-2 -outline-offset-2"
			aria-label="Step {index + 1}: {step.name} (completed, locked)"
			title="Step {index + 1}: {step.name} (completed)"
		>
			<Check class="text-muted-foreground h-5 w-5" />
		</div>
	{:else if step.status === 'current'}
		<div
			class="bg-primary/20 outline-primary/5 flex h-10 w-10 shrink-0 items-center justify-center rounded-full outline-1 -outline-offset-1"
			aria-current="step"
			aria-label="Step {index + 1}: {step.name} (current)"
			title="Step {index + 1}: {step.name} (current)"
		>
			<div class="bg-primary h-5 w-5 rounded-full"></div>
		</div>
	{:else}
		<div
			class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full -outline-offset-1 outline-transparent"
			aria-label="Step {index + 1}: {step.name} (upcoming)"
			title="Step {index + 1}: {step.name} (upcoming)"
		>
			<div class="bg-primary/20 h-5 w-5 rounded-full"></div>
		</div>
	{/if}
{/snippet}

{#snippet smallCircleIndicator(step: StepItem, index: number)}
	{#if step.status === 'completed'}
		{#if step.href}
			<a
				href={step.href}
				class="bg-primary flex h-5 w-5 shrink-0 items-center justify-center rounded-full outline-1 -outline-offset-1 outline-white"
				aria-label="Step {index + 1}: {step.name} (completed, click to return)"
			>
				<Check class="text-primary-foreground h-3 w-3" />
			</a>
		{:else}
			<div
				class="bg-primary flex h-5 w-5 shrink-0 items-center justify-center rounded-full outline-1 -outline-offset-1 outline-white"
				aria-label="Step {index + 1}: {step.name} (completed)"
			>
				<Check class="text-primary-foreground h-3 w-3" />
			</div>
		{/if}
	{:else if step.status === 'completed-locked'}
		<div
			class="bg-popover outline-ring/40 flex h-5 w-5 shrink-0 items-center justify-center rounded-full outline-1 -outline-offset-1"
			aria-label="Step {index + 1}: {step.name} (completed, locked)"
		>
			<Check class="text-primary h-3 w-3" />
		</div>
	{:else if step.status === 'current'}
		<div
			class="bg-primary/20 flex h-5 w-5 shrink-0 items-center justify-center rounded-full outline-[0.5px] -outline-offset-[0.5px]"
			aria-current="step"
			aria-label="Step {index + 1}: {step.name} (current)"
		>
			<div class="bg-primary h-2.5 w-2.5 rounded-full"></div>
		</div>
	{:else}
		<div
			class="flex h-5 w-5 shrink-0 items-center justify-center rounded-full outline-[0.5px] -outline-offset-[0.5px]"
			aria-label="Step {index + 1}: {step.name} (upcoming)"
		>
			<div class="bg-primary/20 h-2.5 w-2.5 rounded-full"></div>
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
					'text-sm leading-5 font-medium',
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
			{@const nextStep = steps[index + 1]}
			<li class="flex flex-1 flex-col items-center gap-4 py-3">
				<!-- Connector line + circle row -->
				<div class="flex w-full items-center gap-1.5">
					<!-- Left line -->
					<div
						class={cn(
							'h-0.5 flex-1',
							isFirst ? 'bg-transparent' : getLineColor(step.status, 'left')
						)}
					></div>

					{@render circleIndicator(step, index)}

					<!-- Right line -->
					<div
						class={cn(
							'h-0.5 flex-1',
							isLast
								? 'bg-transparent'
								: getLineColor(step.status, 'right', nextStep?.status)
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
				<div
					class={cn('h-[1.5px] flex-1', isFirst ? '' : getLineColor(step.status, 'left'))}
				></div>
				{@render smallCircleIndicator(step, index)}
				<div
					class={cn(
						'h-[1.5px] flex-1',
						isLast ? '' : getLineColor(step.status, 'right', steps[index + 1]?.status)
					)}
				></div>
			</li>
		{/each}
	</ol>
</nav>
