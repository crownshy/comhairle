<script lang="ts">
	import { notifications } from '$lib/notifications.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import type {
		ConsentState,
		QuestionLatestDto,
		ConversationDto,
		MyDemographicAnswer,
		QuestionType
	} from '@crownshy/api-client/api';
	import { apiClient } from '@crownshy/api-client/client';
	import { onMount } from 'svelte';
	import type { OnSequenceChange, ToolSequence } from '../toolSequence';
	import Callout from '$lib/components/Callout.svelte';
	import {
		Form,
		Input,
		SelectList,
		Submit,
		Textarea,
		type Option
	} from '$lib/components/EasyForm';
	import Formatter from './Formatter';
	import { typed } from '$lib/utils/types';

	type Props = {
		organizationId: ConversationDto['organizationId'];
		formId: string;
		onDone: () => void;
		onSequenceChange: OnSequenceChange;
	};

	let { organizationId, formId, onDone, onSequenceChange }: Props = $props();

	let questions = $state.raw<QuestionLatestDto[]>([]);
	let questionNumber = $state<number>(1);
	let answers = $state.raw<MyDemographicAnswer[]>([]);

	// eslint-disable-next-line svelte/prefer-svelte-reactivity
	let cache = new Map();

	let submit = $state<HTMLButtonElement | null>(null);

	async function handleSubmission(formData: FormData) {
		const question = questions[questionNumber - 1];
		if (!question) {
			return;
		}

		const type = formData.get('question_type') as QuestionType;
		const consent = typed<ConsentState | null>(formData.get('consent') as 'granted' | null);
		const value = formData.getAll(question.id ?? '') as string[] | null;
		if (!value || (value.length === 1 && value[0] === '')) {
			return;
		}

		const processedValue = Formatter.from(type, value);

		cache.set(question.id, processedValue);

		if (value.length > 0 && question.special_category && consent !== 'granted') {
			notifications.addFlash({
				message: 'Special consent is needed for this question',
				priority: 'ERROR'
			});
			return;
		}

		const putDemographicResponse = await tryCatchAsync(() =>
			apiClient.PutDemographicResponse(
				{ value: processedValue },
				{ params: { version_id: version.id } }
			)
		);
		if (putDemographicResponse.err !== null) {
			notifications.addFlash({
				message: 'Could not update value, please try again',
				priority: 'ERROR'
			});
		}
	}

	function getOptions(question: QuestionLatestDto): Option[] {
		const options: Option[] = question.options.map((o) => ({
			label: o.label,
			value: o.id
		}));

		if (question.allow_other) options.push({ label: 'Other', value: 'other' });

		if (question.prefer_not_to_say)
			options.push({ label: 'Prefer not to say', value: 'prefer_not_to_say' });

		return options;
	}

	let sequence = $derived<ToolSequence>({
		next:
			questionNumber >= questions.length
				? onDone
				: () => {
						submit?.click();
						questionNumber++;
					},
		previous:
			questionNumber <= 1
				? undefined
				: () => {
						submit?.click();
						questionNumber--;
					},
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
				queries: { organization_id: organizationId },
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
			demographicForm.ok.entries.map((entry) =>
				tryCatchAsync(() =>
					apiClient.GetDemographicQuestion({
						queries: { organization_id: organizationId },
						params: { question_id: entry.question_id }
					})
				)
			) ?? []
		);

		questions = questionsResponses.map((r) => r.ok).filter((q) => q !== null);
	});
</script>

{#if questions[questionNumber - 1]?.latest_published}
	{@const question = questions[questionNumber - 1]}
	{#key question.id}
		<Form {handleSubmission}>
			{@const defaultValue = Formatter.to(
				question.questionType,
				cache.get(question.id) ??
					answers.find((a) => a.answer.question_version_id === question?.id)?.answer.value
			)}
			<input type="hidden" name="question_type" value={question.questionType} />
			{#if question.special_category}
				<Callout variant="primary">
					<input
						id="special-consent"
						name="consent"
						type="checkbox"
						required
						value={typed<ConsentState>('granted')}
					/>
					<label for="special-consent" class="pl-3">
						{question.consent_text ??
							'This question requires additional content as it contains sensitive information'}
					</label>
				</Callout>
			{/if}
			{#if question.questionType === 'text' || question.questionType === 'number' || question.questionType === 'date'}
				{#if question.multi === true}
					<Textarea
						label={question.text}
						name={question.id}
						defaultValue={String(defaultValue)}
					/>
				{:else}
					<Input
						label={question.text}
						name={question.id}
						type={question.questionType}
						{defaultValue}
					/>
				{/if}
			{:else if question.question_type === 'select'}
				<SelectList
					title={question.text}
					name={question.id}
					multiple={question.multi ?? false}
					options={getOptions(question)}
					defaultSelected={defaultValue as string[] | undefined}
				/>
			{:else if question.question_type === 'location'}
				<Input label={question.text} name={question.id} {defaultValue} />
			{/if}
			<Submit bind:ref={submit} class="hidden" text="Submit" />
		</Form>
	{/key}
{/if}
