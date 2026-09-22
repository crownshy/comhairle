<script lang="ts">
	// PROTOTYPE - throwaway. B: coach marks. No intro at all. The participant lands on the
	// real voting screen and a spotlight walks them round it, one control at a time, with
	// the first statement as a practice one. For people who close every onboarding carousel.
	import { ArrowRight } from 'lucide-svelte';
	import { fade } from 'svelte/transition';
	import { Button } from '$lib/components/ui/button';
	import { haptic } from '$lib/utils/haptics';
	import FakeVoting from './FakeVoting.svelte';
	import { COPY, sameAs, type Audience, type Vote } from './practice';

	let { audience, onDone }: { audience: Audience; onDone: () => void } = $props();

	let statementEl = $state<HTMLElement>();
	let thumbsEl = $state<HTMLElement>();
	let addEl = $state<HTMLElement>();
	let counterEl = $state<HTMLElement>();

	type Mark = {
		target: () => HTMLElement | undefined;
		title: string;
		body: string;
		waitsForVote?: boolean;
	};

	const marks: Mark[] = [
		{
			target: () => statementEl,
			title: 'This is someone’s opinion',
			body: 'Not a quiz question. A person wrote it. Your job is to react.'
		},
		{
			target: () => thumbsEl,
			title: 'Agree or disagree?',
			body: 'Go on, tap one. This one is practice.',
			waitsForVote: true
		},
		{
			target: () => addEl,
			title: 'Your view not here?',
			body: 'Add it. Then everyone else votes on yours.'
		},
		{
			target: () => counterEl,
			title: 'No replies. No pile-ons.',
			body: 'Just votes. We use them to find what people agree on.'
		}
	];

	let step = $state(0);
	let practiceVote = $state<Vote | null>(null);
	let rect = $state<DOMRect | null>(null);
	let viewportHeight = $state(800);

	const mark = $derived(marks[step]);
	const below = $derived(rect ? rect.bottom + 200 < viewportHeight : true);

	function measure() {
		rect = mark.target()?.getBoundingClientRect() ?? null;
	}

	// Reading layout from the DOM, which is what an effect is for: the spotlight has to
	// follow the element it points at when the step or the viewport changes.
	$effect(() => {
		void step;
		void statementEl;
		requestAnimationFrame(measure);
	});

	function next() {
		haptic('light');
		if (step < marks.length - 1) step++;
		else onDone();
	}

	function onVote(v: Vote) {
		if (!mark.waitsForVote || practiceVote) return;
		practiceVote = v;
		setTimeout(() => step++, 1600);
	}
</script>

<svelte:window bind:innerHeight={viewportHeight} onresize={measure} onscroll={measure} />

<FakeVoting
	statement={COPY[audience].practice[0]}
	counter="Opinion 1 of 24"
	tag={step < 3 ? 'Practice · doesn’t count' : undefined}
	{onVote}
	bind:statementEl
	bind:thumbsEl
	bind:addEl
	bind:counterEl
/>

{#if rect}
	<!-- The spotlight is a hole cut by a huge shadow. It ignores the pointer, so the thumbs
	     under it stay tappable on the practice step. -->
	<div
		class="pointer-events-none fixed z-40 rounded-2xl ring-4 ring-white/80 transition-all duration-300 ease-out"
		style:left="{rect.left - 8}px"
		style:top="{rect.top - 8}px"
		style:width="{rect.width + 16}px"
		style:height="{rect.height + 16}px"
		style:box-shadow="0 0 0 9999px rgb(0 0 0 / 0.62)"
	></div>

	{#key step}
		<div
			class="bg-card text-card-foreground fixed left-1/2 z-50 flex w-[min(24rem,calc(100vw-2rem))] -translate-x-1/2 flex-col gap-2 rounded-2xl p-5 shadow-2xl"
			style:top={below ? `${rect.bottom + 20}px` : undefined}
			style:bottom={below ? undefined : `${viewportHeight - rect.top + 20}px`}
			in:fade={{ duration: 200 }}
		>
			<span class="text-muted-foreground text-base font-medium"
				>{step + 1} of {marks.length}</span
			>
			<p class="text-xl font-bold">{mark.title}</p>
			{#if mark.waitsForVote && practiceVote}
				<p class="text-lg" in:fade>
					Nice. <span class="font-bold">{sameAs(0, practiceVote)}%</span> of people voted the
					same way.
				</p>
			{:else}
				<p class="text-lg">{mark.body}</p>
			{/if}
			<div class="mt-2 flex items-center justify-between">
				<Button variant="ghost" class="text-muted-foreground text-base" onclick={onDone}>
					Skip tour
				</Button>
				{#if !mark.waitsForVote}
					<Button class="h-11 rounded-xl text-base" onclick={next}>
						{step === marks.length - 1 ? 'Start voting' : 'Next'}
						<ArrowRight class="size-4" />
					</Button>
				{/if}
			</div>
		</div>
	{/key}
{/if}
