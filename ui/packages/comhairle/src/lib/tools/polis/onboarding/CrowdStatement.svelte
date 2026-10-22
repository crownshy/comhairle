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
	const othersVoted = $derived(bar.voters > 1);
</script>

<div class={cn('flex w-full flex-col gap-4 rounded-2xl border p-4 sm:p-6', source.cardClass)}>
	<StatementSourceLabel {source} />
	<p class="text-card-foreground text-lg leading-snug sm:text-xl">{statement.text}</p>
	<div class="flex flex-col gap-2">
		<VoteBar agreed={bar.agreed} disagreed={bar.disagreed} passed={bar.passed} notVoted={0} />
		<!-- Words under each end so the bar reads without telling the colours apart. -->
		<div class="flex justify-between gap-3 text-base" aria-hidden="true">
			<span class="flex items-center gap-1.5">
				<span class="size-2.5 rounded-full" style="background: var(--vote-agreed);"></span>
				{m.polis_agree()}
				{Math.round(bar.agreed)}%
			</span>
			<span class="text-muted-foreground flex items-center gap-1.5">
				<span class="size-2.5 rounded-full" style="background: var(--vote-passed);"></span>
				{m.polis_neutral()}
				{Math.round(bar.passed)}%
			</span>
			<span class="flex items-center gap-1.5">
				<span class="size-2.5 rounded-full" style="background: var(--vote-disagreed);"
				></span>
				{m.polis_disagree()}
				{Math.round(bar.disagreed)}%
			</span>
		</div>
	</div>
	<p class="text-muted-foreground flex flex-wrap justify-between gap-x-4 text-base">
		<span>{m.polis_onboarding_your_vote({ vote: voteLabel[ownVote]() })}</span>
		<span>
			{othersVoted
				? m.polis_onboarding_vote_count({ count: bar.voters })
				: m.polis_onboarding_first_to_vote()}
		</span>
	</p>
</div>
