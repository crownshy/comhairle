<!--
	@component A horizontal stacked vote bar using LayerCake.
	Takes a GroupVotePercent and renders agreed/disagreed/passed/not-voted segments.
-->
<script lang="ts">
	import { LayerCake, Svg } from 'layercake';
	import { scaleOrdinal } from 'd3-scale';
	import StackedBar, { type BarSegment } from './beeswarm/StackedBar.svelte';
	import type { GroupVotePercent } from '$lib/types/report';

	const voteColorScale = scaleOrdinal<string, string>()
		.domain(['agreed', 'disagreed', 'passed', 'notVoted'])
		.range(['#0d9488', '#f87171', '#9ca3af', '#ffffff']);

	const voteBorderScale = scaleOrdinal<string, string | undefined>()
		.domain(['agreed', 'disagreed', 'passed', 'notVoted'])
		.range([undefined, undefined, undefined, '#e5e7eb']);

	interface Props {
		data: GroupVotePercent;
		showLabels?: boolean;
	}

	let { data, showLabels = true }: Props = $props();

	const voteKeys = ['agreed', 'disagreed', 'passed', 'notVoted'] as const;

	const segments: BarSegment[] = $derived(
		voteKeys.map((key) => ({
			key,
			value: data[key],
			color: voteColorScale(key),
			borderColor: voteBorderScale(key)
		}))
	);

	const chartData = $derived([{ total: 100 }]);
</script>

<div class="w-full" style="height: 10px;">
	<LayerCake data={chartData}>
		<Svg>
			<StackedBar {segments} {showLabels} />
		</Svg>
	</LayerCake>
</div>
