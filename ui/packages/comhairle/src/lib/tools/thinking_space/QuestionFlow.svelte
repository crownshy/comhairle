<script lang="ts">
	import { tick, untrack } from 'svelte';
	import { Button } from '$lib/components/ui/button';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Progress } from '$lib/components/ui/progress';
	import { CornerDownRight, Shuffle, Check } from 'lucide-svelte';
	import { generateFollowUpOptions, extractMockClaim } from './mockFollowups';
	import type {
		QuestionConfig,
		QuestionAnswers,
		ParticipantClaim,
		FollowUpAnswer
	} from './types';

	type Props = {
		topic: string;
		questions: QuestionConfig[];
		followUpCount: number;
		initialAnswers?: QuestionAnswers[];
		initialClaims?: ParticipantClaim[];
		onProgress?: (snapshot: { answers: QuestionAnswers[]; claims: ParticipantClaim[] }) => void;
		onComplete: (result: { answers: QuestionAnswers[]; claims: ParticipantClaim[] }) => void;
	};

	let {
		topic,
		questions,
		followUpCount,
		initialAnswers = [],
		initialClaims = [],
		onProgress,
		onComplete
	}: Props = $props();

	type Phase = 'main' | 'picking' | 'answering' | 'done';

	type LocalQuestionState = {
		mainAnswer: string;
		mainSubmitted: boolean;
		followUps: FollowUpAnswer[];
		picker: string[];
		currentPick: string;
		currentPickAnswer: string;
		phase: Phase;
	};

	function initialStateFor(qIdx: number): LocalQuestionState {
		const stored = initialAnswers[qIdx];
		if (!stored) {
			return {
				mainAnswer: '',
				mainSubmitted: false,
				followUps: [],
				picker: [],
				currentPick: '',
				currentPickAnswer: '',
				phase: 'main'
			};
		}
		const followUpsDone = stored.followUps.length;
		const needMore = followUpsDone < followUpCount;
		return {
			mainAnswer: stored.mainAnswer,
			mainSubmitted: true,
			followUps: stored.followUps,
			picker: needMore
				? generateFollowUpOptions(
						questions[qIdx]?.text ?? '',
						stored.followUps[followUpsDone - 1]?.answer ?? stored.mainAnswer
					)
				: [],
			currentPick: '',
			currentPickAnswer: '',
			phase: needMore ? 'picking' : 'done'
		};
	}

	let currentQIdx = $state(
		Math.min(
			Math.max(
				initialAnswers.findIndex((a) => a.followUps.length < followUpCount),
				0
			),
			questions.length - 1
		)
	);

	// If all stored questions are fully answered, start at the last one in done state
	if (
		initialAnswers.length === questions.length &&
		initialAnswers.every((a) => a.followUps.length >= followUpCount)
	) {
		currentQIdx = questions.length - 1;
	}

	let states = $state<LocalQuestionState[]>(questions.map((_, i) => initialStateFor(i)));
	let claims = $state<ParticipantClaim[]>([...initialClaims]);
	let transitioning = $state(false);

	let bottomEl = $state<HTMLDivElement | null>(null);
	let mainTextareaEl = $state<HTMLTextAreaElement | null>(null);
	let followUpTextareaEl = $state<HTMLTextAreaElement | null>(null);

	let currentState = $derived(states[currentQIdx]);
	let currentQuestion = $derived(questions[currentQIdx]);
	let followUpsDone = $derived(currentState.followUps.length);
	let followUpsRemaining = $derived(Math.max(0, followUpCount - followUpsDone));
	let isLastQuestion = $derived(currentQIdx === questions.length - 1);
	let allDone = $derived(isLastQuestion && currentState.phase === 'done');

	let totalSteps = $derived(questions.length * (1 + followUpCount));
	let completedSteps = $derived.by(() => {
		let n = 0;
		for (let i = 0; i < currentQIdx; i++) n += 1 + followUpCount;
		if (currentState.phase !== 'main') n += 1;
		n += followUpsDone;
		return Math.min(n, totalSteps);
	});
	let progress = $derived(totalSteps > 0 ? (completedSteps / totalSteps) * 100 : 0);

	function snapshot() {
		const answers: QuestionAnswers[] = states.map((s, i) => ({
			questionId: questions[i].id,
			mainAnswer: s.mainAnswer,
			followUps: s.followUps
		}));
		return { answers, claims };
	}

	function emitProgress() {
		onProgress?.(snapshot());
	}

	async function scrollToBottom() {
		await tick();
		bottomEl?.scrollIntoView({ behavior: 'smooth', block: 'end' });
	}

	$effect(() => {
		// Re-scroll when current phase or follow-up count changes
		const _ = currentState.phase + ':' + currentState.followUps.length + ':' + currentQIdx;
		untrack(() => {
			scrollToBottom();
		});
		// Focus textareas
		untrack(async () => {
			await tick();
			if (currentState.phase === 'main') mainTextareaEl?.focus();
			if (currentState.phase === 'answering') followUpTextareaEl?.focus();
		});
		// reference _ to satisfy linter
		void _;
	});

	function makeClaim(
		content: string,
		source: QuestionConfig | { id: string; text: string }
	): ParticipantClaim {
		return {
			id: `claim-${Date.now()}-${Math.random().toString(36).slice(2, 6)}`,
			content,
			sourceQuestionId: source.id,
			sourceQuestionText: source.text,
			status: 'pending'
		};
	}

	function advanceToNextQuestion() {
		if (isLastQuestion) {
			emitProgress();
			return;
		}
		transitioning = true;
		setTimeout(() => {
			currentQIdx = currentQIdx + 1;
			transitioning = false;
			emitProgress();
		}, 500);
	}

	function submitMainAnswer() {
		const value = currentState.mainAnswer.trim();
		if (!value) return;
		claims = [...claims, makeClaim(extractMockClaim(value), currentQuestion)];
		states[currentQIdx] = {
			...currentState,
			mainAnswer: value,
			mainSubmitted: true
		};
		if (followUpCount === 0) {
			states[currentQIdx].phase = 'done';
			emitProgress();
			advanceToNextQuestion();
		} else {
			states[currentQIdx].picker = generateFollowUpOptions(currentQuestion.text, value);
			states[currentQIdx].phase = 'picking';
			emitProgress();
		}
	}

	function pickFollowUp(q: string) {
		states[currentQIdx] = {
			...currentState,
			currentPick: q,
			currentPickAnswer: '',
			picker: currentState.picker.filter((x) => x !== q),
			phase: 'answering'
		};
	}

	function pickRandom() {
		const pool = currentState.picker;
		if (pool.length === 0) return;
		pickFollowUp(pool[Math.floor(Math.random() * pool.length)]);
	}

	function submitFollowUp() {
		const value = currentState.currentPickAnswer.trim();
		if (!value) return;
		const fu: FollowUpAnswer = { question: currentState.currentPick, answer: value };
		const updatedFollowUps = [...currentState.followUps, fu];
		claims = [
			...claims,
			makeClaim(extractMockClaim(value), {
				id: currentQuestion.id,
				text: currentState.currentPick
			})
		];
		const stillNeed = updatedFollowUps.length < followUpCount;
		states[currentQIdx] = {
			...currentState,
			followUps: updatedFollowUps,
			currentPick: '',
			currentPickAnswer: '',
			picker: stillNeed ? generateFollowUpOptions(currentQuestion.text, value) : [],
			phase: stillNeed ? 'picking' : 'done'
		};
		emitProgress();
		if (!stillNeed) advanceToNextQuestion();
	}

	function handleMainKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' && !e.shiftKey) {
			e.preventDefault();
			submitMainAnswer();
		}
	}

	function handleFollowUpKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' && !e.shiftKey) {
			e.preventDefault();
			submitFollowUp();
		}
	}

	function reviewNow() {
		onComplete(snapshot());
	}
</script>

<div class="flex h-full flex-col">
	<!-- Header / progress -->
	<div class="border-border bg-card/60 border-b px-6 py-4 backdrop-blur">
		<div class="mx-auto max-w-2xl">
			<div class="text-muted-foreground mb-2 flex items-center justify-between text-xs">
				<span class="font-medium tracking-wide uppercase">
					Thinking space · {topic}
				</span>
				<span>
					Question {currentQIdx + 1} of {questions.length} · {Math.round(progress)}%
				</span>
			</div>
			<Progress value={progress} class="h-1.5" />
		</div>
	</div>

	<!-- Scrollable content -->
	<div
		class="flex-1 overflow-y-auto px-6 py-8 transition-opacity duration-300"
		class:opacity-30={transitioning}
	>
		<div class="mx-auto max-w-2xl space-y-6">
			<!-- Main question -->
			<section>
				<p class="text-primary mb-2 text-xs font-semibold tracking-wide uppercase">
					Question {currentQIdx + 1}
				</p>
				<h2 class="text-foreground text-2xl leading-snug font-semibold">
					{currentQuestion.text || '(unnamed question)'}
				</h2>
			</section>

			{#if !currentState.mainSubmitted}
				<section class="space-y-3">
					<Textarea
						bind:ref={mainTextareaEl}
						bind:value={states[currentQIdx].mainAnswer}
						onkeydown={handleMainKeydown}
						placeholder="Write your thoughts here…"
						rows={4}
						class="text-base"
					/>
					<div class="flex justify-end">
						<Button
							onclick={submitMainAnswer}
							disabled={!currentState.mainAnswer.trim()}
						>
							Continue
						</Button>
					</div>
				</section>
			{:else}
				<section class="border-primary/20 bg-primary/5 rounded-xl border p-4">
					<p class="text-primary mb-1 text-xs font-semibold tracking-wide uppercase">
						You answered
					</p>
					<p class="text-foreground text-sm leading-relaxed whitespace-pre-wrap">
						{currentState.mainAnswer}
					</p>
				</section>
			{/if}

			<!-- Completed follow-ups -->
			{#each currentState.followUps as fu, fi (fi)}
				<section class="border-border space-y-2 border-t pt-6">
					<p
						class="text-muted-foreground flex items-center gap-1.5 text-xs font-semibold tracking-wide uppercase"
					>
						<CornerDownRight class="size-3.5" />
						Follow-up {fi + 1}
					</p>
					<p class="text-foreground/80 text-base leading-snug italic">
						{fu.question}
					</p>
					<div class="border-primary/20 bg-primary/5 rounded-lg border p-3">
						<p class="text-foreground text-sm leading-relaxed whitespace-pre-wrap">
							{fu.answer}
						</p>
					</div>
				</section>
			{/each}

			<!-- Picker -->
			{#if currentState.phase === 'picking'}
				<section class="border-border space-y-3 border-t pt-6">
					<div class="flex items-baseline justify-between gap-3">
						<div>
							<p class="text-foreground text-sm font-semibold">
								Pick a follow-up to continue
							</p>
							<p class="text-muted-foreground text-xs">
								{followUpsRemaining} more follow-up{followUpsRemaining === 1
									? ''
									: 's'} to go
							</p>
						</div>
					</div>
					<div class="space-y-2">
						{#each currentState.picker.slice(0, 5) as fq, i (fq + i)}
							<button
								type="button"
								onclick={() => pickFollowUp(fq)}
								class="border-border bg-card hover:border-primary hover:bg-primary/5 w-full rounded-lg border px-4 py-3 text-left text-sm leading-relaxed transition-colors"
							>
								{fq}
							</button>
						{/each}
					</div>
					<Button variant="secondary" size="sm" onclick={pickRandom}>
						<Shuffle class="size-3.5" />
						Pick one for me
					</Button>
				</section>
			{/if}

			<!-- Active follow-up answer input -->
			{#if currentState.phase === 'answering'}
				<section class="border-border space-y-3 border-t pt-6">
					<p
						class="text-muted-foreground flex items-center gap-1.5 text-xs font-semibold tracking-wide uppercase"
					>
						<CornerDownRight class="size-3.5" />
						Follow-up {followUpsDone + 1}
					</p>
					<p class="text-foreground text-lg leading-snug italic">
						{currentState.currentPick}
					</p>
					<Textarea
						bind:ref={followUpTextareaEl}
						bind:value={states[currentQIdx].currentPickAnswer}
						onkeydown={handleFollowUpKeydown}
						placeholder="Write your thoughts…"
						rows={4}
						class="text-base"
					/>
					<div class="flex justify-end">
						<Button
							onclick={submitFollowUp}
							disabled={!currentState.currentPickAnswer.trim()}
						>
							Continue
						</Button>
					</div>
				</section>
			{/if}

			{#if allDone}
				<section class="border-border space-y-4 border-t pt-8 text-center">
					<p class="text-foreground text-base">
						You've answered all the questions. Ready to review your views?
					</p>
					<Button size="lg" onclick={reviewNow}>
						<Check class="size-4" />
						Review my views
					</Button>
				</section>
			{/if}

			<div bind:this={bottomEl} class="h-1"></div>
		</div>
	</div>
</div>
