<script lang="ts">
	import type { SurveyQuestion } from './insights-loader';

	type Props = { questions: SurveyQuestion[] };

	let { questions }: Props = $props();
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
					<span class="text-muted-foreground w-5 shrink-0 tabular-nums">Q{i + 1}.</span>
					<span class="min-w-0 flex-1 truncate">{question.title}</span>
				</a>
			</li>
		{/each}
	</ol>
</nav>
