<!--
	@component One statement in the area-of-consensus table: the statement text
	above a stack of vote bars (OVERALL + one per opinion group).

	Dumb: takes the raw report comment + the group list and computes the of-members
	bars (`computeMemberVoteBars`). N group bars, not a fixed pair.
-->
<script lang="ts">
	import type { ReportComment, ReportGroup } from '$lib/tools/polis/reportTypes';
	import { computeMemberVoteBars } from '$lib/tools/polis/report';
	import VoteBar from './VoteBar.svelte';

	type Props = {
		comment: ReportComment;
		groups: ReportGroup[];
	};

	let { comment, groups }: Props = $props();

	const bars = $derived(computeMemberVoteBars(comment, groups));
</script>

<div class="bg-card flex flex-col gap-3 rounded-md p-4">
	<p class="text-foreground text-xl font-medium">{comment.text}</p>
	<div class="flex flex-col gap-2">
		<VoteBar {...bars.overall} />
		{#each bars.groups as g (g.label)}
			<VoteBar {...g} />
		{/each}
	</div>
</div>
