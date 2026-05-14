<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import { PrioritisationStore, getOrCreateParticipantId } from './store.svelte';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import QuestionField from './QuestionField.svelte';
	import type { AnswerValue, ParticipantDraft } from './types';

	type Props = {
		onDone?: () => void;
		workflowStep?: { id: string };
		// Allow caller to override participant id (used by Preview to use a sandboxed id).
		participantId?: string;
		readonly?: boolean;
		// When provided, use this store instance (preview path); else build one from workflowStep.id
		storeOverride?: PrioritisationStore;
	};

	let { onDone, workflowStep, participantId, readonly = false, storeOverride }: Props = $props();

	let store = $derived(storeOverride ?? new PrioritisationStore(workflowStep?.id ?? 'unknown'));
	let myId = $derived(participantId ?? getOrCreateParticipantId());

	type Phase = 'intro' | 'proposal' | 'review' | 'submitted' | 'closed';
	let phase = $state<Phase>('intro');
	let proposalIdx = $state(0);

	let draft = $state<ParticipantDraft>({
		participantId: myId,
		byProposal: {},
		startedAt: new Date().toISOString()
	});

	$effect(() => {
		draft = store.loadDraft(myId);
	});

	// Timer ticker
	let now = $state(Date.now());
	$effect(() => {
		const t = setInterval(() => (now = Date.now()), 1000);
		return () => clearInterval(t);
	});

	let timeLeft = $derived(store.timeLeftSeconds(now));

	function fmtTime(s: number | null): string {
		if (s === null) return '';
		const m = Math.floor(s / 60)
			.toString()
			.padStart(2, '0');
		const sec = Math.floor(s % 60)
			.toString()
			.padStart(2, '0');
		return `${m}:${sec}`;
	}

	let currentProposal = $derived(store.poll.proposals[proposalIdx]);
	let progressPct = $derived(
		store.poll.proposals.length ? ((proposalIdx + 1) / store.poll.proposals.length) * 100 : 0
	);

	function setAnswer(qid: string, value: AnswerValue) {
		if (readonly || !currentProposal) return;
		draft = store.setDraftAnswer(myId, currentProposal.id, qid, value);
	}

	function next() {
		if (proposalIdx < store.poll.proposals.length - 1) {
			proposalIdx += 1;
		} else {
			phase = 'review';
		}
	}

	function prev() {
		if (proposalIdx > 0) proposalIdx -= 1;
	}

	function submit() {
		if (readonly) {
			phase = 'submitted';
			return;
		}
		const sub = store.submitDraft(myId);
		if (sub) {
			phase = 'submitted';
		} else {
			alert('Some required questions are not answered yet.');
		}
	}

	let canSubmit = $derived(store.draftIsComplete(draft));

	// Auto-redirect if poll state changes mid-flow.
	$effect(() => {
		if (store.poll.state === 'draft') phase = 'closed';
	});
</script>

<div class="mx-auto flex w-full max-w-2xl flex-col gap-4 p-4">
	{#if store.poll.state === 'draft' && !readonly}
		<Card.Root>
			<Card.Content class="p-6">
				<p>This poll isn't open yet. Check back when the facilitator starts it.</p>
			</Card.Content>
		</Card.Root>
	{:else if store.poll.state === 'paused' && phase !== 'submitted'}
		<Card.Root>
			<Card.Content class="p-6">
				<p>The facilitator has paused this poll. Your answers are saved.</p>
			</Card.Content>
		</Card.Root>
	{:else if phase === 'intro'}
		<Card.Root>
			<Card.Header>
				<Card.Title>{store.poll.title || 'Untitled poll'}</Card.Title>
				{#if store.poll.instruction}
					<Card.Description>{store.poll.instruction}</Card.Description>
				{/if}
			</Card.Header>
			<Card.Footer>
				<Button onclick={() => (phase = 'proposal')}>Enter</Button>
			</Card.Footer>
		</Card.Root>
	{:else if phase === 'proposal' && currentProposal}
		<!-- progress bar + timer -->
		<div class="flex items-center justify-between text-xs">
			<div class="flex gap-1">
				{#each store.poll.proposals as _, i (i)}
					<div
						class="h-1 w-8 rounded-full {i <= proposalIdx ? 'bg-primary' : 'bg-muted'}"
					></div>
				{/each}
			</div>
			{#if timeLeft !== null}
				<span class="text-muted-foreground">Time left: {fmtTime(timeLeft)}</span>
			{/if}
		</div>

		<div class="text-muted-foreground text-xs">Proposal {currentProposal.order}</div>
		<h2 class="text-2xl font-semibold">
			{currentProposal.title || `Proposal ${currentProposal.order}`}
		</h2>
		{#if currentProposal.imageDataUrl}
			<img
				src={currentProposal.imageDataUrl}
				alt={currentProposal.title}
				class="max-h-64 w-full rounded-md object-cover"
			/>
		{/if}
		{#if currentProposal.content}
			<ContentRenderer content={currentProposal.content} />
		{/if}

		<div class="flex flex-col gap-6">
			{#if currentProposal.questions.length === 0}
				<p class="text-muted-foreground text-sm">
					No questions configured for this proposal.
				</p>
			{/if}
			{#each currentProposal.questions as q (q.id)}
				<QuestionField
					question={q}
					value={draft.byProposal[currentProposal.id]?.[q.id]}
					onChange={(v) => setAnswer(q.id, v)}
					{readonly}
				/>
			{/each}
		</div>

		<div class="flex justify-between pt-4">
			<Button variant="outline" onclick={prev} disabled={proposalIdx === 0}>
				&lt; Previous
			</Button>
			<Button onclick={next}>
				{proposalIdx < store.poll.proposals.length - 1 ? 'Next >' : 'Review answers'}
			</Button>
		</div>
	{:else if phase === 'review'}
		<Card.Root>
			<Card.Header>
				<Card.Title>Review your answers</Card.Title>
				<Card.Description>
					Check your answers below. You can jump back to any proposal to edit.
				</Card.Description>
			</Card.Header>
			<Card.Content class="flex flex-col gap-3">
				{#each store.poll.proposals as p (p.id)}
					<div class="rounded-md border p-3">
						<div class="flex items-center justify-between">
							<div class="font-medium">
								Proposal {p.order}: {p.title || 'Untitled'}
							</div>
							<Button
								variant="ghost"
								size="sm"
								onclick={() => {
									proposalIdx = store.poll.proposals.findIndex(
										(x) => x.id === p.id
									);
									phase = 'proposal';
								}}
							>
								Edit
							</Button>
						</div>
						<ul class="text-muted-foreground mt-2 list-disc pl-5 text-sm">
							{#each p.questions as q (q.id)}
								<li>
									{q.prompt || `Q${q.order}`}:
									{#if !draft.byProposal[p.id]?.[q.id]}
										<span class="text-destructive"
											>{q.optional ? '(skipped)' : 'Missing'}</span
										>
									{:else if draft.byProposal[p.id][q.id].kind === 'text'}
										{draft.byProposal[p.id][q.id].value || '(empty)'}
									{:else if draft.byProposal[p.id][q.id].kind === 'choice'}
										{q.type === 'multiple_choice'
											? q.choices.find(
													(c) =>
														c.id ===
														draft.byProposal[p.id][q.id].choiceId
												)?.label
											: draft.byProposal[p.id][q.id].choiceId}
									{:else}
										{draft.byProposal[p.id][q.id].value}
									{/if}
								</li>
							{/each}
						</ul>
					</div>
				{/each}
			</Card.Content>
			<Card.Footer class="flex justify-between">
				<Button variant="outline" onclick={() => (phase = 'proposal')}>Back</Button>
				<Button onclick={submit} disabled={!canSubmit}>Submit</Button>
			</Card.Footer>
		</Card.Root>
	{:else if phase === 'submitted'}
		<Card.Root>
			<Card.Header>
				<Card.Title>
					Thank you for answering the poll: {store.poll.title}
				</Card.Title>
			</Card.Header>
			<Card.Content>
				<p class="text-muted-foreground">
					Your answers have been recorded. You can now move on to the next step.
				</p>
			</Card.Content>
			{#if onDone}
				<Card.Footer>
					<Button onclick={onDone}>Continue</Button>
				</Card.Footer>
			{/if}
		</Card.Root>
	{:else if phase === 'closed'}
		<Card.Root>
			<Card.Content class="p-6">
				<p>This poll has ended.</p>
			</Card.Content>
		</Card.Root>
	{/if}
</div>
