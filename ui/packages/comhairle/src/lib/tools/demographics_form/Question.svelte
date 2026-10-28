<script lang="ts" module>
	// eslint-disable-next-line svelte/prefer-svelte-reactivity
	let cache = new Map();
</script>

<script lang="ts">
	import Callout from '$lib/components/Callout.svelte';

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
	{#key q.id}
		{#if q.special_category}
			<Callout variant="primary">
				<input id="special-consent" type="checkbox" />
				<label for="special-consent" class="pl-3">
					{q.consent_text ??
						'This question requires additional content as it contains sensitive information'}
				</label>
			</Callout>
		{/if}
		{#if q.question_type === 'number'}
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
		{:else if q.question_type === 'single_select' || q.question_type === 'multi_select'}
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
		{:else if q.question_type === 'date'}
			<Input
				type="date"
				label={q.text}
				onblur={(e) => {
					const value = e.currentTarget.value.replaceAll('-', '/');
					cache.set(q.id, value);
					onValue(value);
				}}
				defaultValue={cache.get(q.id) ?? defaultValue.replaceAll('/', '-')}
			/>
		{:else if q.question_type === 'location'}
			<Input
				label={q.text}
				onblur={(e) => {
					const { value } = e.currentTarget;
					cache.set(q.id, value);
					onValue(value);
				}}
				defaultValue={cache.get(q.id) ?? defaultValue}
			/>
		{:else}
			<!-- types "any_string" and "custom" -->
			<Input
				label={q.text}
				onblur={(e) => {
					const { value } = e.currentTarget;
					cache.set(q.id, value);
					onValue(value);
				}}
				defaultValue={cache.get(q.id) ?? defaultValue}
			/>
		{/if}
	{/key}
{/if}
