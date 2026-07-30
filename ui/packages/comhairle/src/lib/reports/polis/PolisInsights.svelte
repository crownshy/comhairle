<script lang="ts">
	import type { ReportComment, PolisReportData } from '$lib/tools/polis/reportTypes';
	import type { PolisStatementAux } from '@crownshy/api-client/api';
	import {
		getEngagementStats,
		getConsensusStatements,
		getDifferenceStatements,
		totalVotes,
		groupLabel
	} from '$lib/tools/polis/report';
	import MetricOverviewCard from '$lib/reports/MetricOverviewCard.svelte';
	import ConsensusContinuum from './ConsensusContinuum.svelte';
	import AreaOfConsensus from './AreaOfConsensus.svelte';
	import OpinionGroups from './OpinionGroups.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Download, ChartNoAxesColumn } from '@lucide/svelte';

	let {
		reportData,
		statementAux
	}: {
		workflowStepId: string;
		reportData: PolisReportData | null;
		statementAux: PolisStatementAux[];
	} = $props();

	// "aux" = PolisStatementAux: our supplementary per-statement record (themes,
	// moderation status/reason, step id, seed flag) that Polis itself doesn't store.
	//
	// Local mutable aux map (keyed by Polis tid) so picker edits re-render without
	// waiting for the invalidated load to round-trip. Re-seeded whenever the
	// `statementAux` prop changes (e.g. after an invalidate or slug change).
	let auxByTid = $state<Record<number, PolisStatementAux>>({});
	$effect(() => {
		const map: Record<number, PolisStatementAux> = {};
		for (const a of statementAux) map[a.polis_statement_id] = a;
		auxByTid = map;
	});

	/**
	 * Overlay aux.themes onto each comment so the existing utils (which read
	 * `comment.topics`) get aux-sourced themes for free. Themes are human-authored
	 * today and live only in aux; Polis carries none, so `comment.topics` is empty
	 * until a future source populates it. Comments with no aux row keep that empty
	 * `topics` and simply contribute no themes. Works on a cloned copy so the
	 * `reportData` prop is never mutated in place across reruns.
	 */
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
	// Statement lists for the two areas: the full statement set, ranked by the
	// Polis-provided scores (group_informed_consensus for consensus, divisiveness
	// for disagreement). Same statements, two orderings; the preview surfaces the
	// strongest of each.
	const consensusStatements = $derived(report ? getConsensusStatements(report) : []);
	const disagreementStatements = $derived(report ? getDifferenceStatements(report) : []);
	// Stat-card subtexts. Statement split reads aux moderation_status ("accepted" →
	// "approved" in the UI); avg is mean votes cast per unique voter.
	const approvedCount = $derived(
		Object.values(auxByTid).filter((a) => a.moderation_status === 'accepted').length
	);
	const pendingCount = $derived(
		Object.values(auxByTid).filter((a) => a.moderation_status === 'pending').length
	);
	const avgVotesPerVoter = $derived(
		stats && stats.totalParticipants > 0 ? stats.totalVotes / stats.totalParticipants : 0
	);

	// --- CSV export (inlined; one row per comment, columns match the UI) ---
	/** Wrap a value for CSV: quote, double internal quotes, normalize newlines. */
	function csvField(value: unknown): string {
		if (value === null || value === undefined) return '';
		const s = String(value).replace(/\r\n|\r/g, '\n');
		return `"${s.replace(/"/g, '""')}"`;
	}

	/** Sorted union of every theme appearing on any statement in the report. */
	function collectThemes(comments: ReportComment[]): string[] {
		const seen: Record<string, true> = {};
		for (const c of comments) {
			for (const t of c.topics ?? []) seen[t] = true;
		}
		return Object.keys(seen).sort();
	}

	function buildStatementsCsv(
		data: PolisReportData,
		auxMap: Record<number, PolisStatementAux>
	): string {
		const themeCols = collectThemes(data.comments);
		const groupIds = data.groups.map((g) => g.group_id).sort((a, b) => a - b);

		const header: string[] = [
			'user_id',
			'agrees',
			'disagrees',
			'passes',
			'total_votes',
			...themeCols.map((t) => `theme: ${t}`),
			'statement_text'
		];
		for (const gid of groupIds) {
			const label = groupLabel(gid);
			header.push(
				`group_${label}_agrees`,
				`group_${label}_disagrees`,
				`group_${label}_passes`
			);
		}
		header.push('moderation_status', 'is_seed');

		const lines = [header.map(csvField).join(',')];

		for (const c of data.comments) {
			const aux = auxMap[c.tid];
			const topicSet = new Set(c.topics ?? []);

			const row: unknown[] = [
				aux?.user_id ?? '',
				c.overall_votes.agrees,
				c.overall_votes.disagrees,
				c.overall_votes.passes,
				totalVotes(c),
				...themeCols.map((t) => (topicSet.has(t) ? 'true' : 'false')),
				c.text
			];

			for (const gid of groupIds) {
				const gv = c.group_votes.find((v) => v.group_id === gid);
				row.push(gv?.agrees ?? 0, gv?.disagrees ?? 0, gv?.passes ?? 0);
			}

			row.push(
				aux?.moderation_status ?? '',
				(c.is_seed ?? aux?.is_seed ?? false) ? 'true' : 'false'
			);

			lines.push(row.map(csvField).join(','));
		}

		return lines.join('\n');
	}

	function downloadCsv(filename: string, csv: string): void {
		const blob = new Blob([csv], { type: 'text/csv;charset=utf-8' });
		const url = URL.createObjectURL(blob);
		const a = document.createElement('a');
		a.href = url;
		a.download = filename;
		document.body.appendChild(a);
		a.click();
		a.remove();
		URL.revokeObjectURL(url);
	}

	function handleDownloadCsv() {
		if (!report) return;
		const csv = buildStatementsCsv(report, auxByTid);
		const ts = new Date().toISOString().slice(0, 10);
		downloadCsv(`polis-statements-${ts}.csv`, csv);
	}
</script>

{#if !report || !stats}
	<div
		class="border-border bg-card text-muted-foreground flex flex-col items-center justify-center gap-3 rounded-xl border border-dashed p-12 text-center"
	>
		<ChartNoAxesColumn class="text-muted-foreground/60 size-8" />
		<div class="flex flex-col gap-1">
			<p class="text-foreground text-base font-medium">No insights yet</p>
			<p class="text-base">
				Insights appear once participants start voting on statements in this step.
			</p>
		</div>
	</div>
{:else}
	<div class="flex flex-col gap-10 pb-8">
		<!-- ===== Top stats + page actions ===== -->
		<div class="flex flex-wrap items-start justify-between gap-4">
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
			<Button size="sm" onclick={handleDownloadCsv}>
				<Download class="size-4" />
				Download CSV
			</Button>
		</div>

		<!-- ===== Consensus continuum ===== -->
		<ConsensusContinuum comments={report.comments} groups={report.groups} />

		<!-- ===== Area of consensus ===== -->
		<AreaOfConsensus
			title="Area of consensus"
			comments={consensusStatements}
			groups={report.groups}
		/>

		<!-- ===== Area of disagreement ===== -->
		<AreaOfConsensus
			title="Area of disagreement"
			comments={disagreementStatements}
			groups={report.groups}
		/>

		<!-- ===== Opinion groups ===== -->
		<OpinionGroups comments={report.comments} groups={report.groups} />
	</div>
{/if}
