<script lang="ts">
	import type { SurveyQuestion } from './insights-loader';

	type Props = { questions: SurveyQuestion[] };

	let { questions }: Props = $props();

	const isSkipped = (question: SurveyQuestion) => question.answered < question.total;
	const anySkipped = $derived(questions.some(isSkipped));
</script>

<nav aria-label="Questions" class="bg-card sticky top-6 flex flex-col gap-1 rounded-sm border p-2">
	<p class="flex items-baseline justify-between px-3 pt-2 pb-1 font-semibold">
		Questions
		<span class="text-muted-foreground text-sm font-normal">{questions.length}</span>
	</p>
	<ol class="flex flex-col">
		{#each questions as question, i (question.id)}
			<li>
				<a
					href="#question-{question.id}"
					title={question.title}
					class="hover:bg-muted flex min-h-10 items-center gap-3 rounded-sm px-3"
				>
					<span class="text-muted-foreground w-5 shrink-0 tabular-nums">{i + 1}</span>
					<span class="min-w-0 flex-1 truncate">{question.title}</span>
					{#if isSkipped(question)}
						<span class="size-2 shrink-0 rounded-full bg-amber-600" aria-hidden="true"
						></span>
						<span class="sr-only">(some people skipped)</span>
					{/if}
				</a>
			</li>
		{/each}
	</ol>
	{#if anySkipped}
		<p class="text-muted-foreground border-t px-3 pt-2 pb-1 text-sm">
			<span class="mr-2 inline-block size-2 rounded-full bg-amber-600" aria-hidden="true"
			></span>
			Some people skipped
		</p>
	{/if}
</nav>
