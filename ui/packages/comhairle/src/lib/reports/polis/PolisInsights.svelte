<script lang="ts">
	import type {
		ReportComment,
		PolisReportData,
		ThemeSummary
	} from '$lib/tools/polis/reportTypes';
	import type { PolisStatementAux } from '@crownshy/api-client/api';
	import {
		getEngagementStats,
		themeControversy,
		totalVotes,
		consensusDirection,
		groupLabel
	} from '$lib/tools/polis/report';
	import ThemeBar from '$lib/components/polis-report/ThemeBar.svelte';
	import MetricOverviewCard from '$lib/reports/MetricOverviewCard.svelte';
	import AreaOfConsensus from './AreaOfConsensus.svelte';
	import * as Card from '$lib/components/ui/card';
	import { ChevronDown } from '@lucide/svelte';

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
	// Theme roll-up over ALL tagged statements (aux), not just the Polis report
	// set. Controversy needs vote data (only the report carries), so themes on
	// non-report statements fall back to 'low'.
	const themes = $derived.by<ThemeSummary[]>(() => {
		const counts: Record<string, number> = {};
		for (const row of Object.values(auxByTid)) {
			for (const t of row.themes) counts[t] = (counts[t] ?? 0) + 1;
		}
		return Object.entries(counts)
			.map(([theme, statementCount]) => ({
				theme,
				statementCount,
				controversy: report ? themeControversy(theme, report) : ('low' as const)
			}))
			.sort((a, b) => b.statementCount - a.statementCount);
	});
	// Bars rank against the biggest theme (themes is sorted count-desc).
	const maxThemeCount = $derived(themes[0]?.statementCount ?? 0);

	// Collapse the themes list to a preview; "See all" expands in place.
	const COLLAPSED_ROWS = 5;
	let showAllThemes = $state(false);

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
			'consensus',
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
			const dir = consensusDirection(c, data.groups);
			const consensusLabel = dir === '+' ? 'True +' : dir === '-' ? 'True −' : '';
			const topicSet = new Set(c.topics ?? []);

			const row: unknown[] = [
				aux?.user_id ?? '',
				c.overall_votes.agrees,
				c.overall_votes.disagrees,
				c.overall_votes.passes,
				totalVotes(c),
				...themeCols.map((t) => (topicSet.has(t) ? 'true' : 'false')),
				consensusLabel,
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
	<div class="text-muted-foreground p-8 text-base">
		No report data yet — participants need to vote first.
	</div>
{:else}
	<div class="flex flex-col gap-10 pb-8">
		<!-- ===== Top stats ===== -->
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

		<!-- ===== Themes card ===== -->
		<!-- Display-only for now; the click-to-filter redesign lands with the Themes ticket. -->
		<Card.Root
			class="hover:border-muted-foreground/40 rounded-[20px] p-0 shadow-sm transition-colors duration-200"
		>
			<header class="flex items-start justify-between gap-4 px-8 pt-8">
				<div>
					<h2 class="text-foreground text-lg font-semibold">Themes</h2>
					<p class="text-foreground/70 mt-2 text-base font-medium">
						Themes and subtopics emerged in the conversation.
					</p>
				</div>
			</header>

			<div class="px-8 pt-6 pb-2">
				<div
					class="text-foreground grid grid-cols-[10rem_3rem_1fr_2.5rem] items-center gap-6 px-2 py-2 text-sm font-semibold uppercase"
				>
					<div>Theme</div>
					<div class="text-right">Count</div>
					<div></div>
					<div></div>
				</div>
				{#if themes.length === 0}
					<p class="text-muted-foreground py-6 text-base italic">
						No themes have been generated yet for this conversation.
					</p>
				{:else}
					{#each showAllThemes ? themes : themes.slice(0, COLLAPSED_ROWS) as t (t.theme)}
						<ThemeBar summary={t} barMax={maxThemeCount} />
					{/each}
					{#if themes.length > COLLAPSED_ROWS}
						<button
							type="button"
							onclick={() => (showAllThemes = !showAllThemes)}
							class="text-foreground/70 hover:text-foreground flex w-full items-center justify-center gap-2 py-4 text-base transition-colors"
						>
							{showAllThemes
								? 'Show fewer themes'
								: `See all ${themes.length} themes`}
							<ChevronDown
								class={`text-primary size-4 transition-transform ${showAllThemes ? 'rotate-180' : ''}`}
							/>
						</button>
					{/if}
				{/if}
			</div>
		</Card.Root>

		<!-- ===== Area of consensus ===== -->
		<AreaOfConsensus
			comments={report.comments}
			groups={report.groups}
			onDownloadCsv={handleDownloadCsv}
		/>
	</div>
{/if}
