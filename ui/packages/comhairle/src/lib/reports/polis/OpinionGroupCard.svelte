<!--
	@component One opinion group in the "Opinion groups" section: its size, an
	(AI-generated) name + summary, and its representative statements shown as vote
	blocks (OVERALL + one bar per group), collapsed to a preview with a "See all N"
	expand.

	Size line = this group's `total_members` and its share of all group members.
	Representative comments carry only `{ text, tid }`, so each tid is resolved back
	to the full report comment (from `comments`) to render the vote bars; any that
	don't resolve are skipped.

	The name + summary are NOT in the report payload today (Polis carries neither).
	They come in via the optional `aiSummary` prop and render behind an "AI Generated"
	badge. The live report leaves it undefined for now (so the whole block is hidden);
	Storybook passes it to demo the eventual AI-generated version. Once we have a source
	(likely an on-demand "generate group summaries" agent), the caller fills this in.

	Dumb: takes the group, an optional AI name/summary, and the full comment list.
	Collapse is local view state.
-->
<script lang="ts">
	import type { ReportComment, ReportGroup } from '$lib/tools/polis/reportTypes';
	import { groupLabel } from '$lib/tools/polis/report';
	import * as Card from '$lib/components/ui/card';
	import { ChevronDown, Sparkles } from '@lucide/svelte';
	import StatementVoteBlock from './StatementVoteBlock.svelte';

	type Props = {
		group: ReportGroup;
		/** All report comments, used to resolve representative-comment tids to bars. */
		comments: ReportComment[];
		/** All opinion groups, so each statement block draws OVERALL + one bar per group. */
		groups: ReportGroup[];
		/**
		 * AI-generated name + summary for this group. Omitted in the live report today
		 * (the block is hidden); Storybook passes it to demo the generated version.
		 */
		aiSummary?: { name: string; summary: string };
		/** Frozen-snapshot render (ADR-0012): show every statement, drop the dead expand toggle. */
		frozen?: boolean;
	};

	let { group, comments, groups, aiSummary, frozen = false }: Props = $props();

	const totalParticipants = $derived(groups.reduce((sum, g) => sum + g.total_members, 0));

	const COLLAPSED_ROWS = 3;
	let expanded = $state(false);

	const label = $derived(groupLabel(group.group_id));
	const sharePct = $derived(
		totalParticipants > 0 ? Math.round((group.total_members / totalParticipants) * 100) : 0
	);

	// Resolve each representative comment's tid to its full report comment so the
	// vote block has the group_votes to draw. Skip any that no longer resolve.
	const byTid = $derived(new Map(comments.map((c) => [c.tid, c])));
	const repComments = $derived(
		group.representative_comments
			.map((r) => byTid.get(r.tid))
			.filter((c): c is ReportComment => c !== undefined)
	);
	const visible = $derived(
		frozen || expanded ? repComments : repComments.slice(0, COLLAPSED_ROWS)
	);

	// Legend swatches map to the same tokens VoteBar renders with. `border` flags the
	// not-voted swatch so its (near-white) fill stays visible on the card.
	const legend = [
		{ label: '%Agreed', color: 'var(--vote-agreed)', border: false },
		{ label: '%Disagreed', color: 'var(--vote-disagreed)', border: false },
		{ label: '%Passed', color: 'var(--vote-passed)', border: false },
		{ label: '%not voted', color: 'var(--vote-not-voted)', border: true }
	];
</script>

<Card.Root class="gap-4 rounded-md p-0 px-4 py-3.5 shadow-none">
	<!-- Size line -->
	<div class="flex items-center gap-2">
		<span class="text-primary text-xl font-bold">Group {label}</span>
		<span class="text-muted-foreground text-xs" aria-hidden="true">·</span>
		<span class="text-muted-foreground text-sm">
			{group.total_members} members ({sharePct}%)
		</span>
	</div>

	<!-- Name + AI summary (hidden until an AI source populates it) -->
	{#if aiSummary}
		<div class="flex flex-col gap-4">
			<div class="flex flex-wrap items-center gap-2">
				<h3 class="text-foreground text-2xl font-semibold">{aiSummary.name}</h3>
				<span
					class="bg-accent border-border text-primary flex items-center gap-1 rounded-full border px-2 py-0.5 text-xs font-medium"
				>
					<Sparkles class="size-4" />
					AI Generated
				</span>
			</div>
			<div class="bg-accent border-border rounded-[10px] border p-4">
				<div class="flex items-start gap-2.5">
					<Sparkles class="text-primary mt-0.5 size-4 shrink-0" />
					<p class="text-primary text-base leading-7">{aiSummary.summary}</p>
				</div>
			</div>
		</div>
	{/if}

	<!-- Key statements -->
	<p class="text-muted-foreground text-sm">KEY STATEMENTS</p>

	{#if repComments.length === 0}
		<p class="text-muted-foreground pb-2 text-base italic">
			No representative statements for this group yet.
		</p>
	{:else}
		<!-- Legend -->
		<div class="text-card-foreground flex flex-wrap items-center gap-2 text-xs font-medium">
			{#each legend as l, i (l.label)}
				{#if i > 0}
					<span class="text-secondary-foreground" aria-hidden="true">·</span>
				{/if}
				<span class="flex items-center gap-1.5">
					<span
						class="size-3 rounded-sm"
						class:border={l.border}
						class:border-border={l.border}
						style="background: {l.color};"
					></span>
					{l.label}
				</span>
			{/each}
		</div>

		<!-- Statement list -->
		<div class="flex flex-col">
			{#each visible as c, i (c.tid)}
				<div class="border-border flex items-center gap-5 border-b py-2">
					<span
						class="text-muted-foreground w-6 shrink-0 text-right text-xs font-medium tabular-nums"
					>
						{i + 1}
					</span>
					<div class="min-w-0 flex-1">
						<StatementVoteBlock comment={c} {groups} />
					</div>
				</div>
			{/each}
		</div>

		{#if !frozen && repComments.length > COLLAPSED_ROWS}
			<button
				type="button"
				onclick={() => (expanded = !expanded)}
				class="bg-muted text-accent-foreground hover:bg-muted/70 -mx-4 -mb-3.5 flex items-center justify-center gap-2 rounded-b-md py-3 text-base transition-colors"
			>
				{expanded ? 'Show fewer' : `See all ${repComments.length} statements`}
				<ChevronDown
					class={`text-primary size-4 transition-transform ${expanded ? 'rotate-180' : ''}`}
				/>
			</button>
		{/if}
	{/if}
</Card.Root>
