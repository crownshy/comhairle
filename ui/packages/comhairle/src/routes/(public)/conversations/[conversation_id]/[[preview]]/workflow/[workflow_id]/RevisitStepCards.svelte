<script lang="ts">
	import type { LocalizedWorkflowStepDto } from '@crownshy/api-client/api';
	import { Button } from '$lib/components/ui/button';
	import { workflow_step_url } from '$lib/urls';
	import { REVISIT_COPY } from './revisitCopy';
	import { stepToolType } from './revisitableSteps';

	type Props = {
		steps: LocalizedWorkflowStepDto[];
		conversationId: string;
		workflowId: string;
		isPreview: boolean;
	};
	let { steps, conversationId, workflowId, isPreview }: Props = $props();
</script>

<ol class="flex flex-col gap-4">
	{#each steps as step, index (step.id)}
		{@const copy = REVISIT_COPY[stepToolType(step)]}
		<li
			class="bg-admin-background flex flex-col gap-4 rounded-lg p-5 sm:flex-row sm:items-center sm:justify-between"
		>
			<div class="flex items-center gap-4">
				<span
					class="bg-primary-foreground flex size-9 shrink-0 items-center justify-center rounded-lg text-xl font-medium"
					aria-hidden="true"
				>
					{index + 1}
				</span>
				<div class="flex flex-col gap-1">
					<span class="text-primary text-xl font-medium">{copy.label()}</span>
					<span class="text-lg">{step.name}</span>
					<span class="text-lg">{copy.body()}</span>
				</div>
			</div>
			<Button
				variant="outline"
				size="xl"
				class="w-full sm:w-56 sm:shrink-0"
				href={workflow_step_url(conversationId, workflowId, step.id, isPreview)}
				aria-label={`${copy.action()}: ${step.name}`}
			>
				{copy.action()}
			</Button>
		</li>
	{/each}
</ol>
