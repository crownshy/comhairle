<!--
	@component The "Opinion groups" section from the new Polis report design: a
	heading, a row of group nav chips, then one OpinionGroupCard per opinion group
	(size + AI name/summary + representative statements).

	The chips select which group to review: clicking one focuses that group's card;
	clicking it again clears the filter and shows every group. All groups are shown
	by default.

	The AI name + summary aren't in the report payload yet, so this component holds a
	PLACEHOLDER set (`GROUP_PLACEHOLDERS`, cycled by group index) and hands each card
	its slot. Swap this for the real per-group summaries once the backend provides
	them.

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

	// PLACEHOLDER copy — the report payload carries no group names or summaries yet.
	// Cycled by group index; replace with real AI-generated summaries when available.
	const GROUP_PLACEHOLDERS: { name: string; summary: string }[] = [
		{
			name: 'Progressive Digital Advocates',
			summary:
				'This group strongly believes in the constructive role technology plays in ' +
				"young people's development. They champion digital literacy education, view online " +
				'communities as meaningful social spaces, and tend to support platform-driven ' +
				'innovations that empower rather than restrict youth. Members are generally ' +
				'optimistic about the long-term benefits of social media when guided by thoughtful ' +
				'design and parental engagement, rather than blunt regulatory controls.'
		},
		{
			name: 'Cautious Traditionalists',
			summary:
				'Members of this group express significant concern about the unregulated exposure ' +
				'of young people to social media platforms. They consistently favour in-person ' +
				'interaction over digital engagement and support robust regulatory frameworks, ' +
				'including strict age verification and outright bans for younger children. While not ' +
				'uniformly anti-technology, this group believes current platform designs prioritise ' +
				'engagement over wellbeing.'
		}
	];

	function placeholderFor(index: number): { name: string; summary: string } {
		if (GROUP_PLACEHOLDERS.length === 0) return { name: '', summary: '' };
		return GROUP_PLACEHOLDERS[index % GROUP_PLACEHOLDERS.length];
	}

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
		<OpinionGroupCard group={g} {comments} {groups} placeholder={placeholderFor(g.group_id)} />
	{/each}
</section>
