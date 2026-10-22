<script lang="ts">
	import { Input, SelectList, type Option } from '$lib/components/EasyForm';
	import { notifications } from '$lib/notifications.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import type { ConversationDto, QuestionDto } from '@crownshy/api-client/api';
	import { apiClient } from '@crownshy/api-client/client';
	import { onMount } from 'svelte';

	type Props = {
		organizationId: ConversationDto['organizationId'];
		formId: string;
	};

	let { organizationId, formId }: Props = $props();

	let questions = $state.raw<QuestionDto[]>([]);

	onMount(async () => {
		const demographicForm = await tryCatchAsync(() =>
			apiClient.GetDemographicForm({
				queries: { organization: organizationId },
				params: { form_id: formId }
			})
		);

		if (demographicForm.err !== null) {
			notifications.addFlash({
				message: 'Could not load demographic form, please try again',
				priority: 'ERROR'
			});
			return;
		}

		const questionsResponses = await Promise.all(
			demographicForm.ok.latest_published?.entries.map((entry) =>
				tryCatchAsync(() =>
					apiClient.GetDemographicQuestion({
						queries: { organization: organizationId },
						params: { question_id: entry.question_id }
					})
				)
			) ?? []
		);

		questions = questionsResponses.map((r) => r.ok).filter((q) => q !== null);
	});

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

{#each questions as question (question?.id)}
	{@const q = question.latest_published}
	{#if q}
		{#if q.question_type === 'number'}
			<Input label={q.text} type="number" />
		{:else if question.latest_published?.question_type === 'any_string'}
			<Input label={q.text} />
		{:else if question.latest_published?.question_type === 'single_select'}
			<SelectList title={q.text} name={q.id} options={getOptions(q)} onselect={() => {}} />
		{/if}
	{/if}
{/each}
