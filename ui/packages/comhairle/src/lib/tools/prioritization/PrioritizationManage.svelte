<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { Switch } from '$lib/components/ui/switch';
	import * as Card from '$lib/components/ui/card';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { Plus, Pencil, Trash2, LoaderCircle, GripVertical } from 'lucide-svelte';
	import DraggableList from '$lib/components/DraggableList.svelte';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import { createStore } from './store.svelte';
	import { resolveToolConfig } from './prioritizationApi';
	import ProposalEditorDialog from './components/ProposalEditorDialog.svelte';
	import ProposalListSkeleton from './components/ProposalListSkeleton.svelte';
	import QuestionEditorDialog from './components/QuestionEditorDialog.svelte';
	import type {
		ConversationInput,
		Proposal,
		Question,
		QuestionType,
		WorkflowStepInput
	} from './types';
	import * as Select from '$lib/components/ui/select';
	import Spinner from '$lib/components/ui/spinner/spinner.svelte';

	let {
		conversationId,
		workflowId,
		workflowStep,
		conversation
	}: {
		conversationId: string;
		workflowId: string;
		workflowStep: WorkflowStepInput;
		conversation: ConversationInput;
	} = $props();

	/** The host page keys this component by step id, so the ids are stable for
	 * the component's lifetime — capturing them once is intentional. */
	// svelte-ignore state_referenced_locally
	const store = createStore({
		workflowStepId: workflowStep.id,
		conversationId,
		workflowId,
		isLive: conversation.isLive ?? false
	});

	let toolConfig = $derived(resolveToolConfig(workflowStep, conversation.isLive ?? false));
	let primaryLocale = $derived(conversation.primaryLocale ?? 'en');
	let supportedLocales = $derived(
		conversation.supportedLanguages && conversation.supportedLanguages.length > 0
			? conversation.supportedLanguages
			: [primaryLocale]
	);

	let editorOpen = $state(false);
	let deleteOpen = $state(false);
	let selectedProposal = $state<Proposal | null>(null);
	let deleting = $state(false);

	let questionEditorOpen = $state(false);
	let questionDeleteOpen = $state(false);
	let selectedQuestion = $state<Question | null>(null);
	let deletingQuestionInFlight = $state(false);
	let randomizeSaving = $state(false);

	/** Per-section question editor/delete state (mirrors the proposal-question flow). */
	let sectionQuestionEditorOpen = $state(false);
	let sectionQuestionDeleteOpen = $state(false);
	let selectedSectionQuestion = $state<Question | null>(null);
	let deletingSectionQuestionInFlight = $state(false);

	const questions = $derived<Question[]>(toolConfig.questions ?? []);
	const sectionQuestions = $derived<Question[]>(toolConfig.sectionQuestions ?? []);

	/** Local mirror of `questions` so svelte-dnd-action can mutate during drag. As a writable $derived it tracks upstream by default but stays at any value we assign until the source changes again — exactly the in-flight-then-snap-back behaviour the dnd lib needs. */
	let localQuestions = $derived(questions);
	let localSectionQuestions = $derived(sectionQuestions);
	let savingOrder = $state(false);
	let savingSectionOrder = $state(false);

	async function commitQuestionOrder(next: Question[]) {
		savingOrder = true;
		try {
			await store.saveToolConfig({
				questions: next,
				sectionQuestions,
				randomizeOrder: toolConfig.randomizeOrder
			});
		} catch {
			/** saveToolConfig surfaces an error toast. Revert local view to the upstream order. */
			localQuestions = questions;
		} finally {
			savingOrder = false;
		}
	}

	async function commitSectionQuestionOrder(next: Question[]) {
		savingSectionOrder = true;
		try {
			await store.saveToolConfig({
				questions,
				sectionQuestions: next,
				randomizeOrder: toolConfig.randomizeOrder
			});
		} catch {
			localSectionQuestions = sectionQuestions;
		} finally {
			savingSectionOrder = false;
		}
	}

	$effect(() => {
		void store.refresh();
	});

	function openCreate() {
		selectedProposal = null;
		editorOpen = true;
	}

	function openEdit(p: Proposal) {
		selectedProposal = p;
		editorOpen = true;
	}

	function confirmDelete(p: Proposal) {
		selectedProposal = p;
		deleteOpen = true;
	}

	async function runDelete() {
		if (!selectedProposal) return;
		deleting = true;
		try {
			await store.remove(selectedProposal.id);
			deleteOpen = false;
			selectedProposal = null;
		} catch {
			/** store.remove surfaces an error toast. Keep the dialog open so the admin understands the action did not take effect. */
		} finally {
			deleting = false;
		}
	}

	function openCreateQuestion() {
		selectedQuestion = null;
		questionEditorOpen = true;
	}

	function openEditQuestion(q: Question) {
		selectedQuestion = q;
		questionEditorOpen = true;
	}

	function confirmDeleteQuestion(q: Question) {
		selectedQuestion = q;
		questionDeleteOpen = true;
	}

	async function runDeleteQuestion() {
		if (!selectedQuestion) return;
		deletingQuestionInFlight = true;
		try {
			const next = questions.filter((q) => q.id !== selectedQuestion!.id);
			await store.saveToolConfig({
				questions: next,
				sectionQuestions,
				randomizeOrder: toolConfig.randomizeOrder
			});
			questionDeleteOpen = false;
			selectedQuestion = null;
		} catch {
			/** store.saveToolConfig surfaces an error toast. */
		} finally {
			deletingQuestionInFlight = false;
		}
	}

	function openCreateSectionQuestion() {
		selectedSectionQuestion = null;
		sectionQuestionEditorOpen = true;
	}

	function openEditSectionQuestion(q: Question) {
		selectedSectionQuestion = q;
		sectionQuestionEditorOpen = true;
	}

	function confirmDeleteSectionQuestion(q: Question) {
		selectedSectionQuestion = q;
		sectionQuestionDeleteOpen = true;
	}

	async function runDeleteSectionQuestion() {
		if (!selectedSectionQuestion) return;
		deletingSectionQuestionInFlight = true;
		try {
			const next = sectionQuestions.filter((q) => q.id !== selectedSectionQuestion!.id);
			await store.saveToolConfig({
				questions,
				sectionQuestions: next,
				randomizeOrder: toolConfig.randomizeOrder
			});
			sectionQuestionDeleteOpen = false;
			selectedSectionQuestion = null;
		} catch {
			/** store.saveToolConfig surfaces an error toast. */
		} finally {
			deletingSectionQuestionInFlight = false;
		}
	}

	async function toggleRandomize(checked: boolean) {
		randomizeSaving = true;
		try {
			await store.saveToolConfig({
				questions,
				sectionQuestions,
				randomizeOrder: checked
			});
		} catch {
			/** store.saveToolConfig surfaces an error toast. */
		} finally {
			randomizeSaving = false;
		}
	}

	function describeType(type: QuestionType): string {
		switch (type.kind) {
			case 'text':
				return 'Free text';
			case 'likert':
				return `Likert · ${type.categories.length} options`;
			case 'continuous':
				return `Slider · ${type.subSteps} steps`;
		}
	}

	function summariseScale(type: QuestionType): string {
		if (type.kind === 'likert') {
			const first = type.categories[0]?.label;
			const last = type.categories[type.categories.length - 1]?.label;
			return first && last ? `${first} → ${last}` : '';
		}
		if (type.kind === 'continuous') {
			const range = `${type.minValue}–${type.maxValue}`;
			if (type.minLabel || type.maxLabel) {
				return `${type.minLabel || type.minValue} → ${type.maxLabel || type.maxValue} (${range})`;
			}
			return range;
		}
		return '';
	}

	let savingAlignmentQuestion = $state(false);
	async function setAlignmentQuestion(value: string) {
		savingAlignmentQuestion = true;
		await store.saveToolConfig({
			questions,
			sectionQuestions,
			randomizeOrder: toolConfig.randomizeOrder,
			alignmentQuestionId: value
		});
		savingAlignmentQuestion = false;
	}
</script>

<section class="space-y-10">
	<div class="space-y-4">
		<header class="flex items-start justify-between gap-4">
			<div>
				<h2 class="text-2xl font-semibold">Questions</h2>
				<p class="text-muted-foreground text-sm">
					Participants will answer these for every proposal.
				</p>
			</div>
			<Button onclick={openCreateQuestion}>
				<Plus class="mr-2 h-4 w-4" /> Add question
			</Button>
		</header>

		{#if questions.length === 0}
			<Card.Root>
				<Card.Content class="py-10 text-center">
					<p class="text-muted-foreground">
						No questions yet. Add the first one to get started.
					</p>
				</Card.Content>
			</Card.Root>
		{:else}
			<DraggableList
				items={localQuestions}
				onReorder={(next) => (localQuestions = next)}
				onCommit={commitQuestionOrder}
				dragDisabled={savingOrder}
				class="space-y-3"
			>
				{#snippet children(q: Question)}
					<Card.Root>
						<Card.Header class="flex flex-row items-start justify-between gap-4">
							<div class="flex min-w-0 flex-1 items-start gap-3">
								<button
									type="button"
									aria-label="Drag to reorder"
									class="text-muted-foreground hover:text-foreground mt-1 cursor-grab active:cursor-grabbing"
								>
									<GripVertical class="h-4 w-4" />
								</button>
								<div class="min-w-0 flex-1 space-y-2">
									<Card.Title class="text-lg">
										{q.text || 'Untitled question'}
									</Card.Title>
									<div
										class="text-muted-foreground flex flex-wrap items-center gap-2 text-xs"
									>
										<Badge variant="outline">{describeType(q.type)}</Badge>
										{#if summariseScale(q.type)}
											<span>{summariseScale(q.type)}</span>
										{/if}
									</div>
								</div>
							</div>
							<div class="flex shrink-0 gap-2">
								<Button
									variant="outline"
									size="sm"
									onclick={() => openEditQuestion(q)}
								>
									<Pencil class="mr-2 h-3.5 w-3.5" /> Edit
								</Button>
								<Button
									variant="ghost"
									size="sm"
									class="text-destructive hover:text-destructive"
									onclick={() => confirmDeleteQuestion(q)}
								>
									<Trash2 class="mr-2 h-3.5 w-3.5" /> Delete
								</Button>
							</div>
						</Card.Header>
					</Card.Root>
				{/snippet}
			</DraggableList>
		{/if}
	</div>

	{#if questions.length > 1}
		<div class="flex flex-col gap-2">
			<h3 class="text-lg font-bold">Select your alignment question</h3>
			<Select.Root
				type="single"
				value={toolConfig.alignmentQuestionId ?? questions[0].id}
				onValueChange={setAlignmentQuestion}
			>
				<Select.Trigger>
					{questions.find((q) => q.id === toolConfig.alignmentQuestionId)?.text ??
						questions[0].text}
					{#if savingAlignmentQuestion}
						<Spinner />
					{/if}
				</Select.Trigger>
				<Select.Content>
					{#each questions as question (question.id)}
						<Select.Item value={question.id}>{question.text}</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>
		</div>
	{/if}

	<div class="space-y-4">
		<header class="flex items-start justify-between gap-4">
			<div>
				<h2 class="text-2xl font-semibold">Per-section questions</h2>
				<p class="text-muted-foreground text-sm">
					Participants will answer these for every section of every proposal.
				</p>
			</div>
			<Button onclick={openCreateSectionQuestion}>
				<Plus class="mr-2 h-4 w-4" /> Add question
			</Button>
		</header>

		{#if sectionQuestions.length === 0}
			<Card.Root>
				<Card.Content class="py-10 text-center">
					<p class="text-muted-foreground">
						No per-section questions yet. Add one to ask it about every section.
					</p>
				</Card.Content>
			</Card.Root>
		{:else}
			<DraggableList
				items={localSectionQuestions}
				onReorder={(next) => (localSectionQuestions = next)}
				onCommit={commitSectionQuestionOrder}
				dragDisabled={savingSectionOrder}
				class="space-y-3"
			>
				{#snippet children(q: Question)}
					<Card.Root>
						<Card.Header class="flex flex-row items-start justify-between gap-4">
							<div class="flex min-w-0 flex-1 items-start gap-3">
								<button
									type="button"
									aria-label="Drag to reorder"
									class="text-muted-foreground hover:text-foreground mt-1 cursor-grab active:cursor-grabbing"
								>
									<GripVertical class="h-4 w-4" />
								</button>
								<div class="min-w-0 flex-1 space-y-2">
									<Card.Title class="text-lg">
										{q.text || 'Untitled question'}
									</Card.Title>
									<div
										class="text-muted-foreground flex flex-wrap items-center gap-2 text-xs"
									>
										<Badge variant="outline">{describeType(q.type)}</Badge>
										{#if summariseScale(q.type)}
											<span>{summariseScale(q.type)}</span>
										{/if}
									</div>
								</div>
							</div>
							<div class="flex shrink-0 gap-2">
								<Button
									variant="outline"
									size="sm"
									onclick={() => openEditSectionQuestion(q)}
								>
									<Pencil class="mr-2 h-3.5 w-3.5" /> Edit
								</Button>
								<Button
									variant="ghost"
									size="sm"
									class="text-destructive hover:text-destructive"
									onclick={() => confirmDeleteSectionQuestion(q)}
								>
									<Trash2 class="mr-2 h-3.5 w-3.5" /> Delete
								</Button>
							</div>
						</Card.Header>
					</Card.Root>
				{/snippet}
			</DraggableList>
		{/if}
	</div>

	<div class="space-y-4">
		<header class="flex items-start justify-between gap-4">
			<div>
				<h2 class="text-2xl font-semibold">Proposals</h2>
				<p class="text-muted-foreground text-sm">
					Add the proposals participants will rate against the questions above.
				</p>
			</div>
			<Button onclick={openCreate}>
				<Plus class="mr-2 h-4 w-4" /> Add proposal
			</Button>
		</header>

		<div class="bg-card flex items-center justify-between rounded-md border p-3">
			<div>
				<p class="font-medium">Randomise proposal order</p>
				<p class="text-muted-foreground text-sm">
					Shuffles the order of proposals for each participant.
				</p>
			</div>
			<Switch
				checked={toolConfig.randomizeOrder}
				disabled={randomizeSaving}
				onCheckedChange={toggleRandomize}
			/>
		</div>

		{#if store.state === 'idle' || store.state === 'loading'}
			<!-- Skeleton on first paint too (idle = before the initial fetch fires), so the empty
				 state never flashes before we know whether there are proposals. -->
			<ProposalListSkeleton />
		{:else if store.state === 'error'}
			<p class="text-destructive text-sm">Could not load proposals: {store.error}</p>
		{:else if store.proposals.length === 0}
			<Card.Root>
				<Card.Content class="py-10 text-center">
					<p class="text-muted-foreground">
						No proposals yet. Add the first one to get started.
					</p>
				</Card.Content>
			</Card.Root>
		{:else}
			<ul class="space-y-3">
				{#each store.proposals as proposal (proposal.id)}
					<Card.Root>
						<Card.Header class="flex flex-row items-start justify-between gap-4">
							<div class="min-w-0 flex-1 space-y-2">
								<Card.Title class="text-lg">
									{proposal.title || 'Untitled proposal'}
								</Card.Title>
								{#each proposal.sections as section (section.id)}
									{#if section.body}
										<div class="text-muted-foreground text-sm">
											<ContentRenderer content={section.body} />
										</div>
									{/if}
								{/each}
							</div>
							<div class="flex shrink-0 gap-2">
								<Button
									variant="outline"
									size="sm"
									onclick={() => openEdit(proposal)}
								>
									<Pencil class="mr-2 h-3.5 w-3.5" /> Edit
								</Button>
								<Button
									variant="ghost"
									size="sm"
									class="text-destructive hover:text-destructive"
									onclick={() => confirmDelete(proposal)}
								>
									<Trash2 class="mr-2 h-3.5 w-3.5" /> Delete
								</Button>
							</div>
						</Card.Header>
					</Card.Root>
				{/each}
			</ul>
		{/if}
	</div>
</section>

<ProposalEditorDialog
	open={editorOpen}
	proposal={selectedProposal}
	{store}
	{primaryLocale}
	{supportedLocales}
	onOpenChange={(o) => {
		editorOpen = o;
		if (!o) {
			selectedProposal = null;
			// Silent reconcile: the dialog already flushed + reloaded on close, so a full refresh
			// would only flip the list to its loading state, tearing it down and jumping scroll to top.
			void store.reload();
		}
	}}
/>

<QuestionEditorDialog
	open={questionEditorOpen}
	question={selectedQuestion}
	{store}
	{toolConfig}
	target="proposal"
	onOpenChange={(o) => {
		questionEditorOpen = o;
		if (!o) selectedQuestion = null;
	}}
/>

<QuestionEditorDialog
	open={sectionQuestionEditorOpen}
	question={selectedSectionQuestion}
	{store}
	{toolConfig}
	target="section"
	onOpenChange={(o) => {
		sectionQuestionEditorOpen = o;
		if (!o) selectedSectionQuestion = null;
	}}
/>

<AlertDialog.Root
	open={deleteOpen}
	onOpenChange={(o) => {
		deleteOpen = o;
		if (!o) selectedProposal = null;
	}}
>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title>Delete proposal?</AlertDialog.Title>
			<AlertDialog.Description>
				This will permanently remove the proposal and any responses tied to it.
			</AlertDialog.Description>
		</AlertDialog.Header>
		<AlertDialog.Footer>
			<AlertDialog.Cancel disabled={deleting}>Cancel</AlertDialog.Cancel>
			<AlertDialog.Action disabled={deleting} onclick={runDelete}>
				{#if deleting}
					<LoaderCircle class="mr-2 h-4 w-4 animate-spin" />
				{/if}
				Delete
			</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>

<AlertDialog.Root
	open={questionDeleteOpen}
	onOpenChange={(o) => {
		questionDeleteOpen = o;
		if (!o) selectedQuestion = null;
	}}
>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title>Delete question?</AlertDialog.Title>
			<AlertDialog.Description>
				This will remove the question from this step. Existing responses to it will no
				longer be collected.
			</AlertDialog.Description>
		</AlertDialog.Header>
		<AlertDialog.Footer>
			<AlertDialog.Cancel disabled={deletingQuestionInFlight}>Cancel</AlertDialog.Cancel>
			<AlertDialog.Action disabled={deletingQuestionInFlight} onclick={runDeleteQuestion}>
				{#if deletingQuestionInFlight}
					<LoaderCircle class="mr-2 h-4 w-4 animate-spin" />
				{/if}
				Delete
			</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>

<AlertDialog.Root
	open={sectionQuestionDeleteOpen}
	onOpenChange={(o) => {
		sectionQuestionDeleteOpen = o;
		if (!o) selectedSectionQuestion = null;
	}}
>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title>Delete per-section question?</AlertDialog.Title>
			<AlertDialog.Description>
				This will remove the question from every section in this step. Existing responses to
				it will no longer be collected.
			</AlertDialog.Description>
		</AlertDialog.Header>
		<AlertDialog.Footer>
			<AlertDialog.Cancel disabled={deletingSectionQuestionInFlight}
				>Cancel</AlertDialog.Cancel
			>
			<AlertDialog.Action
				disabled={deletingSectionQuestionInFlight}
				onclick={runDeleteSectionQuestion}
			>
				{#if deletingSectionQuestionInFlight}
					<LoaderCircle class="mr-2 h-4 w-4 animate-spin" />
				{/if}
				Delete
			</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>
