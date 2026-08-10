<script lang="ts">
	import type { PolisReportData } from '$lib/tools/polis/reportTypes';
	import type { PolisStatementAux } from '@crownshy/api-client/api';
	import {
		getEngagementStats,
		getConsensusStatements,
		getDifferenceStatements
	} from '$lib/tools/polis/report';
	import MetricOverviewCard from '$lib/reports/MetricOverviewCard.svelte';
	import AreaOfConsensus from './AreaOfConsensus.svelte';
	import ConsensusContinuum from './ConsensusContinuum.svelte';
	import OpinionGroups from './OpinionGroups.svelte';
	import type { PolisEmbeddableComponentType } from './embeddableComponents';

	/**
	 * Renders a single embeddable Polis section block. This is the unit the embed dialog
	 * freezes to HTML (mount → read innerHTML) and, later, could mount live. It mirrors the
	 * relevant slices of `PolisInsights` so an embed looks identical to the Insights tab.
	 */
	let {
		componentType,
		reportData,
		statementAux,
		frozen = true
	}: {
		componentType: PolisEmbeddableComponentType;
		reportData: PolisReportData | null;
		statementAux: PolisStatementAux[];
		/**
		 * Render the frozen-snapshot variant (default): interactive controls dropped, all rows
		 * shown. This is the embed's render, so it defaults on; a future live embed passes false.
		 */
		frozen?: boolean;
	} = $props();

	const auxByTid = $derived.by<Record<number, PolisStatementAux>>(() => {
		const map: Record<number, PolisStatementAux> = {};
		for (const a of statementAux) map[a.polis_statement_id] = a;
		return map;
	});

	// Overlay aux themes onto comments, as PolisInsights does, so themed utils get their
	// data. Cloned so the prop is never mutated.
	const report = $derived.by<PolisReportData | null>(() => {
		if (!reportData) return null;
		return {
			...reportData,
			comments: reportData.comments.map((c) => {
				const aux = auxByTid[c.tid];
				return aux ? { ...c, topics: aux.themes } : c;
			})
		};
	});

	const stats = $derived(report ? getEngagementStats(report) : null);
	const consensusStatements = $derived(report ? getConsensusStatements(report) : []);
	const disagreementStatements = $derived(report ? getDifferenceStatements(report) : []);

	const approvedCount = $derived(
		Object.values(auxByTid).filter((a) => a.moderation_status === 'accepted').length
	);
	const pendingCount = $derived(
		Object.values(auxByTid).filter((a) => a.moderation_status === 'pending').length
	);
	const avgVotesPerVoter = $derived(
		stats && stats.totalParticipants > 0 ? stats.totalVotes / stats.totalParticipants : 0
	);
</script>

{#if !report || !stats}
	<div
		class="border-border bg-card text-muted-foreground rounded-xl border border-dashed p-8 text-center text-base"
	>
		No data yet for this component. It will fill in once participants start voting.
	</div>
{:else if componentType === 'polis-key-stats'}
	<div class="flex flex-wrap gap-4">
		<MetricOverviewCard
			superText="Participants"
			metric={stats.totalParticipants}
			subText="unique voters"
		/>
		<MetricOverviewCard
			superText="Statements"
			metric={stats.totalStatements}
			subText="{approvedCount} approved · {pendingCount} pending"
		/>
		<MetricOverviewCard
			superText="Vote cast"
			metric={stats.totalVotes}
			subText="{avgVotesPerVoter.toFixed(1)} avg per voter"
		/>
	</div>
{:else if componentType === 'polis-area-consensus'}
	<AreaOfConsensus
		title="Area of consensus"
		comments={consensusStatements}
		groups={report.groups}
		{frozen}
	/>
{:else if componentType === 'polis-area-disagreement'}
	<AreaOfConsensus
		title="Area of disagreement"
		comments={disagreementStatements}
		groups={report.groups}
		{frozen}
	/>
{:else if componentType === 'polis-consensus-continuum'}
	<ConsensusContinuum comments={report.comments} groups={report.groups} {frozen} />
{:else if componentType === 'polis-opinion-groups'}
	<OpinionGroups comments={report.comments} groups={report.groups} {frozen} />
{/if}
