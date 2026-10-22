<script lang="ts">
	import { notifications } from '$lib/notifications.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import type { ConversationDto, QuestionDto } from '@crownshy/api-client/api';
	import { apiClient } from '@crownshy/api-client/client';
	import { onMount } from 'svelte';
	import type { OnSequenceChange, ToolSequence } from '../toolSequence';
	import Question from './Question.svelte';

	type Props = {
		organizationId: ConversationDto['organizationId'];
		formId: string;
		onDone: () => void;
		onSequenceChange: OnSequenceChange;
	};

	let { organizationId, formId, onDone, onSequenceChange }: Props = $props();

	let questions = $state.raw<QuestionDto[]>([]);
	let questionNumber = $state<number>(1);

	let sequence = $derived<ToolSequence>({
		next: questionNumber >= questions.length ? onDone : () => questionNumber++,
		previous: questionNumber <= 1 ? undefined : () => questionNumber--,
		progress: questionNumber / questions.length,
		position: `Question ${questionNumber} of ${questions.length}`
	});

	$effect(() => {
		onSequenceChange?.(sequence);
	});

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
</script>

{#if questions.length > 0}
	<Question q={questions[questionNumber - 1].latest_published} />
{/if}
