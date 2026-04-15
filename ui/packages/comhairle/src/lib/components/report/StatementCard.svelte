<script lang="ts">
	import type { ReportComment, ReportGroup } from '$lib/types/report';
	import { computeGroupVotePercents, computeOverallVotePercents } from '$lib/utils/report';
	import InfoIcon from '@lucide/svelte/icons/info';
	import VoteBarChart from './VoteBarChart.svelte';

	interface Props {
		comment: ReportComment;
		groups: ReportGroup[];
		totalParticipants?: number;
	}

	let { comment, groups, totalParticipants = 0 }: Props = $props();

	const groupPercents = $derived(computeGroupVotePercents(comment, groups));
	const overall = $derived(
		computeOverallVotePercents(
			comment,
			totalParticipants || groups.reduce((s, g) => s + g.total_members, 0)
		)
	);
	const allRows = $derived([overall, ...groupPercents]);

	const bridgingLabel = $derived.by(() => {
		const c = comment.group_informed_consensus;
		if (c >= 0.75) return 'High';
		if (c >= 0.5) return 'Medium';
		if (c >= 0.25) return 'Low';
		return 'Very low';
	});

	const bridgingColor = $derived(
		bridgingLabel === 'High'
			? 'text-teal-600'
			: bridgingLabel === 'Medium'
				? 'text-amber-500'
				: 'text-gray-500'
	);

	let showSeedTooltip = $state(false);
</script>

<div
	class="{comment.is_seed
		? 'pt-10'
		: ''} bg-card relative w-full rounded border border-gray-200 p-3"
>
	<!-- Seed badge -->
	{#if comment.is_seed}
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div class="absolute top-0 -left-px z-10 inline-flex w-full items-center">
			<div
				class="rounded-br-nav inline-flex items-center gap-1 rounded-tl-sm bg-slate-700 p-2"
				onmouseenter={() => (showSeedTooltip = true)}
				onmouseleave={() => (showSeedTooltip = false)}
			>
				<span class="text-center text-xs leading-4 font-medium text-white"
					>conversation starter</span
				>
				<InfoIcon class="h-2.5 w-2.5 text-white" />
			</div>
			{#if showSeedTooltip}
				<div class="text-muted-foreground z-20 px-3 py-2 text-xs">
					This statement was submitted by the conversation organiser.
				</div>
			{/if}
		</div>
	{/if}

	<!-- Main layout -->
	<div class="flex flex-col sm:flex-row" style="padding-top: {comment.is_seed ? '30px' : '0'};">
		<!-- Left: statement text + vote bars -->
		<div class="flex min-w-0 flex-1 flex-col">
			<!-- Statement text -->
			<div class="flex items-center gap-2 px-4 pt-3 pb-3">
				<div
					class="text-card-foreground flex-1 text-base leading-6 font-semibold sm:text-2xl sm:leading-7"
				>
					{comment.text.trim()}
				</div>
			</div>

			<!-- Vote bars (LayerCake) -->
			<div class="px-4 pb-3">
				<div class="flex flex-col gap-0.5">
					{#each allRows as row (row.label)}
						<div class="flex items-center gap-1">
							<div
								class="text-card-foreground w-14 shrink-0 text-xs leading-4 font-medium"
							>
								{row.label}
							</div>
							<div class="flex-1">
								<VoteBarChart data={row} />
							</div>
						</div>
					{/each}
				</div>

				<!-- Legend -->
				<div class="mt-2 flex flex-col gap-1 sm:mt-1 sm:flex-row sm:items-center sm:gap-0">
					<div class="w-14 shrink-0 text-xs leading-4 font-medium text-gray-600">
						Legend
					</div>
					<div class="flex flex-wrap items-center gap-2 sm:gap-3">
						<div class="flex items-center gap-0.5">
							<div
								class="h-2 w-2 rounded"
								style="background: var(--vote-agreed);"
							></div>
							<div class="text-xs leading-4 font-medium text-gray-600">%Agreed</div>
						</div>
						<div class="flex items-center gap-0.5">
							<div
								class="h-2 w-2 rounded"
								style="background: var(--vote-disagreed);"
							></div>
							<div class="text-xs leading-4 font-medium text-gray-600">
								%Disagreed
							</div>
						</div>
						<div class="flex items-center gap-0.5">
							<div
								class="h-2 w-2 rounded"
								style="background: var(--vote-passed);"
							></div>
							<div class="text-xs leading-4 font-medium text-gray-600">%Passed</div>
						</div>
						<div class="flex items-center gap-0.5">
							<div
								class="h-2 w-2 rounded"
								style="background: var(--vote-not-voted); border: 1px solid var(--vote-not-voted-border);"
							></div>
							<div class="text-xs leading-4 font-medium text-gray-600">
								%Not voted
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>

		<!-- Right: bridging strength + tags -->
		<div class="flex shrink-0 flex-col gap-2 px-4 pt-2 sm:px-0 sm:pt-3 sm:pr-4">
			<!-- Bridging label -->
			<!-- <div>
				<span class="{bridgingColor} text-sm leading-4 font-medium">{bridgingLabel}</span>
				<br />
				<span class="text-[8px] leading-4 font-medium text-gray-600">bridging strength</span
				>
			</div> -->

			<!-- Topic tags -->
			{#if comment.topics?.length}
				<div class="flex flex-wrap gap-1">
					{#each comment.topics as topic (topic)}
						<div
							class="inline-flex items-center gap-0.5 rounded-lg bg-orange-200 px-1 py-0.5"
						>
							<span class="text-center text-[8px] leading-4 font-medium text-gray-950"
								>{topic}</span
							>
						</div>
					{/each}
				</div>
			{/if}

			<!-- Subtopic tags -->
			{#if comment.subtopics?.length}
				<div class="flex flex-wrap items-center gap-1">
					{#each comment.subtopics as subtopic (subtopic)}
						<div class="flex items-center gap-1">
							<div class="h-2 w-2 rounded-full bg-orange-200"></div>
							<div
								class="inline-flex items-center gap-0.5 rounded-lg bg-orange-200 px-1 py-0.5"
							>
								<span
									class="text-center text-[8px] leading-4 font-medium text-gray-950"
									>{subtopic}</span
								>
							</div>
						</div>
					{/each}
				</div>
			{/if}
		</div>
	</div>
</div>
