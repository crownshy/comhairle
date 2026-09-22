<script lang="ts">
	// PROTOTYPE - throwaway. C: living map. A crowd of dots sits above a practice statement.
	// Every vote moves the whole crowd and drops the participant's own dot in, so the thing
	// Polis does with votes is watched, not explained.
	import { ThumbsUp, ThumbsDown, ArrowRight, Handshake } from 'lucide-svelte';
	import { fade, fly } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { Button } from '$lib/components/ui/button';
	import { haptic } from '$lib/utils/haptics';
	import { COPY, CROWD, place, tally, closestGroup, type Audience, type Vote } from './practice';

	let { audience, onDone }: { audience: Audience; onDone: () => void } = $props();

	const copy = $derived(COPY[audience]);
	let votes = $state<Vote[]>([]);
	const known = $derived(votes.length);
	const finished = $derived(known === 3);

	const headlines = [
		{ title: 'Each dot is a person.', body: 'Vote and watch where you land.' },
		{ title: 'See them move?', body: 'Your vote put you next to people who voted like you.' },
		{ title: 'Almost everyone agreed on that.', body: 'So almost everyone moved together.' },
		{ title: 'Two groups. You’re in one.', body: '' }
	];
	const headline = $derived(headlines[known]);

	const me = $derived(place(votes, known, { x: 50, y: 60 }, { x: 0, y: 0 }));
	const myGroup = $derived(closestGroup(votes));

	function vote(v: Vote) {
		if (finished) return;
		haptic(votes.length === 2 ? 'success' : 'light');
		votes.push(v);
	}
</script>

<div class="flex min-h-0 flex-1 flex-col gap-4">
	<div class="flex items-center justify-between gap-3">
		<span class="bg-primary/10 text-primary rounded-full px-3 py-1 text-base font-semibold">
			{finished ? 'Warm-up done' : `Warm-up ${known + 1} of 3 · doesn’t count`}
		</span>
		<Button variant="ghost" class="text-muted-foreground text-base" onclick={onDone}
			>Skip</Button
		>
	</div>

	{#key known}
		<div class="flex flex-col" in:fade={{ duration: 250 }}>
			<h2 class="text-foreground text-2xl leading-tight font-bold">{headline.title}</h2>
			{#if finished}
				<p class="text-muted-foreground text-lg">
					You’re with <span class="text-foreground font-semibold"
						>{copy.groupNames[myGroup]}</span
					>.
				</p>
			{:else}
				<p class="text-muted-foreground text-lg">{headline.body}</p>
			{/if}
		</div>
	{/key}

	<div class="bg-card relative shrink-0 overflow-hidden rounded-2xl border">
		<svg
			viewBox="0 0 100 70"
			class="block w-full"
			role="img"
			aria-label="Map of people by how they voted"
		>
			{#if finished}
				<ellipse
					cx="22"
					cy="21"
					rx="20"
					ry="17"
					class="fill-primary/10"
					in:fade={{ duration: 600 }}
				/>
				<ellipse
					cx="78"
					cy="21"
					rx="20"
					ry="17"
					class="fill-[var(--chart-2)]/15"
					in:fade={{ duration: 600 }}
				/>
			{/if}
			{#each CROWD as p (p.id)}
				{@const pos = place(
					p.votes,
					known,
					{ x: p.startX, y: p.startY },
					{ x: p.jitterX, y: p.jitterY }
				)}
				<g
					class="dot"
					style:transform="translate({pos.x}px, {pos.y}px)"
					style:transition-delay="{p.id * 12}ms"
				>
					<circle
						r="1.7"
						class="bob transition-colors duration-700 {finished
							? pos.x < 50
								? 'fill-primary/60'
								: 'fill-[var(--chart-2)]'
							: 'fill-muted-foreground/45'}"
						style:animation-delay="{-(p.id % 9) * 0.37}s"
					/>
				</g>
			{/each}
			{#if known > 0}
				<g class="dot" style:transform="translate({me.x}px, {me.y}px)" in:fade>
					<circle r="4.5" class="fill-primary/25 pulse" />
					<circle r="2.6" class="fill-primary stroke-background" stroke-width="0.8" />
					<text
						y="-4.6"
						text-anchor="middle"
						class="fill-foreground text-[3.4px] font-bold">You</text
					>
				</g>
			{/if}
		</svg>
	</div>

	<div
		class="bg-muted/50 flex min-h-0 flex-1 flex-col justify-center gap-5 overflow-y-auto rounded-2xl px-6 py-5"
	>
		{#if !finished}
			{#key known}
				<p
					class="text-card-foreground text-[clamp(1.25rem,3.2vh,2rem)] leading-tight font-bold"
					in:fly={{ y: 20, duration: 450, easing: cubicOut }}
				>
					{copy.practice[known]}
				</p>
			{/key}
			<div class="flex items-center justify-between gap-3">
				<button
					type="button"
					class="bg-primary text-primary-foreground flex h-16 flex-1 items-center justify-center gap-2 rounded-full text-lg font-semibold transition-transform active:scale-95"
					onclick={() => vote('agree')}
				>
					<ThumbsUp class="size-6" /> Agree
				</button>
				<button
					type="button"
					class="bg-primary text-primary-foreground flex h-16 flex-1 items-center justify-center gap-2 rounded-full text-lg font-semibold transition-transform active:scale-95"
					onclick={() => vote('disagree')}
				>
					<ThumbsDown class="size-6" /> Disagree
				</button>
			</div>
		{:else}
			<div class="flex flex-col gap-4" in:fly={{ y: 16, duration: 400, delay: 500 }}>
				<div class="flex items-start gap-3">
					<Handshake class="text-primary mt-1 size-6 shrink-0" />
					<p class="text-foreground text-lg">
						<span class="font-bold">Both groups</span> agreed: “{copy.practice[1]}”
						<span class="text-muted-foreground">({tally(1).agree}% of everyone)</span>
					</p>
				</div>
				<p class="text-foreground text-lg font-semibold">
					That’s what your votes are for. Next, real opinions from real people.
				</p>
				<Button size="lg" class="h-14 w-full rounded-2xl text-lg" onclick={onDone}>
					Start voting
					<ArrowRight class="size-5" />
				</Button>
			</div>
		{/if}
	</div>
</div>

<style>
	.dot {
		transition: transform 1100ms cubic-bezier(0.2, 0.8, 0.2, 1);
	}

	.bob {
		animation: bob 3.2s ease-in-out infinite;
	}

	.pulse {
		transform-origin: center;
		transform-box: fill-box;
		animation: pulse 1.8s ease-in-out infinite;
	}

	@keyframes bob {
		50% {
			transform: translateY(-0.8px);
		}
	}

	@keyframes pulse {
		50% {
			transform: scale(1.35);
			opacity: 0.4;
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.dot {
			transition: none;
		}
		.bob,
		.pulse {
			animation: none;
		}
	}
</style>
