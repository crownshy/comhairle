<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import StepSelector from './StepSelector.svelte';

	const { Story } = defineMeta({
		title: 'Patterns/StepSelector',
		component: StepSelector,
		tags: ['autodocs']
	});

	const twoSteps = [
		{ id: 'step-1', name: 'Learn about the topic' },
		{ id: 'step-2', name: 'Tell us about you' }
	];

	const threeSteps = [
		{ id: 'step-1', name: 'Learn about the topic' },
		{ id: 'step-2', name: 'Tell us about you' },
		{ id: 'step-3', name: 'Interactive discussion' }
	];

	const fiveSteps = [
		{ id: 'step-1', name: 'Privacy Policy' },
		{ id: 'step-2', name: 'Learn about the topic' },
		{ id: 'step-3', name: 'Tell us about you' },
		{ id: 'step-4', name: 'Interactive discussion' },
		{ id: 'step-5', name: 'Summary' }
	];
</script>

<Story name="Two Steps - First Active">
	<div class="p-8">
		<StepSelector steps={twoSteps} currentStepId="step-1" />
	</div>
</Story>

<Story name="Two Steps - Second Active">
	<div class="p-8">
		<StepSelector steps={twoSteps} currentStepId="step-2" />
	</div>
</Story>

<Story name="Three Steps - Middle Active">
	<div class="p-8">
		<StepSelector steps={threeSteps} currentStepId="step-2" />
	</div>
</Story>

<Story name="Five Steps - Third Active">
	<div class="p-8">
		<StepSelector steps={fiveSteps} currentStepId="step-3" />
	</div>
</Story>

<Story name="All States">
	<div class="flex flex-col gap-8 p-8">
		<div>
			<p class="mb-2 text-sm text-muted-foreground">First step active</p>
			<StepSelector steps={threeSteps} currentStepId="step-1" />
		</div>
		<div>
			<p class="mb-2 text-sm text-muted-foreground">Middle step active</p>
			<StepSelector steps={threeSteps} currentStepId="step-2" />
		</div>
		<div>
			<p class="mb-2 text-sm text-muted-foreground">Last step active</p>
			<StepSelector steps={threeSteps} currentStepId="step-3" />
		</div>
	</div>
</Story>
