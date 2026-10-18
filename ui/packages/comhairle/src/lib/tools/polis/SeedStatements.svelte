<script lang="ts">
	import { invalidate } from '$app/navigation';
	import type { PolisStatementAux } from '@crownshy/api-client/api';
	import AddSeedStatementsDialog from './polis-moderation/AddSeedStatementsDialog.svelte';

	type Props = {
		conversationId: string;
		workflowStepId: string;
		/** Every statement in the step. Seeds are listed; the rest are used for duplicate flags. */
		statements: PolisStatementAux[];
		canEdit: boolean;
	};

	let { conversationId, workflowStepId, statements, canEdit }: Props = $props();

	let seeds = $derived(
		statements
			.filter((statement) => statement.is_seed)
			.sort((a, b) => a.polis_statement_id - b.polis_statement_id)
	);
</script>

<div class="flex flex-col gap-4">
	<div class="flex flex-wrap items-start justify-between gap-4">
		<div class="flex flex-col gap-1">
			<h3 class="text-base font-bold">Seed statements</h3>
			<span class="text-muted-foreground text-sm">
				The first statements participants vote on. They are approved automatically.
			</span>
		</div>
		{#if canEdit}
			<AddSeedStatementsDialog
				{conversationId}
				{workflowStepId}
				existingStatements={statements.map((statement) => statement.statement_text)}
				onSeeded={() => invalidate('polis:statement-aux')}
			/>
		{/if}
	</div>

	{#if seeds.length === 0}
		<p class="text-muted-foreground text-base">No seed statements yet.</p>
	{:else}
		<ol class="bg-background divide-border border-border divide-y rounded-xl border">
			{#each seeds as seed, index (seed.id)}
				<li class="flex gap-4 px-5 py-3 text-base">
					<span class="text-muted-foreground w-6 shrink-0 tabular-nums">{index + 1}</span>
					<span>{seed.statement_text}</span>
				</li>
			{/each}
		</ol>
	{/if}
</div>
