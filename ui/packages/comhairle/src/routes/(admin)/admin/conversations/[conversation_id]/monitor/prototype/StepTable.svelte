<script lang="ts">
	import * as Table from '$lib/components/ui/table';
	import type { StepProgress } from './sampleMonitorData';

	type Props = { steps: StepProgress[] };

	let { steps }: Props = $props();

	function finishRate(step: StepProgress): number {
		return step.reached === 0 ? 0 : Math.round((step.finished / step.reached) * 100);
	}
</script>

<div class="bg-card overflow-x-auto rounded-xl border">
	<Table.Root class="min-w-[720px]">
		<Table.Header>
			<Table.Row>
				<Table.Head class="px-5">Step</Table.Head>
				<Table.Head>Started</Table.Head>
				<Table.Head>Finished</Table.Head>
				<Table.Head class="w-[28%]">Finish rate</Table.Head>
				<Table.Head>Median time</Table.Head>
				<Table.Head>Inactive 3+ days</Table.Head>
			</Table.Row>
		</Table.Header>
		<Table.Body>
			{#each steps as step (step.name)}
				<Table.Row>
					<Table.Cell class="px-5 font-semibold">{step.name}</Table.Cell>
					<Table.Cell>{step.reached}</Table.Cell>
					<Table.Cell>{step.finished}</Table.Cell>
					<Table.Cell>
						<div class="flex items-center gap-3">
							<div class="bg-muted h-2 flex-1 rounded-full">
								<div
									class="bg-primary h-2 rounded-full"
									style:width="{finishRate(step)}%"
								></div>
							</div>
							<span class="w-10 text-right">{finishRate(step)}%</span>
						</div>
					</Table.Cell>
					<Table.Cell>{step.medianMinutes} min</Table.Cell>
					<Table.Cell>{step.inactiveThreeDays}</Table.Cell>
				</Table.Row>
			{/each}
		</Table.Body>
	</Table.Root>
</div>
