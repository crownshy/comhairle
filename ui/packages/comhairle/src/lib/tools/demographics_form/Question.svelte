<script lang="ts" module>
	// eslint-disable-next-line svelte/prefer-svelte-reactivity
	let cache = new Map();
</script>

<script lang="ts">
	import { Input, SelectList, type Option } from '$lib/components/EasyForm';
	import type { QuestionDto } from '@crownshy/api-client/api';

	type Props = {
		q: QuestionDto['latest_published'] | undefined;
		onValue: (value: unknown) => void;
		defaultValue: unknown;
	};

	let { q, onValue, defaultValue }: Props = $props();

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
			<Input
				label={q.text}
				type="number"
				onblur={(e) => {
					const { value } = e.currentTarget;
					cache.set(q.id, value);
					onValue(Number(value));
				}}
				defaultValue={cache.get(q.id) ?? defaultValue}
			/>
		{/key}
 {:else if q.question_type === 'single_select' || q.question_type === 'multi_select'}
		{#key q.id}
			<SelectList
				title={q.text}
				name={q.id}
				multiple={q.question_type === 'multi_select'}
				options={getOptions(q)}
				onselect={(value) => {
					cache.set(q.id, value);
					onValue({ option_ids: value });
				}}
				defaultSelected={cache.get(q.id) ?? defaultValue?.option_ids}
			/>
				}}
				defaultSelected={cache.get(q.id) ?? defaultValue?.option_ids?.[0]}
			/>
		{/key}
	{:else if q.question_type === 'location'}
		{#key q.id}
			<Input
				label={q.text}
				onblur={(e) => {
					const { value } = e.currentTarget;
					cache.set(q.id, value);
					onValue(value);
				}}
				defaultValue={cache.get(q.id) ?? defaultValue}
			/>
		{/key}
	{:else}
		{#key q.id}
			<Input
				label={q.text}
				onblur={(e) => {
					const { value } = e.currentTarget;
					cache.set(q.id, value);
					onValue(value);
				}}
				defaultValue={cache.get(q.id) ?? defaultValue}
			/>
		{/key}
	{/if}
{/if}
