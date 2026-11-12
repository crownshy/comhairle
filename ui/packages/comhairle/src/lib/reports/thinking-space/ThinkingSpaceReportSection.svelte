<script lang="ts">
	import MetricOverviewCard from '$lib/reports/MetricOverviewCard.svelte';
	import * as Table from '$lib/components/ui/table';
	import ThinkingSpaceThemes from './ThinkingSpaceThemes.svelte';
	import type { ThinkingSpaceEmbedData } from './embedData';

	/** Renders one embeddable Thinking Space section block from its public report data. */
	let { data }: { data: ThinkingSpaceEmbedData } = $props();
</script>

{#if data.componentType === 'thinkingspace-key-stats'}
	<div class="flex flex-wrap gap-4">
		<MetricOverviewCard
			superText="Participants"
			metric={data.stats.participants}
			subText="shared their summary"
		/>
		<MetricOverviewCard
			superText="Follow-up questions"
			metric={Number(data.stats.followUpsPerParticipant.toFixed(2))}
			subText="per participant"
		/>
	</div>
{:else if data.componentType === 'thinkingspace-questions'}
	{#if data.questions.length === 0}
		<p class="text-muted-foreground text-base">No one has answered an opening question yet.</p>
	{:else}
		<Table.Root class="bg-card rounded-xl border">
			<Table.Header>
				<Table.Row>
					<Table.Head class="text-base">Opening question</Table.Head>
					<Table.Head class="text-right text-base">Participants</Table.Head>
					<Table.Head class="text-right text-base">Follow-ups</Table.Head>
				</Table.Row>
			</Table.Header>
			<Table.Body>
				{#each data.questions as question, index (index)}
					<Table.Row>
						<Table.Cell class="text-base text-wrap!">{question.question}</Table.Cell>
						<Table.Cell class="text-right text-base">{question.participants}</Table.Cell
						>
						<Table.Cell class="text-right text-base">{question.followUps}</Table.Cell>
					</Table.Row>
				{/each}
			</Table.Body>
		</Table.Root>
	{/if}
{:else if data.componentType === 'thinkingspace-themes'}
	<ThinkingSpaceThemes themes={data.themes} />
{/if}
