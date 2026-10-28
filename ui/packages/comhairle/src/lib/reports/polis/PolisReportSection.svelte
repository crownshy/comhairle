<script lang="ts">
	import MetricOverviewCard from '$lib/reports/MetricOverviewCard.svelte';
	import AreaOfConsensus from './AreaOfConsensus.svelte';
	import ConsensusContinuum from './ConsensusContinuum.svelte';
	import OpinionGroups from './OpinionGroups.svelte';
	import type { PolisEmbedData } from './embedData';

	/**
	 * Renders a single embeddable Polis section block from its public report data. Uses the
	 * same section components as `PolisInsights` so an embed looks like the Insights tab.
	 */
	type Props = {
		data: PolisEmbedData;
		/**
		 * When true, render the static variant: interactive controls dropped, all rows shown,
		 * beeswarm laid out synchronously. Kept for a future no-JS export (ADR-0012). Live
		 * embeds leave it false (interactive).
		 */
		frozen?: boolean;
	};

	let { data, frozen = false }: Props = $props();
</script>

{#if data.componentType === 'polis-key-stats'}
	{@const stats = data.stats}
	{@const votesPerVoter = stats.participants > 0 ? stats.votes / stats.participants : 0}
	<div class="flex flex-wrap gap-4">
		<MetricOverviewCard
			superText="Participants"
			metric={stats.participants}
			subText="unique voters"
		/>
		<MetricOverviewCard
			superText="Statements"
			metric={stats.statements}
			subText="approved by moderators"
		/>
		<MetricOverviewCard
			superText="Vote cast"
			metric={stats.votes}
			subText="{votesPerVoter.toFixed(1)} avg per voter"
		/>
	</div>
{:else if data.componentType === 'polis-area-consensus'}
	<AreaOfConsensus
		title="Area of consensus"
		comments={data.statements}
		groups={data.groups}
		{frozen}
	/>
{:else if data.componentType === 'polis-area-disagreement'}
	<AreaOfConsensus
		title="Area of disagreement"
		comments={data.statements}
		groups={data.groups}
		{frozen}
	/>
{:else if data.componentType === 'polis-consensus-continuum'}
	<ConsensusContinuum comments={data.statements} groups={data.groups} {frozen} />
{:else if data.componentType === 'polis-opinion-groups'}
	<OpinionGroups comments={data.statements} groups={data.groups} {frozen} />
{/if}
