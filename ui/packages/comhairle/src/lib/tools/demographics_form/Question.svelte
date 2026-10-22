<script lang="ts">
	import { Input, SelectList, type Option } from '$lib/components/EasyForm';
	import type { QuestionDto } from '@crownshy/api-client/api';

	type Props = {
		q: QuestionDto['latest_published'] | undefined;
	};

	let { q }: Props = $props();

	function getOptions(question: NonNullable<QuestionDto['latest_published']>): Option[] {
		const options: Option[] = question.options.map((o) => ({
			label: o.label,
			value: o.id
		}));

		if (question.allow_other) options.push({ label: 'Other', value: 'other' });

		if (question.prefer_not_to_say)
			options.push({ label: 'Prefer not to say', value: 'prefer_not_to_say' });

		return options;
	}
</script>

{#if q}
	{#if q.question_type === 'number'}
		{#key q.id}
			<Input label={q.text} type="number" />
		{/key}
	{:else if q.question_type === 'single_select'}
		<SelectList title={q.text} name={q.id} options={getOptions(q)} onselect={() => {}} />
	{:else}
		{#key q.id}
			<Input label={q.text} />
		{/key}
	{/if}
{/if}
