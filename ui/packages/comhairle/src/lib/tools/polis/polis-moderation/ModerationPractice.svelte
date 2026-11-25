<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import Callout from '$lib/components/Callout.svelte';
	import type { RejectReason } from '$lib/moderation/moderationPolicy';
	import type { PolisStatementAux } from '@crownshy/api-client/api';
	import CircleCheck from '@lucide/svelte/icons/circle-check';
	import CircleX from '@lucide/svelte/icons/circle-x';
	import RotateCcw from '@lucide/svelte/icons/rotate-ccw';
	import StatementsTable from './StatementsTable.svelte';
	import { moderatePracticeRows, practiceFeedback, practiceRows } from './practiceStatements';
	import { pluralise } from '$lib/utils/pluralise';

	type ModerationStatus = PolisStatementAux['moderation_status'];

	type Props = {
		/** The step's moderation policy reasons, offered on reject as in the real list. */
		rejectReasons: RejectReason[];
	};

	let { rejectReasons }: Props = $props();

	let rows = $state(practiceRows());
	let selected = $state<Record<string, boolean>>({});

	const feedback = $derived(practiceFeedback(rows));
	const anyDecided = $derived(feedback.length > 0);
	const heading = $derived(`${rows.length} practice ${pluralise(rows.length, 'statement')}`);

	function moderate(ids: string[], status: ModerationStatus, reason?: string) {
		rows = moderatePracticeRows(rows, ids, status, reason);
		selected = {};
	}

	function moderateRow(row: PolisStatementAux, status: ModerationStatus, reason?: string) {
		// As in the real list, a decision on a selected row applies to the whole selection.
		if (selected[row.id]) return moderateSelection(status, reason);
		moderate([row.id], status, reason);
	}

	function moderateSelection(status: ModerationStatus, reason?: string) {
		moderate(
			rows.filter((row) => selected[row.id]).map((row) => row.id),
			status,
			reason
		);
	}

	function toggleAll(checked: boolean) {
		selected = Object.fromEntries(rows.map((row) => [row.id, checked]));
	}

	function startOver() {
		rows = practiceRows();
		selected = {};
	}
</script>

<div class="flex flex-col gap-4">
	<StatementsTable
		{rows}
		showStatus
		{heading}
		{rejectReasons}
		{selected}
		pending={{}}
		bulkAction={null}
		lineage={{}}
		translationsByStatement={{}}
		onTranslationChange={() => {}}
		onToggleSelect={(id, checked) => (selected = { ...selected, [id]: checked })}
		onToggleAll={toggleAll}
		onClear={() => (selected = {})}
		onBulkModerate={moderateSelection}
		onModerate={moderateRow}
	>
		{#snippet empty()}{/snippet}
	</StatementsTable>

	{#if anyDecided}
		<ul class="flex flex-col gap-2" aria-live="polite">
			{#each feedback as item (item.id)}
				{@const Icon = item.correct ? CircleCheck : CircleX}
				<li>
					<Callout
						variant={item.correct ? 'primary' : 'danger'}
						class="flex items-start gap-3 px-4"
					>
						<Icon class="mt-1 size-5 shrink-0" />
						<p class="text-base">
							<span class="font-semibold tabular-nums">#{item.number}</span>
							{item.message}
						</p>
					</Callout>
				</li>
			{/each}
		</ul>
	{/if}

	<div class="flex flex-wrap items-center justify-between gap-3">
		<p class="text-muted-foreground text-base">Practice only. Nothing here is saved.</p>
		{#if anyDecided}
			<Button variant="outline" onclick={startOver}>
				<RotateCcw class="size-4" />
				Start over
			</Button>
		{/if}
	</div>
</div>
