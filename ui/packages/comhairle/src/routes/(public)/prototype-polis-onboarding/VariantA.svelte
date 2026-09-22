<script lang="ts">
	// PROTOTYPE - throwaway. A: poll reveal. Straight into three practice votes. Each tap
	// shows how everyone else voted, like a poll sticker in a story, because that reveal is
	// the reward people already expect after voting. Then one screen on what it all means.
	import { ThumbsUp, ThumbsDown, PenLine, Users, EyeOff, ArrowRight } from 'lucide-svelte';
	import { fade, fly } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { Button } from '$lib/components/ui/button';
	import { haptic } from '$lib/utils/haptics';
	import { COPY, tally, sameAs, closestGroup, type Audience, type Vote } from './practice';

	let { audience, onDone }: { audience: Audience; onDone: () => void } = $props();

	const copy = $derived(COPY[audience]);
	let index = $state(0);
	let votes = $state<Vote[]>([]);
	let screen = $state<'practice' | 'pattern' | 'deal'>('practice');

	const voted = $derived(votes.length > index);
	const result = $derived(tally(index));

	function vote(v: Vote) {
		if (voted) return;
		haptic('light');
		votes.push(v);
	}

	function next() {
		if (index < 2) {
			index++;
		} else {
			haptic('success');
			screen = 'pattern';
		}
	}

	const voteLabel: Record<Vote, string> = {
		agree: 'agreed',
		disagree: 'disagreed',
		pass: 'weren’t sure'
	};
</script>

<div class="flex min-h-0 flex-1 flex-col gap-4">
	<div class="flex items-center justify-between gap-3">
		<span class="bg-primary/10 text-primary rounded-full px-3 py-1 text-base font-semibold">
			{#if screen === 'practice'}
				Warm-up {index + 1} of 3 · doesn’t count
			{:else}
				30-second warm-up
			{/if}
		</span>
		<Button variant="ghost" class="text-muted-foreground text-base" onclick={onDone}>
			Skip
		</Button>
	</div>

	<div class="flex gap-1.5" aria-hidden="true">
		{#each [0, 1, 2, 3] as i (i)}
			{@const filled =
				screen === 'deal'
					? true
					: screen === 'pattern'
						? i < 3
						: i < index + (voted ? 1 : 0)}
			<div class="bg-muted h-1.5 flex-1 overflow-hidden rounded-full">
				<div
					class="bg-primary h-full rounded-full transition-[width] duration-500"
					style:width={filled ? '100%' : '0%'}
				></div>
			</div>
		{/each}
	</div>

	<div class="bg-muted/50 flex min-h-0 flex-1 flex-col overflow-y-auto rounded-2xl">
		{#if screen === 'practice'}
			{#key index}
				<div
					class="flex flex-1 flex-col justify-center gap-8 px-6 py-8"
					in:fade={{ duration: 250 }}
				>
					{#if index === 0 && !voted}
						<p class="text-muted-foreground text-lg">
							Tap a thumb. Then see how everyone else voted.
						</p>
					{/if}
					<p
						class="text-card-foreground text-[clamp(1.5rem,4vh,2.25rem)] leading-tight font-bold"
						in:fly={{ y: 20, duration: 500, easing: cubicOut }}
					>
						{copy.practice[index]}
					</p>

					{#if !voted}
						<div class="flex flex-col items-center gap-4">
							<div class="flex w-full max-w-[560px] items-center justify-between">
								<button
									type="button"
									class="bg-primary text-primary-foreground flex aspect-square w-[44%] max-w-[clamp(5rem,17vh,148px)] items-center justify-center rounded-full transition-transform active:scale-90"
									aria-label="Agree"
									onclick={() => vote('agree')}
								>
									<ThumbsUp class="size-[55%]" />
								</button>
								<button
									type="button"
									class="bg-primary text-primary-foreground flex aspect-square w-[44%] max-w-[clamp(5rem,17vh,148px)] items-center justify-center rounded-full transition-transform active:scale-90"
									aria-label="Disagree"
									onclick={() => vote('disagree')}
								>
									<ThumbsDown class="size-[55%]" />
								</button>
							</div>
							<Button
								variant="ghost"
								class="text-muted-foreground text-base"
								onclick={() => vote('pass')}
							>
								Not sure
							</Button>
						</div>
					{:else}
						<div class="flex flex-col gap-3" in:fade={{ duration: 200 }}>
							{#each [{ key: 'agree', label: 'Agree', Icon: ThumbsUp }, { key: 'disagree', label: 'Disagree', Icon: ThumbsDown }] as row (row.key)}
								{@const mine = votes[index] === row.key}
								{@const pct = result[row.key as Vote]}
								<div
									class="bg-background relative flex h-14 items-center overflow-hidden rounded-xl border-2 {mine
										? 'border-primary'
										: 'border-transparent'}"
								>
									<div
										class="grow-bar absolute inset-y-0 left-0 {mine
											? 'bg-primary/25'
											: 'bg-muted'}"
										style:width="{pct}%"
									></div>
									<div
										class="relative flex w-full items-center justify-between px-4 text-lg"
									>
										<span class="flex items-center gap-2 font-semibold">
											<row.Icon class="size-5" />
											{row.label}
											{#if mine}<span class="text-primary text-base"
													>· you</span
												>{/if}
										</span>
										<span class="font-bold tabular-nums">{pct}%</span>
									</div>
								</div>
							{/each}
							<p class="text-foreground text-lg">
								<span class="font-bold">{sameAs(index, votes[index])}%</span> of
								people {voteLabel[votes[index]]} too.
							</p>
							<Button
								size="lg"
								class="mt-2 h-14 w-full rounded-2xl text-lg"
								onclick={next}
							>
								{index < 2 ? 'Next one' : 'What did that show?'}
								<ArrowRight class="size-5" />
							</Button>
						</div>
					{/if}
				</div>
			{/key}
		{:else if screen === 'pattern'}
			{@const group = closestGroup(votes)}
			<div
				class="flex flex-1 flex-col justify-center gap-6 px-6 py-8"
				in:fade={{ duration: 300 }}
			>
				<h2 class="text-card-foreground text-2xl leading-tight font-bold">
					You just made two groups appear.
				</h2>
				<div class="grid grid-cols-2 gap-3">
					{#each copy.groupNames as name, g (name)}
						<div
							class="flex flex-col gap-3 rounded-2xl border-2 p-4 {g === group
								? 'border-primary bg-primary/10'
								: 'bg-background border-transparent'}"
							in:fly={{ y: 16, delay: 150 + g * 200, duration: 400 }}
						>
							<div class="flex flex-wrap gap-1">
								{#each Array(g === 0 ? 9 : 7) as _, i (i)}
									<span class="bg-muted-foreground/40 size-3 rounded-full"></span>
								{/each}
								{#if g === group}
									<span
										class="bg-primary ring-primary/30 size-3 rounded-full ring-4"
									></span>
								{/if}
							</div>
							<span class="text-foreground text-base font-semibold">{name}</span>
							{#if g === group}<span class="text-primary text-base font-semibold"
									>You’re here</span
								>{/if}
						</div>
					{/each}
				</div>
				<p class="text-foreground text-lg">{copy.pattern}</p>
				<div
					class="bg-background rounded-2xl p-4"
					in:fly={{ y: 16, delay: 600, duration: 400 }}
				>
					<p class="text-muted-foreground text-base font-semibold">
						But {tally(1).agree}% of everyone agreed on
					</p>
					<p class="text-foreground text-lg font-bold">“{copy.practice[1]}”</p>
				</div>
				<p class="text-foreground text-lg font-semibold">
					That’s the point: find the groups, then find what they share.
				</p>
				<Button
					size="lg"
					class="h-14 w-full rounded-2xl text-lg"
					onclick={() => (screen = 'deal')}
				>
					Got it
					<ArrowRight class="size-5" />
				</Button>
			</div>
		{:else}
			<div
				class="flex flex-1 flex-col justify-center gap-6 px-6 py-8"
				in:fade={{ duration: 300 }}
			>
				<h2 class="text-card-foreground text-3xl leading-tight font-bold">Now for real.</h2>
				<ul class="flex flex-col gap-4">
					{#each [{ Icon: ThumbsUp, title: 'Vote on other people’s opinions', body: 'No replies, no arguments.' }, { Icon: PenLine, title: 'Missing a view? Add your own', body: 'Everyone else gets to vote on it.' }, { Icon: Users, title: 'We look for common ground', body: 'Across everyone who takes part.' }, { Icon: EyeOff, title: 'Nobody sees how you voted', body: 'Only the overall patterns.' }] as item, i (item.title)}
						<li
							class="flex items-start gap-3"
							in:fly={{ x: -12, delay: i * 120, duration: 300 }}
						>
							<span
								class="bg-primary/10 text-primary flex size-10 shrink-0 items-center justify-center rounded-full"
							>
								<item.Icon class="size-5" />
							</span>
							<span class="flex flex-col">
								<span class="text-foreground text-lg font-semibold"
									>{item.title}</span
								>
								<span class="text-muted-foreground text-base">{item.body}</span>
							</span>
						</li>
					{/each}
				</ul>
				<Button size="lg" class="h-14 w-full rounded-2xl text-lg" onclick={onDone}>
					Start voting
					<ArrowRight class="size-5" />
				</Button>
			</div>
		{/if}
	</div>
</div>

<style>
	.grow-bar {
		transform-origin: left;
		animation: grow 700ms cubic-bezier(0.2, 0.8, 0.2, 1);
	}

	@keyframes grow {
		from {
			transform: scaleX(0);
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.grow-bar {
			animation: none;
		}
	}
</style>
