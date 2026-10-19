<script lang="ts">
	import { invalidate } from '$app/navigation';
	import type { PolisStatementAux } from '@crownshy/api-client/api';
	import { apiClient } from '@crownshy/api-client/client';
	import { ChevronDown, RotateCcw, Trash2 } from '@lucide/svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Collapsible from '$lib/components/ui/collapsible';
	import { notifications } from '$lib/notifications.svelte';
	import { permissions } from '$lib/permissions.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import AddSeedStatementsDialog from './polis-moderation/AddSeedStatementsDialog.svelte';

	type Props = {
		conversationId: string;
		workflowStepId: string;
		/** Every statement in the step. Seeds are listed; the rest are used for duplicate flags. */
		statements: PolisStatementAux[];
		canEdit: boolean;
	};

	let { conversationId, workflowStepId, statements, canEdit }: Props = $props();

	// Polis has no delete, so removing a seed rejects it and restoring accepts it again.
	// The reason keeps the moderation log readable.
	const REMOVED_REASON = 'Removed by admin';

	const canChangeSeeds = $derived(
		canEdit && permissions.can('conversation', 'conversation_moderate', conversationId)
	);

	let pending = $state<Record<string, boolean>>({});
	let removedOpen = $state(false);

	const sortedSeeds = $derived(
		statements
			.filter((statement) => statement.is_seed)
			.sort((a, b) => a.polis_statement_id - b.polis_statement_id)
	);
	const seeds = $derived(sortedSeeds.filter((seed) => seed.moderation_status !== 'rejected'));
	const removedSeeds = $derived(
		sortedSeeds.filter((seed) => seed.moderation_status === 'rejected')
	);

	async function setSeedStatus(seed: PolisStatementAux, decision: 'accept' | 'reject') {
		if (pending[seed.id]) return;
		pending = { ...pending, [seed.id]: true };

		const result = await tryCatchAsync(() =>
			apiClient.PolisModerateStatementAux(
				{ decision, moderation_reason: decision === 'reject' ? REMOVED_REASON : null },
				{ params: { id: seed.id } }
			)
		);
		if (result.err !== null) {
			pending = { ...pending, [seed.id]: false };
			notifications.send({
				priority: 'ERROR',
				message:
					decision === 'reject'
						? 'Could not remove the seed statement'
						: 'Could not restore the seed statement'
			});
			return;
		}
		await invalidate('polis:statement-aux');
		pending = { ...pending, [seed.id]: false };
	}
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
				removedSeeds={removedSeeds.map((seed) => seed.statement_text)}
				onSeeded={() => invalidate('polis:statement-aux')}
			/>
		{/if}
	</div>

	{#if seeds.length === 0}
		<p class="text-muted-foreground text-base">No seed statements yet.</p>
	{:else}
		<ol class="bg-background divide-border border-border divide-y rounded-xl border">
			{#each seeds as seed, index (seed.id)}
				<li class="flex items-center gap-4 px-5 py-3 text-base">
					<span class="text-muted-foreground w-6 shrink-0 tabular-nums">{index + 1}</span>
					<span class="flex-1">{seed.statement_text}</span>
					{#if canChangeSeeds}
						<Button
							variant="ghost"
							size="icon"
							disabled={pending[seed.id]}
							onclick={() => setSeedStatus(seed, 'reject')}
							title={`Remove seed statement ${index + 1}`}
						>
							<Trash2 class="size-4" />
						</Button>
					{/if}
				</li>
			{/each}
		</ol>
	{/if}

	{#if removedSeeds.length > 0}
		<Collapsible.Root bind:open={removedOpen}>
			<Collapsible.Trigger
				class="text-muted-foreground inline-flex items-center gap-1 text-base hover:underline"
			>
				Removed ({removedSeeds.length})
				<ChevronDown
					class="size-4 transition-transform {removedOpen ? 'rotate-180' : ''}"
				/>
			</Collapsible.Trigger>
			<Collapsible.Content>
				<ul
					class="divide-border border-border mt-2 divide-y rounded-xl border border-dashed"
				>
					{#each removedSeeds as seed (seed.id)}
						<li class="flex items-center gap-4 px-5 py-3 text-base">
							<span class="text-muted-foreground flex-1">{seed.statement_text}</span>
							{#if canChangeSeeds}
								<Button
									variant="ghost"
									size="sm"
									disabled={pending[seed.id]}
									onclick={() => setSeedStatus(seed, 'accept')}
								>
									<RotateCcw class="size-4" />
									Restore
								</Button>
							{/if}
						</li>
					{/each}
				</ul>
			</Collapsible.Content>
		</Collapsible.Root>
	{/if}
</div>
