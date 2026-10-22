<script lang="ts">
	import VoteBar from '$lib/reports/polis/VoteBar.svelte';
	import StatementSourceLabel from '../StatementSourceLabel.svelte';
	import { statementSourceOf } from '../statementSource';
	import { cn } from '$lib/utils';
	import { m } from '$lib/paraglide/messages';
	import { crowdBar, type PracticeStatement, type Vote } from './onboarding';

	type Props = {
		statement: PracticeStatement;
		ownVote: Vote;
	};

	let { statement, ownVote }: Props = $props();

	const source = $derived(statementSourceOf(statement.is_seed));
	const bar = $derived(crowdBar(statement, ownVote));
	const voteLabel = { agree: m.polis_agree, disagree: m.polis_disagree, pass: m.polis_neutral };
	const othersVoted = $derived((bar?.voters ?? 0) > 1);
	const legend = $derived(
		bar && [
			{
				label: m.polis_agree(),
				percent: bar.agreed,
				color: 'var(--vote-agreed)',
				muted: false
			},
			{
				label: m.polis_neutral(),
				percent: bar.passed,
				color: 'var(--vote-passed)',
				muted: true
			},
			{
				label: m.polis_disagree(),
				percent: bar.disagreed,
				color: 'var(--vote-disagreed)',
				muted: false
			}
		]
	);
</script>

<div class={cn('flex w-full flex-col gap-4 rounded-2xl border p-4 sm:p-6', source.cardClass)}>
	<StatementSourceLabel {source} />
	<p class="text-card-foreground text-lg leading-snug sm:text-xl">{statement.text}</p>
	{#if bar}
		<div class="flex flex-col gap-2">
			<VoteBar
				agreed={bar.agreed}
				disagreed={bar.disagreed}
				passed={bar.passed}
				notVoted={0}
			/>
			<!-- Words under each end so the bar reads without telling the colours apart. -->
			<div
				class="flex flex-wrap justify-between gap-x-4 gap-y-1 text-base whitespace-nowrap"
				aria-hidden="true"
			>
				{#each legend ?? [] as entry (entry.color)}
					<span
						class={cn(
							'flex items-center gap-1.5',
							entry.muted && 'text-muted-foreground'
						)}
					>
						<span class="size-2.5 rounded-full" style:background={entry.color}></span>
						{entry.label}
						{Math.round(entry.percent)}%
					</span>
				{/each}
			</div>
		</div>
	{/if}
	<p class="text-muted-foreground flex flex-wrap justify-between gap-x-4 text-base">
		<span>{m.polis_onboarding_your_vote({ vote: voteLabel[ownVote]() })}</span>
		<span>
			{#if !bar}
				{m.polis_onboarding_counts_pending()}
			{:else if othersVoted}
				{m.polis_onboarding_vote_count({ count: bar.voters })}
			{:else}
				{m.polis_onboarding_first_to_vote()}
			{/if}
		</span>
	</p>
</div>
