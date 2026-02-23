<script lang="ts">
	import { cn } from '$lib/utils';

	interface Step {
		id: string;
		name: string;
	}

	interface StepSelectorProps {
		steps: Step[];
		currentStepId: string;
	}

	let { steps, currentStepId }: StepSelectorProps = $props();
</script>

<nav aria-label="Workflow steps" class="hidden md:flex">
	<ol class="flex flex-row gap-2">
		{#each steps as step (step.id)}
			<li
				class={cn(
					'flex items-center rounded-full px-5 py-2.5 text-sm font-medium transition-colors',
					currentStepId === step.id
						? 'bg-primary text-primary-foreground shadow-sm'
						: 'bg-primary/10 text-muted-foreground ring-1 ring-primary/20'
				)}
				aria-current={currentStepId === step.id ? 'step' : undefined}
			>
				<span class="text-center">{step.name}</span>
			</li>
		{/each}
	</ol>
</nav>
