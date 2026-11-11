<script lang="ts">
	import Label from '$lib/components/ui/label/label.svelte';
	import { Switch } from '$lib/components/ui/switch';
	import type { QuestionLatestDto } from '@crownshy/api-client/api';

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
		{@render Toggle('Special category')}
		{@render Toggle('Allow other')}
	{/if}
</section>
