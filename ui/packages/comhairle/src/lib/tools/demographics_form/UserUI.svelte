<script lang="ts">
	import { notifications } from '$lib/notifications.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import type {
		ConversationDto,
		MyDemographicAnswer,
		QuestionDto
	} from '@crownshy/api-client/api';
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
	let answers: MyDemographicAnswer[] = $state.raw<MyDemographicAnswer[]>([]);

	let value: unknown;

	const getVersion = (question: QuestionDto) => question.versions.at(-1)?.id;

	async function move(questionNum: number) {
		const currentQuestion = questions[questionNum - 1];
		const version_id = getVersion(currentQuestion);
		if (!version_id || !value) {
			return;
		}

		const putDemographicResponse = await tryCatchAsync(() =>
			apiClient.PutDemographicResponse({ value }, { params: { version_id } })
		);
		if (putDemographicResponse.err !== null) {
			notifications.addFlash({
				message: 'Could not update value, please try again',
				priority: 'ERROR'
			});
		}
	}

	let sequence = $derived<ToolSequence>({
		next: questionNumber >= questions.length ? onDone : () => move(questionNumber++),
		previous: questionNumber <= 1 ? undefined : () => move(questionNumber--),
		progress: questionNumber / questions.length,
		position: `Question ${questionNumber} of ${questions.length}`
	});

	$effect(() => {
		onSequenceChange?.(sequence);
	});

	onMount(async () => {
		const answersResponse = await tryCatchAsync(() => apiClient.ListMyDemographicAnswers());
		if (answersResponse.err !== null) {
			notifications.addFlash({
				message: 'Could not load answers, please try again',
				priority: 'ERROR'
			});
			return;
		}
		answers = answersResponse.ok;

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
	<Question
		q={questions[questionNumber - 1].latest_published}
		onValue={(newValue) => void (value = newValue)}
		defaultValue={answers.find(
			(a) =>
				a.answer.question_version_id === questions[questionNumber - 1].versions.at(-1)?.id
		)?.answer.value}
	/>
{/if}
