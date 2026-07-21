<script lang="ts">
	import * as Table from '$lib/components/ui/table';
	import type { ThinkingSpaceUserInsights } from '@crownshy/api-client/api';

	type Props = {
		userInsights: ThinkingSpaceUserInsights[];
	};
	let { userInsights }: Props = $props();
</script>

<Table.Root class="w-full table-fixed">
	<Table.Header>
		<Table.Row class="text-xs">
			<Table.Head class="text-muted-foreground w-[10%] px-5">PID</Table.Head>
			<Table.Head class="text-muted-foreground w-2/5 px-5">Summary</Table.Head>
			<Table.Head class="text-muted-foreground w-2/5 px-5">Question/Responses</Table.Head>
		</Table.Row>
	</Table.Header>
	<Table.Body>
		{#each userInsights as insight (insight.userId)}
			<Table.Row>
				<Table.Cell class="text-muted-foreground border-l-6 px-5 py-9"
					>{insight.userId.substring(0, 8).concat('...')}</Table.Cell
				>
				<Table.Cell class="text-foreground h-auto px-5 text-wrap!">
					{#if insight.summary.summary.length > 120}
						{insight.summary.summary.substring(0, 120).concat('...')}
					{:else}
						{insight.summary.summary}
					{/if}
				</Table.Cell>
				<Table.Cell class="text-muted-foreground h-auto px-5 text-wrap!">
					{#if insight.answers[0].root.question.length > 120}
						{insight.answers[0].root.question.substring(0, 120).concat('...')}
					{:else}
						{insight.answers[0].root.question}
					{/if}
				</Table.Cell>
			</Table.Row>
		{/each}
	</Table.Body>
</Table.Root>
