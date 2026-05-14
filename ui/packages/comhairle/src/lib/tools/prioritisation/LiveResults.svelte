<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { aggregatePoll, type QuestionAggregate } from './aggregation';
	import { letterFor } from './types';
	import type { PrioritisationStore } from './store.svelte';

	let { store }: { store: PrioritisationStore } = $props();

	let aggregates = $derived(aggregatePoll(store.poll, store.submissions));

	function proposal(id: string) {
		return store.poll.proposals.find((p) => p.id === id);
	}

	function pct(n: number): string {
		return `${Math.round(n * 100)}%`;
	}
	function num(n: number, digits = 2): string {
		return Number.isFinite(n) ? n.toFixed(digits) : '—';
	}
</script>

<div class="flex flex-col gap-4">
	<Card.Root>
		<Card.Header>
			<Card.Title>Realtime result for poll: {store.poll.title || 'Untitled'}</Card.Title>
			<Card.Description>
				{store.submissions.length} submission{store.submissions.length === 1 ? '' : 's'}.
				Each proposal has its own questions, so cross-proposal ranking is not shown.
			</Card.Description>
		</Card.Header>
		<Card.Content class="flex flex-col gap-3">
			{#each aggregates as agg (agg.proposalId)}
				<div class="rounded-md border p-4">
					<div class="mb-2 flex items-center justify-between">
						<div class="font-medium">
							Proposal {proposal(agg.proposalId)?.order}: {proposal(agg.proposalId)
								?.title || 'Untitled'}
						</div>
					</div>

					<div class="flex flex-col gap-3">
						{#if !proposal(agg.proposalId) || proposal(agg.proposalId).questions.length === 0}
							<p class="text-muted-foreground text-xs">No questions configured.</p>
						{/if}
						{#each proposal(agg.proposalId)?.questions ?? [] as q (q.id)}
							<div>
								<div class="text-muted-foreground text-xs">
									Q{q.order}: {q.prompt || ''}
								</div>
								{#if agg.perQuestion[q.id]?.kind === 'choice' && q.type === 'multiple_choice'}
									<div class="flex flex-col gap-1">
										{#each q.choices as c, i (c.id)}
											<div class="flex items-center gap-2">
												<span
													class="bg-muted flex size-5 items-center justify-center rounded text-xs"
													>{letterFor(i)}</span
												>
												<span class="w-40 truncate text-sm">{c.label}</span>
												<div
													class="bg-muted relative h-3 flex-1 overflow-hidden rounded-full"
												>
													<div
														class="bg-primary absolute inset-y-0 left-0"
														style="width: {agg.perQuestion[q.id]
															.percentages[c.id] * 100}%"
													></div>
												</div>
												<span
													class="text-muted-foreground w-16 text-right text-xs"
													>{agg.perQuestion[q.id].counts[c.id]} ({pct(
														agg.perQuestion[q.id].percentages[c.id]
													)})</span
												>
											</div>
										{/each}
									</div>
								{:else if agg.perQuestion[q.id]?.kind === 'numeric'}
									<div class="flex items-center gap-3">
										<div class="text-sm">
											mean <span class="font-mono"
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

			{#if aggregates.length === 0}
				<p class="text-muted-foreground text-sm">
					Waiting for participants — share the QR code to begin.
				</p>
			{/if}
		</Card.Content>
	</Card.Root>
</div>
