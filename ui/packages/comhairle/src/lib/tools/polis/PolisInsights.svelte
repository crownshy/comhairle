<script lang="ts">
	import type {
		ReportComment,
		PolisReportData,
		ThemeSummary
	} from '$lib/tools/polis/reportTypes';
	import type { PolisStatementAux } from '@crownshy/api-client/api';
	import { apiClient } from '@crownshy/api-client/client';
	import {
		getEngagementStats,
		getConsensusStatements,
		getDifferenceStatements,
		themeControversy,
		classifyStatement,
		isLowQuality,
		totalVotes,
		consensusDirection,
		groupLabel
	} from '$lib/tools/polis/report';
	import { notifications } from '$lib/notifications.svelte';
	import StatementRow from '$lib/components/polis-report/StatementRow.svelte';
	import StatementSection from '$lib/components/polis-report/StatementSection.svelte';
	import ThemeBar from '$lib/components/polis-report/ThemeBar.svelte';
	import ThemeChip from '$lib/components/polis-report/ThemeChip.svelte';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Download, ChevronDown } from '@lucide/svelte';
	import { onMount, tick } from 'svelte';
	import { page } from '$app/state';
	import { replaceState, invalidate } from '$app/navigation';

	const ALL_STATEMENTS_ID = 'all-statements';

	let {
		workflowStepId,
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
	// Theme roll-up over ALL tagged statements (aux), not just the Polis report
	// set. Controversy needs vote data (only the report carries), so themes on
	// non-report statements fall back to 'low'.
	const themes = $derived.by<ThemeSummary[]>(() => {
		const counts = new Map<string, number>();
		for (const row of Object.values(auxByTid)) {
			for (const t of row.themes) counts.set(t, (counts.get(t) ?? 0) + 1);
		}
		return [...counts.entries()]
			.map(([theme, statementCount]) => ({
				theme,
				statementCount,
				controversy: report ? themeControversy(theme, report) : ('low' as const)
			}))
			.sort((a, b) => b.statementCount - a.statementCount);
	});
	// Bars rank against the biggest theme (themes is sorted count-desc).
	const maxThemeCount = $derived(themes[0]?.statementCount ?? 0);

	// --- Section filter state (Consensus / Difference) ---
	let consensusExcludeHosts = $state(false);
	let consensusExcludePasses = $state(false);
	let differencesExcludeHosts = $state(false);
	let differencesExcludePasses = $state(false);

	// Collapse long lists to a preview; "See all" expands in place.
	const COLLAPSED_ROWS = 5;
	let consensusExpanded = $state(false);
	let differencesExpanded = $state(false);
	let showAllThemes = $state(false);

	// Low-quality rows (any group < 10 votes) are hidden by default in every table
	// but stay in the counts; each table reveals its own set.
	let consensusShowLow = $state(false);
	let differencesShowLow = $state(false);

	const consensus = $derived(
		report ? getConsensusStatements(report, { excludePasses: consensusExcludePasses }) : []
	);
	const differences = $derived(
		report ? getDifferenceStatements(report, { excludePasses: differencesExcludePasses }) : []
	);

	const filterHosts = (list: ReportComment[], excludeHosts: boolean) =>
		excludeHosts ? list.filter((c) => !c.is_seed) : list;

	const consensusFiltered = $derived(filterHosts(consensus, consensusExcludeHosts));
	const differencesFiltered = $derived(filterHosts(differences, differencesExcludeHosts));

	// Split each list into trustworthy rows (shown) and low-quality rows (behind a
	// reveal). Both halves stay counted in the section total.
	const consensusMain = $derived(consensusFiltered.filter((c) => !isLowQuality(c)));
	const consensusLow = $derived(consensusFiltered.filter((c) => isLowQuality(c)));
	const differencesMain = $derived(differencesFiltered.filter((c) => !isLowQuality(c)));
	const differencesLow = $derived(differencesFiltered.filter((c) => isLowQuality(c)));

	/** All themes used anywhere on this conversation — powers the picker dropdown. */
	const availableThemes = $derived.by(() => {
		const set = new Set<string>();
		for (const row of Object.values(auxByTid)) {
			for (const t of row.themes) set.add(t);
		}
		return [...set].sort();
	});

	// --- All Statements: theme filter state ---
	// Multi-select theme filter (OR/union). Seeded from ?theme=a,b so the view is
	// deep-linkable/shareable.
	let selectedThemes = $state<string[]>(
		(page.url.searchParams.get('theme') ?? '')
			.split(',')
			.map((s) => s.trim())
			.filter(Boolean)
	);
	let explorerExcludePasses = $state(false);
	let explorerExcludeHosts = $state(false);

	const explorerStatements = $derived.by(() => {
		if (!report) return [] as ReportComment[];
		let list: ReportComment[] = [...report.comments];
		if (explorerExcludeHosts) list = list.filter((c) => !c.is_seed);
		// OR: keep statements matching ANY selected theme.
		if (selectedThemes.length > 0) {
			list = list.filter((c) => selectedThemes.some((t) => c.topics?.includes(t)));
		}
		return list.sort((a, b) => totalVotes(b) - totalVotes(a));
	});

	// "All Statements" shows everything by default (it's last, so length doesn't
	// disrupt) but the reveal is still reversible. Low-quality rows split out as in
	// the other tables.
	let explorerExpanded = $state(true);
	let showLowQuality = $state(false);
	const explorerMain = $derived(explorerStatements.filter((c) => !isLowQuality(c)));
	const explorerLowQuality = $derived(explorerStatements.filter((c) => isLowQuality(c)));
	const explorerTotal = $derived(explorerMain.length + explorerLowQuality.length);

	/** Set the theme filter and mirror it into ?theme=a,b (shallow — no history spam). */
	function setThemes(next: string[]) {
		selectedThemes = next;
		const url = new URL(page.url);
		if (next.length) url.searchParams.set('theme', next.join(','));
		else url.searchParams.delete('theme');
		replaceState(url, {});
	}

	/** Chip bar: add/remove one theme from the OR-combined filter. */
	function toggleTheme(theme: string) {
		setThemes(
			selectedThemes.includes(theme)
				? selectedThemes.filter((t) => t !== theme)
				: [...selectedThemes, theme]
		);
	}

	/** Themes card: replace the filter with just this theme and scroll to the table. */
	async function focusTheme(theme: string) {
		setThemes([theme]);
		await tick();
		document
			.getElementById(ALL_STATEMENTS_ID)
			?.scrollIntoView({ behavior: 'smooth', block: 'start' });
	}

	// Arriving via a ?theme= deep-link: jump to the (already filtered) table.
	onMount(() => {
		if (selectedThemes.length) {
			document.getElementById(ALL_STATEMENTS_ID)?.scrollIntoView({ block: 'start' });
		}
	});

	/** Shared theme-picker wiring for a row (disabled until the aux row exists). */
	function pickerFor(tid: number) {
		return {
			availableThemes,
			disabled: !auxByTid[tid],
			onAddTheme: (theme: string) => addThemeFor(tid, theme),
			onRemoveTheme: (theme: string) => removeThemeFor(tid, theme)
		};
	}

	/**
	 * Persist a picker edit. Optimistic + roll-back on failure, then invalidate so
	 * the parent load re-seeds `statementAux`. No aux row means the statement hasn't
	 * been backfilled yet — the picker is disabled for those, so this only fires
	 * for taggable rows.
	 */
	async function addThemeFor(tid: number, theme: string) {
		const row = auxByTid[tid];
		if (!row || row.themes.includes(theme)) return;
		const prevThemes = row.themes;
		auxByTid = { ...auxByTid, [tid]: { ...row, themes: [...prevThemes, theme] } };
		try {
			const updated = await apiClient.PolisAddStatementAuxTheme(
				{ theme },
				{ params: { id: row.id } }
			);
			auxByTid = { ...auxByTid, [tid]: updated };
			await invalidate('polis:statement-aux');
		} catch (e) {
			console.error('PolisAddStatementAuxTheme failed', e);
			auxByTid = { ...auxByTid, [tid]: { ...row, themes: prevThemes } };
			notifications.send({ priority: 'ERROR', message: 'Failed to add theme' });
		}
	}

	async function removeThemeFor(tid: number, theme: string) {
		const row = auxByTid[tid];
		if (!row || !row.themes.includes(theme)) return;
		const prevThemes = row.themes;
		auxByTid = {
			...auxByTid,
			[tid]: { ...row, themes: prevThemes.filter((t) => t !== theme) }
		};
		try {
			const updated = await apiClient.PolisRemoveStatementAuxTheme(
				{ theme },
				{ params: { id: row.id } }
			);
			auxByTid = { ...auxByTid, [tid]: updated };
			await invalidate('polis:statement-aux');
		} catch (e) {
			console.error('PolisRemoveStatementAuxTheme failed', e);
			auxByTid = { ...auxByTid, [tid]: { ...row, themes: prevThemes } };
			notifications.send({ priority: 'ERROR', message: 'Failed to remove theme' });
		}
	}

	// --- CSV export (inlined; one row per comment, columns match the UI) ---
	/** Wrap a value for CSV: quote, double internal quotes, normalize newlines. */
	function csvField(value: unknown): string {
		if (value === null || value === undefined) return '';
		const s = String(value).replace(/\r\n|\r/g, '\n');
		return `"${s.replace(/"/g, '""')}"`;
	}

	/** Sorted union of every theme appearing on any statement in the report. */
	function collectThemes(comments: ReportComment[]): string[] {
		const set = new Set<string>();
		for (const c of comments) {
			for (const t of c.topics ?? []) set.add(t);
		}
		return [...set].sort();
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
		<div class="flex flex-wrap items-end justify-between gap-4">
			<div class="flex flex-wrap gap-6">
				{#each [{ label: 'Total Statements', value: stats.totalStatements }, { label: 'Themes', value: themes.length }, { label: 'Areas of Consensus', value: consensus.length }] as s (s.label)}
					<div class="flex flex-col gap-1">
						<span class="text-foreground text-2xl font-bold tabular-nums"
							>{s.value}</span
						>
						<span class="text-muted-foreground text-sm">{s.label}</span>
					</div>
				{/each}
			</div>
			<Button onclick={handleDownloadCsv} class="w-full sm:w-auto">
				<Download class="size-4" />
				Download CSV
			</Button>
		</div>

		<!-- ===== Themes card ===== -->
		<Card.Root
			class="hover:border-muted-foreground/40 rounded-[20px] p-0 shadow-sm transition-colors duration-200"
		>
			<header class="flex items-start justify-between gap-4 px-8 pt-8">
				<div>
					<h2 class="text-foreground text-lg font-semibold">Themes</h2>
					<p class="text-foreground/70 mt-2 text-base font-medium">
						Click a theme to see all of the statements associated with it.
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
						<ThemeBar
							summary={t}
							barMax={maxThemeCount}
							onclick={() => focusTheme(t.theme)}
						/>
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

		<!-- ===== Areas of Consensus ===== -->
		<StatementSection
			title="Areas of Consensus"
			count={consensusFiltered.length}
			countAccent="consensus"
			description="with greater than 80% agreement across all groups."
			metricLabel="Min Agree"
			groupCount={report.groups.length}
			total={consensusMain.length}
			collapsedCount={COLLAPSED_ROWS}
			lowQualityCount={consensusLow.length}
			bind:expanded={consensusExpanded}
			bind:showLowQuality={consensusShowLow}
			bind:excludeHosts={consensusExcludeHosts}
			bind:excludePasses={consensusExcludePasses}
		>
			{#if consensusMain.length === 0}
				<p class="text-muted-foreground col-span-full px-4 py-6 text-base italic">
					No consensus statements yet.
				</p>
			{:else}
				{#each consensusExpanded ? consensusMain : consensusMain.slice(0, COLLAPSED_ROWS) as c, i (c.tid)}
					<StatementRow
						index={i + 1}
						comment={c}
						groups={report.groups}
						variant="consensus"
						excludePasses={consensusExcludePasses}
						picker={pickerFor(c.tid)}
					/>
				{/each}
			{/if}

			{#snippet lowQuality()}
				{#each consensusLow as c, i (c.tid)}
					<StatementRow
						index={i + 1}
						comment={c}
						groups={report.groups}
						variant="consensus"
						excludePasses={consensusExcludePasses}
						picker={pickerFor(c.tid)}
					/>
				{/each}
			{/snippet}
		</StatementSection>

		<!-- ===== Areas of Difference ===== -->
		<StatementSection
			title="Areas of Difference"
			count={differencesFiltered.length}
			countAccent="difference"
			description="with greater than 30% difference across the groups."
			metricLabel="Difference"
			groupCount={report.groups.length}
			total={differencesMain.length}
			collapsedCount={COLLAPSED_ROWS}
			lowQualityCount={differencesLow.length}
			bind:expanded={differencesExpanded}
			bind:showLowQuality={differencesShowLow}
			bind:excludeHosts={differencesExcludeHosts}
			bind:excludePasses={differencesExcludePasses}
		>
			{#if differencesMain.length === 0}
				<p class="text-muted-foreground col-span-full px-4 py-6 text-base italic">
					No clear differences yet.
				</p>
			{:else}
				{#each differencesExpanded ? differencesMain : differencesMain.slice(0, COLLAPSED_ROWS) as c, i (c.tid)}
					<StatementRow
						index={i + 1}
						comment={c}
						groups={report.groups}
						variant="difference"
						excludePasses={differencesExcludePasses}
						picker={pickerFor(c.tid)}
					/>
				{/each}
			{/if}

			{#snippet lowQuality()}
				{#each differencesLow as c, i (c.tid)}
					<StatementRow
						index={i + 1}
						comment={c}
						groups={report.groups}
						variant="difference"
						excludePasses={differencesExcludePasses}
						picker={pickerFor(c.tid)}
					/>
				{/each}
			{/snippet}
		</StatementSection>

		<!-- Areas of Uncertainty is deferred. Ships as Consensus + Difference only. -->

		<!-- ===== All Statements ===== -->
		<StatementSection
			id={ALL_STATEMENTS_ID}
			title="All Statements"
			count={explorerTotal}
			countAccent="all"
			description="in total. Use labels below to filter by theme."
			metricLabel="Count"
			groupCount={report.groups.length}
			total={explorerMain.length}
			collapsedCount={COLLAPSED_ROWS}
			lowQualityCount={explorerLowQuality.length}
			bind:expanded={explorerExpanded}
			bind:showLowQuality
			bind:excludeHosts={explorerExcludeHosts}
			bind:excludePasses={explorerExcludePasses}
		>
			{#snippet headerAction()}
				<Button size="sm" onclick={handleDownloadCsv}>
					<Download class="size-4" />
					Download CSV
				</Button>
			{/snippet}

			{#snippet toolbar()}
				<div class="flex flex-wrap gap-2">
					{#each themes as t (t.theme)}
						<ThemeChip
							label={t.theme}
							variant="primary"
							selected={selectedThemes.includes(t.theme)}
							onclick={() => toggleTheme(t.theme)}
						/>
					{/each}
					{#if themes.length === 0}
						<span class="text-muted-foreground text-sm italic">No themes yet.</span>
					{/if}
				</div>
			{/snippet}

			{#if explorerMain.length === 0}
				<p class="text-muted-foreground col-span-full px-4 py-6 text-base italic">
					No statements match the current filters.
				</p>
			{:else}
				{#each explorerExpanded ? explorerMain : explorerMain.slice(0, COLLAPSED_ROWS) as c, i (c.tid)}
					<StatementRow
						index={i + 1}
						comment={c}
						groups={report.groups}
						variant={classifyStatement(c, report.groups, {
							excludePasses: explorerExcludePasses
						})}
						excludePasses={explorerExcludePasses}
						picker={pickerFor(c.tid)}
					/>
				{/each}
			{/if}

			{#snippet lowQuality()}
				{#each explorerLowQuality as c, i (c.tid)}
					<StatementRow
						index={i + 1}
						comment={c}
						groups={report.groups}
						variant={classifyStatement(c, report.groups, {
							excludePasses: explorerExcludePasses
						})}
						excludePasses={explorerExcludePasses}
						picker={pickerFor(c.tid)}
					/>
				{/each}
			{/snippet}
		</StatementSection>
	</div>
{/if}
