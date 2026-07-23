<!--
	@component The "Area of consensus" section: the statement explorer from the
	new design. A legend, author-type filter chips (seed / participant / all), and
	a numbered list of statements each shown as a StatementVoteBlock (OVERALL + N
	group bars), collapsed to a preview with a "See all N" expand.

	Dumb: takes the comments + groups and an optional CSV handler. Filtering and
	collapse are local view state, not data fetching.
-->
<script lang="ts">
	import type { ReportComment, ReportGroup } from '$lib/tools/polis/reportTypes';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Download, ChevronDown } from '@lucide/svelte';
	import StatementVoteBlock from './StatementVoteBlock.svelte';

	type AuthorFilter = 'all' | 'seed' | 'participant';

	type Props = {
		comments: ReportComment[];
		groups: ReportGroup[];
		/** Wired from PolisInsights; omit to hide the CSV action. */
		onDownloadCsv?: () => void;
	};

	let { comments, groups, onDownloadCsv }: Props = $props();

	const COLLAPSED_ROWS = 4;
	let filter = $state<AuthorFilter>('all');
	let expanded = $state(false);

	const filtered = $derived(
		filter === 'seed'
			? comments.filter((c) => c.is_seed)
			: filter === 'participant'
				? comments.filter((c) => !c.is_seed)
				: comments
	);
	const visible = $derived(expanded ? filtered : filtered.slice(0, COLLAPSED_ROWS));

	const chips: { value: AuthorFilter; label: string }[] = [
		{ value: 'seed', label: 'Seed statement' },
		{ value: 'participant', label: 'Participant statement' },
		{ value: 'all', label: 'All' }
	];

	// Legend swatches map to the same tokens VoteBar renders with.
	const legend = [
		{ label: '% Agreed', color: 'var(--vote-agreed)', border: false },
		{ label: '% Disagreed', color: 'var(--vote-disagreed)', border: false },
		{ label: '% Passed', color: 'var(--vote-passed)', border: false },
		{ label: '% Not voted', color: 'var(--vote-not-voted)', border: true }
	];
</script>

<Card.Root class="rounded-[20px] p-0 shadow-sm">
	<header class="flex items-start justify-between gap-4 px-8 pt-8">
		<div>
			<h2 class="text-foreground text-lg font-semibold">Area of consensus</h2>
			<p class="text-foreground/70 mt-2 text-base font-medium">
				Where participants agree, disagree, or split by opinion group.
			</p>
		</div>
		{#if onDownloadCsv}
			<Button size="sm" onclick={onDownloadCsv}>
				<Download class="size-4" />
				Download CSV
			</Button>
		{/if}
	</header>

	<!-- Legend -->
	<div
		class="text-foreground flex flex-wrap items-center gap-x-4 gap-y-2 px-8 pt-4 text-xs font-medium"
	>
		{#each legend as l (l.label)}
			<span class="flex items-center gap-1.5">
				<span
					class="size-3 rounded-full"
					class:border={l.border}
					class:border-border={l.border}
					style="background: {l.color};"
				></span>
				{l.label}
			</span>
		{/each}
	</div>

	<!-- Author-type filter -->
	<div class="flex flex-wrap gap-2 px-8 pt-4">
		{#each chips as c (c.value)}
			<button
				type="button"
				onclick={() => (filter = c.value)}
				class="rounded-full px-3 py-0.5 text-sm font-medium transition-colors {filter ===
				c.value
					? 'bg-primary text-primary-foreground'
					: 'bg-accent text-accent-foreground hover:bg-accent/70'}"
			>
				{c.label}
			</button>
		{/each}
	</div>

	<!-- Statement list -->
	<div class="flex flex-col px-8 pt-4 pb-2">
		{#if filtered.length === 0}
			<p class="text-muted-foreground py-6 text-base italic">
				No statements match the current filter.
			</p>
		{:else}
			{#each visible as c, i (c.tid)}
				<div class="border-border flex items-start gap-5 border-b py-2">
					<span
						class="text-muted-foreground w-6 shrink-0 pt-5 text-right text-xs font-medium tabular-nums"
					>
						{i + 1}
					</span>
					<div class="min-w-0 flex-1">
						<StatementVoteBlock comment={c} {groups} />
					</div>
				</div>
			{/each}

			{#if filtered.length > COLLAPSED_ROWS}
				<button
					type="button"
					onclick={() => (expanded = !expanded)}
					class="text-foreground/70 hover:text-foreground flex w-full items-center justify-center gap-2 py-4 text-base transition-colors"
				>
					{expanded ? 'Show fewer' : `See all ${filtered.length} statements`}
					<ChevronDown
						class={`text-primary size-4 transition-transform ${expanded ? 'rotate-180' : ''}`}
					/>
				</button>
			{/if}
		{/if}
	</div>
</Card.Root>
