<script lang="ts">
	import type { ReportComment, ReportGroup } from '$lib/types/report';
	import { computeGroupVotePercents, computeOverallVotePercents } from '$lib/types/report';
	import VoteBarChart from './VoteBarChart.svelte';

	interface Props {
		comments: ReportComment[];
		groups: ReportGroup[];
		highlightedTid?: number | null;
		onSelectComment?: (comment: ReportComment) => void;
	}

	let { comments, groups, highlightedTid = null, onSelectComment }: Props = $props();

	const totalParticipants = $derived(groups.reduce((sum, g) => sum + g.members.length, 0));
</script>

<div class="w-full overflow-x-auto">
	<table class="w-full text-left text-sm">
		<thead>
			<tr class="border-b-2 border-gray-900">
				<th class="pr-4 pb-2 font-bold text-gray-950 uppercase">Statement</th>
				<th class="w-40 px-2 pb-2 font-bold text-gray-950 uppercase">Overall</th>
				{#each groups as group (group.group_id)}
					{@const label = String.fromCharCode(65 + group.group_id)}
					<th class="w-40 px-2 pb-2 font-bold text-gray-950 uppercase">
						{label}
						<span class="text-xs font-normal text-gray-500">{group.members.length}</span
						>
					</th>
				{/each}
			</tr>
		</thead>
		<tbody>
			{#each comments as comment (comment.tid)}
				{@const overall = computeOverallVotePercents(comment, totalParticipants)}
				{@const groupPercents = computeGroupVotePercents(comment, groups)}
				<tr
					class="cursor-pointer border-b border-gray-100 transition-colors hover:bg-gray-50"
					class:bg-yellow-50={highlightedTid === comment.tid}
					onclick={() => onSelectComment?.(comment)}
				>
					<td class="py-3 pr-4">
						<div class="flex items-start gap-2">
							<span class="shrink-0 text-xs font-medium text-gray-400"
								>{comment.tid}</span
							>
							<span class="line-clamp-2 text-sm text-gray-700"
								>{comment.text.trim()}</span
							>
						</div>
					</td>
					<td class="px-2 py-3">
						<VoteBarChart data={overall} height={10} showLabels={false} />
						<div class="mt-0.5 flex gap-1 text-[10px] leading-none">
							<span class="text-teal-700">{Math.round(overall.agreed)}%</span>
							<span class="text-red-500">{Math.round(overall.disagreed)}%</span>
							<span class="text-gray-500">{Math.round(overall.passed)}%</span>
						</div>
					</td>
					{#each groupPercents as gp (gp.group_id)}
						<td class="px-2 py-3">
							<VoteBarChart data={gp} height={10} showLabels={false} />
							<div class="mt-0.5 flex gap-1 text-[10px] leading-none">
								<span class="text-teal-700">{Math.round(gp.agreed)}%</span>
								<span class="text-red-500">{Math.round(gp.disagreed)}%</span>
								<span class="text-gray-500">{Math.round(gp.passed)}%</span>
							</div>
						</td>
					{/each}
				</tr>
			{/each}
		</tbody>
	</table>
</div>
