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
			if (side === 'left') return 'bg-primary';
			return 'bg-primary/20';
		}
		return 'bg-primary/20';
	}
</script>

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

					<!-- Circle indicator -->
					{#if step.status === 'completed'}
						{#if step.href}
							<a
								href={step.href}
								class="bg-primary flex h-10 w-10 items-center justify-center rounded-full transition-opacity hover:opacity-80"
								aria-label="Step {index + 1}: {step.name} (completed)"
							>
								<Check class="text-primary-foreground h-5 w-5" />
							</a>
						{:else}
							<div
								class="bg-primary flex h-10 w-10 items-center justify-center rounded-full"
								aria-label="Step {index + 1}: {step.name} (completed)"
							>
								<Check class="text-primary-foreground h-5 w-5" />
							</div>
						{/if}
					{:else if step.status === 'completed-locked'}
						<div
							class="bg-popover outline-ring/40 flex h-10 w-10 items-center justify-center rounded-full outline-2 -outline-offset-2"
							aria-label="Step {index + 1}: {step.name} (completed, locked)"
						>
							<Check class="text-muted-foreground h-5 w-5" />
						</div>
					{:else if step.status === 'current'}
						<div
							class="bg-primary-foreground outline-border flex h-10 w-10 items-center justify-center rounded-full outline-1 -outline-offset-1"
							aria-current="step"
							aria-label="Step {index + 1}: {step.name} (current)"
						>
							<div class="bg-primary h-5 w-5 rounded-full"></div>
						</div>
					{:else}
						<div
							class="outline-border flex h-10 w-10 items-center justify-center rounded-full outline-1 -outline-offset-1"
							aria-label="Step {index + 1}: {step.name} (upcoming)"
						>
							<div class="bg-primary-foreground h-5 w-5 rounded-full"></div>
						</div>
					{/if}

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

				<!-- Step label -->
				<div class="flex flex-col items-center text-center">
					<span
						class={cn(
							'text-xs leading-5 font-medium uppercase',
							step.status === 'completed-locked'
								? 'text-secondary-foreground/60'
								: 'text-ring'
						)}
					>
						Step {index + 1}
					</span>
					{#if step.status === 'completed' && step.href}
						<a
							href={step.href}
							class={cn(
								'text-foreground text-sm leading-5 font-medium hover:underline'
							)}
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
			</li>
		{/each}
	</ol>
</nav>
