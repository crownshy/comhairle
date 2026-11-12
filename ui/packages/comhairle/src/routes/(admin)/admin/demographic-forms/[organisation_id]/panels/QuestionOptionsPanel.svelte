<script lang="ts">
	import Select from '$lib/components/EasyForm/Select/Select.svelte';
	import Label from '$lib/components/ui/label/label.svelte';
	import { Switch } from '$lib/components/ui/switch';
	import { capitalise } from '$lib/utils/casingUtils';
	import { QuestionType, type QuestionLatestDto } from '@crownshy/api-client/api';

	type Props = {
		questions: QuestionLatestDto[];
		selectedQuestionId: QuestionLatestDto['id'] | undefined;
	};

	let { questions = $bindable(), selectedQuestionId }: Props = $props();

	let question = $derived(questions.find((question) => question.id === selectedQuestionId));
</script>

{#snippet Toggle(label: string)}
	<Label class="text-muted-foreground flex max-w-60 justify-between text-sm font-semibold">
		{label}
		<Switch />
	</Label>
{/snippet}
<section class="flex flex-col gap-y-2">
	<span class="mb-2 text-sm font-semibold">Options</span>
	{#if question}
		<Select
			id={question.id}
			name={question.id}
			question={question.text}
			options={QuestionType.options.map((option) => ({
				label: capitalise(option),
				value: option
			}))}
			defaultSelected={question.questionType}
			bind:selected={question.questionType}
		/>
		{@render Toggle('Special category')}
		{@render Toggle('Allow other')}
	{/if}
</section>
