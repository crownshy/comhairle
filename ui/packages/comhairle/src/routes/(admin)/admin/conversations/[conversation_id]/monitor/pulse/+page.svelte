<script lang="ts">
	import ActionHint from '../ActionHint.svelte';
	import MonitorStatsBar from '../MonitorStatsBar.svelte';
	import TodayJourneys from '../TodayJourneys.svelte';
	import ActivityHeatmap from '../ActivityHeatmap.svelte';
	import { Button } from '$lib/components/ui/button';
	import * as sample from '../sampleMonitorData';

	// TODO: make util
	function scrollToCard(id: string) {
		const card = document.getElementById(id);
		const reduceMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
		card?.scrollIntoView({ behavior: reduceMotion ? 'auto' : 'smooth', block: 'start' });
		card?.focus({ preventScroll: true });
	}

	let worstStep = $derived(
		sample.SAMPLE_STEPS.reduce((worst, step) =>
			step.reached - step.finished > worst.reached - worst.finished ? step : worst
		)
	);
</script>

<h2>Coming soon</h2>
<!-- <div class="flex flex-col gap-6"> -->
<!-- 	<ActionHint> -->
<!-- 		Most people who stop leave at <strong>{worstStep.name}</strong>: -->
<!-- 		{worstStep.reached - worstStep.finished} of the {worstStep.reached} who reached it didn't complete -->
<!-- 		it. -->
<!-- 		{#snippet actions()} -->
<!-- 			<Button -->
<!-- 				class="rounded-full bg-orange-700 text-white hover:bg-orange-800" -->
<!-- 				onclick={() => scrollToCard('today-journeys')}>See more</Button -->
<!-- 			> -->
<!-- 		{/snippet} -->
<!-- 	</ActionHint> -->
<!-- 	<MonitorStatsBar -->
<!-- 		stats={[ -->
<!-- 			{ -->
<!-- 				name: 'Active now', -->
<!-- 				amount: sample.SAMPLE_ACTIVE_NOW, -->
<!-- 				unit: null, -->
<!-- 				explanation: -->
<!-- 					'People who did something in the conversation in the last 5 minutes, in any step.' -->
<!-- 			}, -->
<!-- 			{ -->
<!-- 				name: 'Joined today', -->
<!-- 				amount: 10, -->
<!-- 				unit: null, -->
<!-- 				explanation: -->
<!-- 					'People who opened the conversation for the first time today, through any link, email or QR code. Counts reset at midnight.' -->
<!-- 			}, -->
<!-- 			{ -->
<!-- 				name: 'Completed today', -->
<!-- 				amount: 3, -->
<!-- 				unit: null, -->
<!-- 				explanation: -->
<!-- 					'People who reached the end of the last step today, whenever they first joined. Counts reset at midnight.' -->
<!-- 			} -->
<!-- 		]} -->
<!-- 	/> -->
<!-- 	<TodayJourneys steps={sample.SAMPLE_STEPS} journeys={sample.SAMPLE_TODAY_JOURNEYS} /> -->
<!-- 	<ActivityHeatmap -->
<!-- 		days={sample.HEATMAP_DAYS} -->
<!-- 		slots={sample.HEATMAP_SLOTS} -->
<!-- 		counts={sample.SAMPLE_HEATMAP} -->
<!-- 	/> -->
<!-- </div> -->
