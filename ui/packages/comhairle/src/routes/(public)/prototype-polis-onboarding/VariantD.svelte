<script lang="ts">
	// PROTOTYPE - throwaway. D: myth or fact. Three quick true-or-false calls, each one a
	// thing first-timers get wrong about Polis. Guessing before being told makes people
	// read the answer, which a plain list of rules does not.
	import { Check, X, ThumbsUp, ThumbsDown, ArrowRight, Sparkles, PenLine } from 'lucide-svelte';
	import { fade, fly, scale } from 'svelte/transition';
	import { Button } from '$lib/components/ui/button';
	import { haptic } from '$lib/utils/haptics';
	import { COPY, sameAs, type Audience, type Vote } from './practice';

	let { audience, onDone }: { audience: Audience; onDone: () => void } = $props();

	const copy = $derived(COPY[audience]);

	const questions = [
		{
			claim: 'You can reply to other people’s opinions.',
			answer: false,
			reveal: 'No replies, so no pile-ons. You just agree or disagree. Try it:'
		},
		{
			claim: 'Other people can see how you voted.',
			answer: false,
			reveal: 'Nobody sees your votes. We only look at the patterns across everyone.'
		},
		{
			claim: 'If your view is missing, you can add it.',
			answer: true,
			reveal: 'Write one short opinion. Everyone else then gets to vote on it.'
		}
	];

	let index = $state(0);
	let answers = $state<boolean[]>([]);
	let demoVote = $state<Vote | null>(null);

	const answered = $derived(answers.length > index);
	const question = $derived(questions[index]);
	const correct = $derived(answered && answers[index] === question.answer);
	const done = $derived(index >= questions.length);
	const score = $derived(answers.filter((a, i) => a === questions[i].answer).length);

	function guess(value: boolean) {
		if (answered) return;
		answers.push(value);
		haptic(value === question.answer ? 'success' : 'medium');
	}
</script>

<div class="flex min-h-0 flex-1 flex-col gap-4">
	<div class="flex items-center justify-between gap-3">
		<span class="bg-primary/10 text-primary rounded-full px-3 py-1 text-base font-semibold">
			{done ? 'Quiz done' : `Myth or fact? ${index + 1} of 3 · 20 sec`}
		</span>
		<Button variant="ghost" class="text-muted-foreground text-base" onclick={onDone}
			>Skip</Button
		>
	</div>

	<div class="flex gap-1.5" aria-hidden="true">
		{#each questions as q, i (q.claim)}
			<div
				class="h-1.5 flex-1 rounded-full transition-colors duration-300 {i < answers.length
					? answers[i] === q.answer
						? 'bg-primary'
						: 'bg-muted-foreground/50'
					: 'bg-muted'}"
			></div>
		{/each}
	</div>

	<div class="bg-muted/50 flex min-h-0 flex-1 flex-col overflow-y-auto rounded-2xl">
		{#if !done}
			{#key index}
				<div
					class="flex flex-1 flex-col justify-center gap-6 px-6 py-8"
					in:fly={{ x: 40, duration: 350 }}
				>
					<p class="text-muted-foreground text-lg font-semibold">True or false?</p>
					<p
						class="text-card-foreground text-[clamp(1.5rem,4vh,2.25rem)] leading-tight font-bold"
					>
						{question.claim}
					</p>

					<div class="grid grid-cols-2 gap-3">
						{#each [true, false] as value (value)}
							{@const picked = answered && answers[index] === value}
							{@const isAnswer = question.answer === value}
							<button
								type="button"
								disabled={answered}
								class="flex h-16 items-center justify-center gap-2 rounded-2xl border-2 text-xl font-bold transition-all
									{!answered ? 'bg-background border-border active:scale-95' : ''}
									{answered && isAnswer ? 'bg-primary text-primary-foreground border-primary' : ''}
									{answered && !isAnswer ? 'bg-background border-border opacity-50' : ''}
									{picked && !isAnswer ? 'shake' : ''}"
								onclick={() => guess(value)}
							>
								{#if answered && isAnswer}
									<span in:scale={{ duration: 250 }}
										><Check class="size-6" /></span
									>
								{:else if picked}
									<X class="size-6" />
								{/if}
								{value ? 'True' : 'False'}
							</button>
						{/each}
					</div>

					{#if answered}
						<div class="flex flex-col gap-4" in:fade={{ duration: 250 }}>
							<p class="text-foreground text-lg">
								<span class="font-bold">{correct ? 'Yep.' : 'Not quite.'}</span>
								{question.reveal}
							</p>

							{#if index === 0}
								<div class="bg-background flex flex-col gap-4 rounded-2xl p-4">
									<p class="text-foreground text-lg font-bold">
										{copy.practice[0]}
									</p>
									{#if !demoVote}
										<div class="flex gap-3">
											<button
												type="button"
												class="bg-primary text-primary-foreground flex h-12 flex-1 items-center justify-center gap-2 rounded-full text-base font-semibold active:scale-95"
												onclick={() => (demoVote = 'agree')}
											>
												<ThumbsUp class="size-5" /> Agree
											</button>
											<button
												type="button"
												class="bg-primary text-primary-foreground flex h-12 flex-1 items-center justify-center gap-2 rounded-full text-base font-semibold active:scale-95"
												onclick={() => (demoVote = 'disagree')}
											>
												<ThumbsDown class="size-5" /> Disagree
											</button>
										</div>
									{:else}
										<p class="text-foreground text-base" in:fade>
											<span class="font-bold">{sameAs(0, demoVote)}%</span> of people
											voted the same way as you. That’s all a vote is.
										</p>
									{/if}
								</div>
							{:else if index === 2}
								<div class="bg-background flex items-center gap-3 rounded-2xl p-4">
									<PenLine class="text-primary size-6 shrink-0" />
									<p class="text-foreground text-base">
										Your opinion
										<ArrowRight class="inline size-4" />
										<span class="font-semibold"
											>voted on by everyone who comes after you</span
										>
									</p>
								</div>
							{/if}

							<Button
								size="lg"
								class="h-14 w-full rounded-2xl text-lg"
								disabled={index === 0 && !demoVote}
								onclick={() => index++}
							>
								{index < 2 ? 'Next' : 'See my score'}
								<ArrowRight class="size-5" />
							</Button>
						</div>
					{/if}
				</div>
			{/key}
		{:else}
			<div
				class="flex flex-1 flex-col items-center justify-center gap-5 px-6 py-8 text-center"
				in:fade
			>
				<span in:scale={{ duration: 400, start: 0.4 }}>
					<Sparkles class="text-primary size-12" />
				</span>
				<h2 class="text-card-foreground text-4xl font-bold">{score} of 3</h2>
				<p class="text-foreground max-w-[28ch] text-lg">
					{score === 3
						? 'You already get Polis better than most people.'
						: 'Now you know. That’s honestly all there is to it.'}
				</p>
				<p class="text-muted-foreground max-w-[32ch] text-base">
					Vote on opinions. Add yours if it’s missing. We find what people agree on.
				</p>
				<Button
					size="lg"
					class="h-14 w-full max-w-[360px] rounded-2xl text-lg"
					onclick={onDone}
				>
					Start voting
					<ArrowRight class="size-5" />
				</Button>
			</div>
		{/if}
	</div>
</div>

<style>
	.shake {
		animation: shake 360ms ease-in-out;
	}

	@keyframes shake {
		25% {
			transform: translateX(-6px);
		}
		75% {
			transform: translateX(6px);
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.shake {
			animation: none;
		}
	}
</style>
