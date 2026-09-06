<script lang="ts">
	import { CornerDownRight, ArrowRight, Check } from 'lucide-svelte';
	import * as m from '$lib/paraglide/messages';
	import type { FlowMode } from './questionFlowState.svelte';

	let {
		mode,
		questionNumber,
		total,
		followUpsDone,
		isLastQuestion,
		onDeeper,
		onMoveOn
	}: {
		mode: FlowMode;
		questionNumber: number;
		total: number;
		followUpsDone: number;
		/** Decides whether moving on is the next question or the end of the flow. */
		isLastQuestion: boolean;
		onDeeper: () => void;
		onMoveOn: () => void;
	} = $props();

	// Extension mode has no "next question": leaving a chain returns to the root picker.
	let leaving = $derived.by(() => {
		if (mode === 'extension') {
			return {
				label: m.thinking_space_question_done(),
				description: m.thinking_space_crossroads_done_desc(),
				icon: Check
			};
		}
		if (isLastQuestion) {
			return {
				label: m.thinking_space_crossroads_finish(),
				description: m.thinking_space_crossroads_finish_desc(),
				icon: Check
			};
		}
		return {
			label: m.thinking_space_crossroads_move_on({ next: questionNumber + 1 }),
			description: m.thinking_space_crossroads_move_on_desc(),
			icon: ArrowRight
		};
	});

	let LeavingIcon = $derived(leaving.icon);
</script>

<!-- The fork. Carrying on and moving on are the same size and sit next to each other, because
     past the configured follow-ups neither one is the expected answer. -->
<section class="mx-auto flex w-full max-w-2xl flex-1 flex-col justify-center gap-6 py-8">
	<header>
		{#if mode === 'initial'}
			<p class="text-primary text-base font-medium">
				{m.thinking_space_question_position({ current: questionNumber, total })}
			</p>
		{/if}
		<h2
			class="text-foreground mt-1.5 text-2xl leading-snug font-semibold sm:text-3xl sm:leading-tight"
		>
			{m.thinking_space_crossroads_heading()}
		</h2>
		<p class="text-muted-foreground mt-3 text-base leading-relaxed">
			{#if followUpsDone === 1}
				{m.thinking_space_crossroads_count_one()}
			{:else}
				{m.thinking_space_crossroads_count_many({ count: followUpsDone })}
			{/if}
		</p>
	</header>

	<div class="flex flex-col gap-3">
		<button
			type="button"
			onclick={onDeeper}
			class="border-border bg-card hover:border-primary hover:bg-accent flex w-full items-start gap-3 rounded-xl border px-4 py-4 text-left transition-colors"
		>
			<CornerDownRight class="text-primary mt-0.5 size-5 shrink-0" />
			<span class="min-w-0 flex-1">
				<span class="text-foreground block text-base leading-snug font-medium">
					{m.thinking_space_crossroads_deeper()}
				</span>
				<span class="text-muted-foreground mt-1 block text-base leading-relaxed">
					{m.thinking_space_crossroads_deeper_desc()}
				</span>
			</span>
		</button>

		<button
			type="button"
			onclick={onMoveOn}
			class="border-border bg-card hover:border-primary hover:bg-accent flex w-full items-start gap-3 rounded-xl border px-4 py-4 text-left transition-colors"
		>
			<LeavingIcon class="text-primary mt-0.5 size-5 shrink-0" />
			<span class="min-w-0 flex-1">
				<span class="text-foreground block text-base leading-snug font-medium">
					{leaving.label}
				</span>
				<span class="text-muted-foreground mt-1 block text-base leading-relaxed">
					{leaving.description}
				</span>
			</span>
		</button>
	</div>
</section>
