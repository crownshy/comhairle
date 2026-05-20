<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import { PrioritizationStore, getOrCreateParticipantId } from './store.svelte';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import QuestionField from './QuestionField.svelte';
	import type { AnswerValue, ParticipantDraft, Proposal } from './types';

	type Props = {
		onDone?: () => void;
		workflowStep?: { id: string };
		participantId?: string;
		readonly?: boolean;
		storeOverride?: PrioritizationStore;
	};

	let { onDone, workflowStep, participantId, readonly = false, storeOverride }: Props = $props();

	let store = $derived(storeOverride ?? new PrioritizationStore(workflowStep?.id ?? 'unknown'));
	let myId = $derived(participantId ?? getOrCreateParticipantId());

	type Phase = 'intro' | 'proposal' | 'review' | 'submitted';
	let phase = $state<Phase>('intro');
	let proposalIdx = $state(0);

	let draft = $state<ParticipantDraft>({
		participantId: 'pending',
		byProposal: {},
		startedAt: new Date().toISOString()
	});

	$effect(() => {
		// Re-hydrate draft when participant id or step changes.
		draft = store.loadDraft(myId);
	});

	let questions = $derived(store.poll.toolConfig.questions);

	/** Per-participant proposal order (optionally randomized at intro->proposal transition). */
	let orderedProposals = $state<Proposal[]>([]);

	function shuffle<T>(arr: T[]): T[] {
		const a = [...arr];
		for (let i = a.length - 1; i > 0; i--) {
			const j = Math.floor(Math.random() * (i + 1));
			[a[i], a[j]] = [a[j], a[i]];
		}
		return a;
	}

	function startAnswering() {
		orderedProposals = store.poll.toolConfig.randomizeOrder
			? shuffle(store.poll.proposals)
			: [...store.poll.proposals];
		proposalIdx = 0;
		phase = 'proposal';
	}

	let currentProposal = $derived(orderedProposals[proposalIdx]);

	function setAnswer(qid: string, value: AnswerValue) {
		if (readonly || !currentProposal) return;
		draft = store.setDraftAnswer(myId, currentProposal.id, qid, value);
	}

	function next() {
		if (proposalIdx < orderedProposals.length - 1) {
			proposalIdx += 1;
		} else {
			phase = 'review';
		}
	}

	function prev() {
		if (proposalIdx > 0) proposalIdx -= 1;
	}

	async function submit() {
		if (readonly) {
			phase = 'submitted';
			return;
		}
		const sub = await store.submitDraft(myId);
		if (sub) {
			phase = 'submitted';
		} else {
			alert('Some required questions are not answered yet, or submission failed.');
		}
	}

	let canSubmit = $derived(store.draftIsComplete(draft));
</script>

<div class="mx-auto flex w-full max-w-2xl flex-col gap-4 p-4">
	{#if phase === 'intro'}
		<Card.Root>
			<Card.Header>
				<Card.Title>{store.poll.title || 'Untitled poll'}</Card.Title>
				{#if store.poll.description}
					<Card.Description>{store.poll.description}</Card.Description>
				{/if}
			</Card.Header>
			<Card.Footer>
				<Button onclick={startAnswering} disabled={store.poll.proposals.length === 0}>
					Enter
				</Button>
			</Card.Footer>
		</Card.Root>
	{:else if phase === 'proposal' && currentProposal}
		<!-- progress bar -->
		<div class="flex items-center justify-between text-xs">
			<div class="flex gap-1">
				{#each orderedProposals as _, i (i)}
					<div
						class="h-1 w-8 rounded-full {i <= proposalIdx ? 'bg-primary' : 'bg-muted'}"
					></div>
				{/each}
			</div>
			<span class="text-muted-foreground">
				{proposalIdx + 1} / {orderedProposals.length}
			</span>
		</div>

		<div class="text-muted-foreground text-xs">Proposal {proposalIdx + 1}</div>
		<h2 class="text-2xl font-semibold">
			{currentProposal.title || `Proposal ${proposalIdx + 1}`}
		</h2>
		{#if currentProposal.imageDataUrl}
			<img
				src={currentProposal.imageDataUrl}
				alt={currentProposal.title}
				class="max-h-64 w-full rounded-md object-cover"
			/>
		{/if}
		{#if currentProposal.body}
			<ContentRenderer content={currentProposal.body} />
		{/if}

		<div class="flex flex-col gap-6">
			{#if questions.length === 0}
				<p class="text-muted-foreground text-sm">No questions configured.</p>
			{/if}
			{#each questions as q (q.id)}
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
				{proposalIdx < orderedProposals.length - 1 ? 'Next >' : 'Review answers'}
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
				{#each orderedProposals as p, i (p.id)}
					<div class="rounded-md border p-3">
						<div class="flex items-center justify-between">
							<div class="font-medium">
								Proposal {i + 1}: {p.title || 'Untitled'}
							</div>
							<Button
								variant="ghost"
								size="sm"
								onclick={() => {
									proposalIdx = i;
									phase = 'proposal';
								}}
							>
								Edit
							</Button>
						</div>
						<ul class="text-muted-foreground mt-2 list-disc pl-5 text-sm">
							{#each questions as q (q.id)}
								<li>
									{q.prompt || `Q${q.order}`}:
									{#if !draft.byProposal[p.id]?.[q.id]}
										<span class="text-destructive">
											{q.optional ? '(skipped)' : 'Missing'}
										</span>
									{:else if draft.byProposal[p.id][q.id].kind === 'text'}
										{draft.byProposal[p.id][q.id].value || '(empty)'}
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
					Thank you for answering: {store.poll.title}
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
	{/if}
</div>
