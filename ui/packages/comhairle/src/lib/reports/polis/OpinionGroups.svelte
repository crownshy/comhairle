<!--
	@component The "Opinion groups" section from the new Polis report design: a
	heading, a row of group nav chips, then one OpinionGroupCard per opinion group
	(size + AI name/summary + representative statements).

	The chips select which group to review: clicking one focuses that group's card;
	clicking it again clears the filter and shows every group. All groups are shown
	by default.

	The AI group name + summary aren't in the report payload yet, so the live report
	renders each card without them (the OpinionGroupCard hides that block when it has
	no `aiSummary`). When we have a source for it (likely an on-demand agent that reads
	each group's statements), hand each card its `aiSummary`. Storybook still demos the
	generated version at the OpinionGroupCard level.

	Dumb: takes the comments + groups. Chip selection is local view state.
-->
<script lang="ts">
	import type { ReportComment, ReportGroup } from '$lib/tools/polis/reportTypes';
	import { groupLabel } from '$lib/tools/polis/report';
	import OpinionGroupCard from './OpinionGroupCard.svelte';

	type Props = {
		comments: ReportComment[];
		groups: ReportGroup[];
	};

	let { comments, groups }: Props = $props();

	// null = show every group; a group_id = focus that one.
	let selectedGroupId = $state<number | null>(null);
	const shownGroups = $derived(
		selectedGroupId === null ? groups : groups.filter((g) => g.group_id === selectedGroupId)
	);

	function toggle(groupId: number) {
		selectedGroupId = selectedGroupId === groupId ? null : groupId;
	}
</script>

<section class="flex flex-col gap-4">
	<header class="flex flex-col gap-2 py-2">
		<h2 class="text-foreground text-xl font-bold">Opinion groups</h2>
		<p class="text-muted-foreground text-sm font-medium">
			Click on the groups below to review various groups.
		</p>
	</header>

	<!-- Group nav chips -->
	<div class="flex flex-wrap gap-2">
		{#each groups as g (g.group_id)}
			<button
				type="button"
				onclick={() => toggle(g.group_id)}
				aria-pressed={selectedGroupId === g.group_id}
				class="rounded-full px-2 py-0.5 text-xs font-medium transition-colors {selectedGroupId ===
					null || selectedGroupId === g.group_id
					? 'bg-primary text-primary-foreground'
					: 'bg-accent text-accent-foreground hover:bg-accent/70'}"
			>
				Group {groupLabel(g.group_id)}
			</button>
		{/each}
	</div>

	{#each shownGroups as g (g.group_id)}
		<OpinionGroupCard group={g} {comments} {groups} />
	{/each}
</section>
