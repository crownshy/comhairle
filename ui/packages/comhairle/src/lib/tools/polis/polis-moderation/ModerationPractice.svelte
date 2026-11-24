<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import type { PolisStatementAux } from '@crownshy/api-client/api';
	import { splitReason, type RejectReason } from '$lib/moderation/moderationPolicy';
	import CircleAlert from '@lucide/svelte/icons/circle-alert';
	import CircleCheck from '@lucide/svelte/icons/circle-check';
	import RotateCcw from '@lucide/svelte/icons/rotate-ccw';
	import StatementsTable from './StatementsTable.svelte';
	import {
		PRACTICE_STATEMENTS,
		type PracticeChoice,
		type PracticeStatement
	} from './moderationGuidance';

	type Props = { rejectReasons: RejectReason[] };
	let { rejectReasons }: Props = $props();

	type ModerationStatus = PolisStatementAux['moderation_status'];
	type Feedback = { correct: boolean; message: string };

	// Practice rows live only in this component, so the real list can be poked at freely.
	// It's left without an Edit action: accept or reject is enough to practise the rules.
	function practiceRows(): PolisStatementAux[] {
		const now = new Date().toISOString();
		return PRACTICE_STATEMENTS.map((statement, i) => ({
			id: `practice-${i}`,
			created_at: now,
			updated_at: now,
			is_seed: false,
			moderation_status: 'pending',
			moderation_reason: null,
			original_statement_id: null,
			polis_conversation_id: 'practice',
			polis_statement_id: i + 1,
			statement_text: statement.text,
			themes: [],
			workflow_step_id: 'practice',
			zid: 0
		}));
	}

	let rows = $state(practiceRows());
	let selected = $state<Record<string, boolean>>({});
	let feedback = $state<Record<string, Feedback>>({});

	const practiceById = Object.fromEntries(
		PRACTICE_STATEMENTS.map((statement, i) => [`practice-${i}`, statement])
	);
	const CHOICE_LABELS: Record<PracticeChoice, string> = { accept: 'Accept', reject: 'Reject' };
	const reasonLabels = $derived(rejectReasons.map((reason) => reason.label));

	const decidedCount = $derived(Object.keys(feedback).length);
	const correctCount = $derived(Object.values(feedback).filter((f) => f.correct).length);

	function judge(practice: PracticeStatement, choice: PracticeChoice, reason?: string): Feedback {
		if (choice !== practice.answer) {
			return {
				correct: false,
				message: `Not quite. The best call is ${CHOICE_LABELS[practice.answer]}. ${practice.explanation}`
			};
		}
		const pickedLabel = splitReason(reason, reasonLabels).label;
		if (practice.reason && pickedLabel !== practice.reason) {
			return {
				correct: false,
				message: `Right to reject, but the best reason is ${practice.reason}. ${practice.explanation}`
			};
		}
		const reasonNote = practice.reason ? ` Reason: ${practice.reason}.` : '';
		return { correct: true, message: `Right. ${practice.explanation}${reasonNote}` };
	}

	function setStatus(id: string, status: ModerationStatus, reason?: string) {
		rows = rows.map((r) =>
			r.id === id
				? {
						...r,
						moderation_status: status,
						moderation_reason: status === 'rejected' ? (reason ?? null) : null
					}
				: r
		);
		const practice = practiceById[id];
		if (!practice) return;
		if (status === 'pending') {
			const { [id]: _removed, ...rest } = feedback;
			feedback = rest;
			return;
		}
		const choice = status === 'accepted' ? 'accept' : 'reject';
		feedback = { ...feedback, [id]: judge(practice, choice, reason) };
	}

	function moderate(row: PolisStatementAux, status: ModerationStatus, reason?: string) {
		if (selected[row.id]) return bulkModerate(status, reason);
		setStatus(row.id, status, reason);
	}

	function bulkModerate(status: ModerationStatus, reason?: string) {
		for (const id of Object.keys(selected).filter((id) => selected[id])) {
			setStatus(id, status, reason);
		}
		selected = {};
	}

	function restart() {
		rows = practiceRows();
		selected = {};
		feedback = {};
	}
</script>

<div class="flex flex-col gap-4">
	<StatementsTable
		{rows}
		showStatus
		heading="{PRACTICE_STATEMENTS.length} practice statements"
		{rejectReasons}
		{selected}
		pending={{}}
		bulkAction={null}
		lineage={{}}
		translationsByStatement={{}}
		onTranslationChange={() => {}}
		onToggleSelect={(id, checked) => (selected = { ...selected, [id]: checked })}
		onToggleAll={(checked) => (selected = Object.fromEntries(rows.map((r) => [r.id, checked])))}
		onClear={() => (selected = {})}
		onBulkModerate={bulkModerate}
		onModerate={moderate}
		onUndoSplit={() => {}}
	>
		{#snippet empty()}
			<p class="text-muted-foreground px-4 py-8 text-base">No practice statements.</p>
		{/snippet}
	</StatementsTable>

	{#if decidedCount > 0}
		<ul class="flex flex-col gap-2" aria-live="polite">
			{#each PRACTICE_STATEMENTS as _statement, i (i)}
				{@const result = feedback[`practice-${i}`]}
				{#if result}
					<li
						class={[
							'flex items-start gap-3 rounded-lg border p-3 text-base',
							result.correct
								? 'border-primary/30 bg-primary/5'
								: 'border-amber-300 bg-amber-50 dark:border-amber-800 dark:bg-amber-950/40'
						]}
					>
						{#if result.correct}
							<CircleCheck class="text-primary mt-0.5 size-5 shrink-0" />
						{:else}
							<CircleAlert
								class="mt-0.5 size-5 shrink-0 text-amber-700 dark:text-amber-400"
							/>
						{/if}
						<p><span class="font-semibold">#{i + 1}</span> {result.message}</p>
					</li>
				{/if}
			{/each}
		</ul>
	{/if}

	<div class="flex flex-wrap items-center justify-between gap-2">
		<p class="text-muted-foreground text-base">
			{#if decidedCount === PRACTICE_STATEMENTS.length}
				{correctCount} of {PRACTICE_STATEMENTS.length} right. Practice only, nothing is saved.
			{:else}
				Practice only. Nothing here is saved.
			{/if}
		</p>
		{#if decidedCount > 0}
			<Button variant="outline" onclick={restart}>
				<RotateCcw class="size-4" />
				Start over
			</Button>
		{/if}
	</div>
</div>
