<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import StepSelector from './StepSelector.svelte';

	const { Story } = defineMeta({
		title: 'Patterns/StepSelector',
		component: StepSelector,
		tags: ['autodocs']
	});

	// Step 3 of 6: current, step 1 revisitable, step 2 locked, 4-6 upcoming
	const sixStepsMiddle = [
		{ id: 's1', name: 'Learn about the topic', status: 'completed', href: '#step-1' },
		{ id: 's2', name: 'Privacy policy', status: 'completed-locked' },
		{ id: 's3', name: 'Tell us about you', status: 'current' },
		{ id: 's4', name: 'About Scottish government', status: 'upcoming' },
		{ id: 's5', name: 'Interactive discussion', status: 'upcoming' },
		{ id: 's6', name: 'Summary', status: 'upcoming' }
	];

	// Step 5 of 6: completed-locked steps (can't go back), current at step 5
	const sixStepsLocked = [
		{ id: 's1', name: 'Learn about the topic', status: 'completed', href: '#step-1' },
		{ id: 's2', name: 'Privacy policy', status: 'completed', href: '#step-2' },
		{ id: 's3', name: 'Tell us about you', status: 'completed-locked' },
		{ id: 's4', name: 'See how others think', status: 'completed-locked' },
		{ id: 's5', name: 'What do you think', status: 'current' },
		{ id: 's6', name: 'Summary', status: 'upcoming' }
	];

	// Summary page: step 1 revisitable, all others locked, current at last
	const sixStepsSummary = [
		{ id: 's1', name: 'Learn about the topic', status: 'completed', href: '#step-1' },
		{ id: 's2', name: 'Privacy policy', status: 'completed-locked' },
		{ id: 's3', name: 'Tell us about you', status: 'completed-locked' },
		{ id: 's4', name: 'See how others think', status: 'completed-locked' },
		{ id: 's5', name: 'What do you think', status: 'completed-locked' },
		{ id: 's6', name: 'Summary', status: 'current' }
	];

	// First step active
	const threeStepsFirst = [
		{ id: 's1', name: 'Learn about the topic', status: 'current' },
		{ id: 's2', name: 'Tell us about you', status: 'upcoming' },
		{ id: 's3', name: 'Interactive discussion', status: 'upcoming' }
	];

	// Two steps
	const twoSteps = [
		{ id: 's1', name: 'Learn about the topic', status: 'completed', href: '#step-1' },
		{ id: 's2', name: 'Tell us about you', status: 'current' }
	];
</script>

<Story name="Step 3 of 6 — Current">
	<div class="p-8">
		<StepSelector steps={sixStepsMiddle} />
	</div>
</Story>

<Story name="Step 5 of 6 — Locked Steps">
	<div class="p-8">
		<StepSelector steps={sixStepsLocked} />
	</div>
</Story>

<Story name="Summary — All Done">
	<div class="p-8">
		<StepSelector steps={sixStepsSummary} />
	</div>
</Story>

<Story name="First Step Active">
	<div class="p-8">
		<StepSelector steps={threeStepsFirst} />
	</div>
</Story>

<Story name="Two Steps">
	<div class="p-8">
		<StepSelector steps={twoSteps} />
	</div>
</Story>

<Story name="All Variants">
	<div class="flex flex-col gap-8 p-8">
		<div>
			<p class="text-muted-foreground mb-2 text-sm">First step active</p>
			<StepSelector steps={threeStepsFirst} />
		</div>
		<div>
			<p class="text-muted-foreground mb-2 text-sm">Middle step — only step 1 revisitable</p>
			<StepSelector steps={sixStepsMiddle} />
		</div>
		<div>
			<p class="text-muted-foreground mb-2 text-sm">Later step — locked completed steps</p>
			<StepSelector steps={sixStepsLocked} />
		</div>
		<div>
			<p class="text-muted-foreground mb-2 text-sm">Summary — all steps completed</p>
			<StepSelector steps={sixStepsSummary} />
		</div>
	</div>
</Story>
