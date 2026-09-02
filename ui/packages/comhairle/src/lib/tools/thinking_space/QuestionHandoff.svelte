<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { ArrowRight } from 'lucide-svelte';
	import * as m from '$lib/paraglide/messages';
	import type { Handoff } from './questionFlowState.svelte';

	let {
		variant,
		questionNumber,
		total,
		question,
		followUpCount,
		onStart
	}: {
		/** Which card this is: the one that opens the flow, or the one between questions. */
		variant: Exclude<Handoff, null>;
		questionNumber: number;
		total: number;
		question: string;
		/** Zero means there are no follow-ups to describe, so the copy drops that half. */
		followUpCount: number;
		onStart: () => void;
	} = $props();

	let heading = $derived(
		variant === 'intro' ? m.thinking_space_intro_heading() : m.thinking_space_handoff_heading()
	);

	let description = $derived.by(() => {
		if (variant === 'intro') {
			return followUpCount > 0
				? m.thinking_space_intro_desc()
				: m.thinking_space_intro_desc_plain();
		}
		return followUpCount > 0
			? m.thinking_space_handoff_desc()
			: m.thinking_space_handoff_desc_plain();
	});

	let startLabel = $derived(
		variant === 'intro' ? m.thinking_space_intro_start() : m.thinking_space_handoff_start()
	);
</script>

<!-- The pause between questions. The next question is on the card rather than waiting behind
     the button, so moving on is a decision about something the participant has read. -->
<section class="mx-auto flex w-full max-w-2xl flex-1 flex-col justify-center gap-6 py-8">
	<header>
		<h2
			class="text-foreground text-2xl leading-snug font-semibold sm:text-3xl sm:leading-tight"
		>
			{heading}
		</h2>
		<p class="text-muted-foreground mt-3 text-base leading-relaxed">
			{description}
		</p>
	</header>

	<div class="border-border bg-card rounded-xl border px-4 py-4">
		<p class="text-primary text-base font-medium">
			{m.thinking_space_question_position({ current: questionNumber, total })}
		</p>
		<p class="text-foreground mt-2 text-lg leading-snug font-medium">
			{question}
		</p>
	</div>

	<div class="flex justify-end">
		<Button size="lg" onclick={onStart}>
			{startLabel}
			<ArrowRight class="size-4" />
		</Button>
	</div>
</section>
