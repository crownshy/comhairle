<script lang="ts">
	import ActionHint from '../ActionHint.svelte';
	import StepFunnel from '../StepFunnel.svelte';
	import * as sample from '../sampleMonitorData';

	let worstStep = $derived(
		sample.SAMPLE_STEPS.reduce((worst, step) =>
			step.reached - step.finished > worst.reached - worst.finished ? step : worst
		)
	);

	let showFunnelAs = $state<'people' | 'percent'>('people');
</script>

<!-- Journey -->
<div class="flex flex-col gap-6">
	<!-- TODO: commented out as dependent on solution to notify only drop off users -->
	<!-- <ActionHint tone="info"> -->
	<!-- 	{worstStep.inactiveThreeDays} people started <strong>{worstStep.name}</strong> but -->
	<!-- 	haven't been back for 3 days or more. A short reminder can bring them back. -->
	<!-- 	{#snippet actions()} -->
	<!-- 		<Button href="{base}/notifications" class="rounded-full">Message them</Button> -->
	<!-- 	{/snippet} -->
	<!-- </ActionHint> -->
	<StepFunnel
		steps={sample.SAMPLE_STEPS}
		joined={sample.SAMPLE_JOINED}
		worstStep={worstStep.name}
		bind:showAs={showFunnelAs}
	/>
</div>
