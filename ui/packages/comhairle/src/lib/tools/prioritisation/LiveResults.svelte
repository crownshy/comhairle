<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { aggregatePoll } from './aggregation';
	import type { PrioritisationStore } from './store.svelte';

	let { store }: { store: PrioritisationStore } = $props();

	let aggregates = $derived(aggregatePoll(store.poll, store.submissions));
	let questions = $derived(store.poll.toolConfig.questions);

	function proposal(id: string) {
		return store.poll.proposals.find((p) => p.id === id);
	}

	function num(n: number, digits = 2): string {
		return Number.isFinite(n) ? n.toFixed(digits) : '—';
	}
</script>

<div class="flex flex-col gap-4">
	<Card.Root>
		<Card.Header>
			<Card.Title>Realtime results: {store.poll.title || 'Untitled'}</Card.Title>
			<Card.Description>
				{store.submissions.length} submission{store.submissions.length === 1 ? '' : 's'}.
				The same questions are asked of every proposal.
			</Card.Description>
		</Card.Header>
		<Card.Content class="flex flex-col gap-3">
			{#if questions.length === 0}
				<p class="text-muted-foreground text-sm">
					No questions configured yet — define them in the Questions tab.
				</p>
			{/if}

			{#each aggregates as agg (agg.proposalId)}
				<div class="rounded-md border p-4">
					<div class="mb-2 font-medium">
						Proposal {proposal(agg.proposalId)?.order}: {proposal(agg.proposalId)
							?.title || 'Untitled'}
					</div>

					<div class="flex flex-col gap-3">
						{#each questions as q (q.id)}
							<div>
								<div class="text-muted-foreground text-xs">
									Q{q.order}: {q.prompt || ''}
								</div>
								{#if agg.perQuestion[q.id]?.kind === 'numeric'}
									<div class="flex items-center gap-3">
										<div class="text-sm">
											mean
											<span class="font-mono"
												>{num(agg.perQuestion[q.id].mean)}</span
											>
										</div>
										<div class="text-muted-foreground text-xs">
											min {num(agg.perQuestion[q.id].min)} · max {num(
												agg.perQuestion[q.id].max
											)} · var {num(agg.perQuestion[q.id].variance, 3)} · n {agg
												.perQuestion[q.id].n}
										</div>
									</div>
								{:else if agg.perQuestion[q.id]?.kind === 'text'}
									<div class="text-muted-foreground text-xs">
										{agg.perQuestion[q.id].n} text response{agg.perQuestion[
											q.id
										].n === 1
											? ''
											: 's'}
									</div>
									{#if agg.perQuestion[q.id].samples.length > 0}
										<ul class="ml-4 list-disc text-sm">
											{#each agg.perQuestion[q.id].samples.slice(0, 3) as s, i (i)}
												<li class="truncate">{s}</li>
											{/each}
										</ul>
									{/if}
								{/if}
							</div>
						{/each}
					</div>
				</div>
			{/each}

			{#if aggregates.length === 0 && questions.length > 0}
				<p class="text-muted-foreground text-sm">
					Waiting for participants — add proposals to begin collecting answers.
				</p>
			{/if}
		</Card.Content>
	</Card.Root>
</div>
