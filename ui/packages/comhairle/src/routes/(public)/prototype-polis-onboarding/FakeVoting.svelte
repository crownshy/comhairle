<script lang="ts">
	// PROTOTYPE - throwaway. A stand-in for the voting screen in PolisEmbed.svelte, close
	// enough that the hand-off from onboarding to the real thing can be judged.
	import { ThumbsUp, ThumbsDown, MessageSquare } from 'lucide-svelte';
	import { fly } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { Button } from '$lib/components/ui/button';
	import { haptic } from '$lib/utils/haptics';
	import type { Vote } from './practice';

	let {
		statement,
		counter,
		tag,
		onVote,
		statementEl = $bindable(),
		thumbsEl = $bindable(),
		addEl = $bindable(),
		counterEl = $bindable()
	}: {
		statement: string;
		counter: string;
		tag?: string;
		onVote?: (vote: Vote) => void;
		statementEl?: HTMLElement;
		thumbsEl?: HTMLElement;
		addEl?: HTMLElement;
		counterEl?: HTMLElement;
	} = $props();

	function vote(v: Vote) {
		haptic('light');
		onVote?.(v);
	}
</script>

<div class="flex min-h-0 w-full flex-1 flex-col gap-3">
	<div class="flex items-center justify-between">
		<span
			bind:this={counterEl}
			class="text-muted-foreground bg-muted rounded-full px-3 py-1 text-base font-medium"
		>
			{counter}
		</span>
	</div>
	<div class="bg-muted/50 flex min-h-0 w-full flex-1 flex-col rounded-2xl">
		<div class="flex flex-1 flex-col justify-center gap-10 px-6 py-8">
			<div bind:this={statementEl} class="flex flex-col gap-2 rounded-xl">
				{#if tag}
					<span
						class="bg-primary/10 text-primary w-fit rounded-full px-3 py-0.5 text-base font-semibold"
					>
						{tag}
					</span>
				{/if}
				{#key statement}
					<p
						class="text-card-foreground text-[clamp(1.5rem,4vh,2.25rem)] leading-tight font-bold"
						in:fly={{ y: 20, duration: 500, easing: cubicOut }}
					>
						{statement}
					</p>
				{/key}
			</div>
			<div
				bind:this={thumbsEl}
				class="flex w-full max-w-[560px] items-center justify-between"
			>
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
		</div>
		<div
			bind:this={addEl}
			class="bg-background flex w-full shrink-0 flex-col items-center gap-1 rounded-b-2xl border-t px-6 py-3"
		>
			<p class="text-muted-foreground text-base">
				Think your view is missing from the conversation?
			</p>
			<Button
				variant="ghost"
				class="text-primary flex items-center gap-2 text-base font-semibold"
			>
				<MessageSquare class="h-5 w-5" />
				Add your own
			</Button>
		</div>
	</div>
</div>
