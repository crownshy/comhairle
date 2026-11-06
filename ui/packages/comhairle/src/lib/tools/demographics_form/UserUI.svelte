<script lang="ts">
	import { notifications } from '$lib/notifications.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import type {
		ConversationDto,
		MyDemographicAnswer,
		QuestionDto,
		QuestionType2
	} from '@crownshy/api-client/api';
	import { apiClient } from '@crownshy/api-client/client';
	import { onMount } from 'svelte';
	import type { OnSequenceChange, ToolSequence } from '../toolSequence';
	import Callout from '$lib/components/Callout.svelte';
	import { Form, Input, SelectList, Submit, type Option } from '$lib/components/EasyForm';
	import Formatter from './Formatter';
	import Demographics from '$lib/interfaces/Demographics';

	type Props = {
		organizationId: ConversationDto['organizationId'];
		formId: string;
		onDone: () => void;
		onSequenceChange: OnSequenceChange;
	};

	let { organizationId, formId, onDone, onSequenceChange }: Props = $props();

	let questions = $state.raw<QuestionDto[]>([]);
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

		const type = formData.get('question_type') as QuestionType2;
		const consent = formData.get('consent') as 'accepted' | null;
		const value = formData.getAll(question.id ?? '') as string[] | null;
		if (!value || (value.length === 1 && value[0] === '')) {
			return;
		}

		const processedValue = Formatter.from(type, value);

		cache.set(question.id, processedValue);

		if (value.length > 0 && question.latest_published?.special_category && consent === null) {
			// TODO: Add in error saying need to accept special consent
			return;
		}

		const version = Demographics.getVersion(question);
		if (!version) {
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

	function getInputType(
		questionType: Omit<QuestionType2, 'single_select' | 'multi_select' | 'location'>
	): HTMLInputElement['type'] {
		switch (questionType) {
			case 'number':
				return 'number';
			case 'date':
				return 'date';
			case 'any_string':
			case 'custom':
			default:
				return 'text';
		}
	}

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

{#if questions[questionNumber - 1]?.latest_published}
	{@const question = questions[questionNumber - 1]}
	{@const q = question.latest_published!}
	{#key question.id}
		<Form {handleSubmission}>
			{@const defaultValue = Formatter.to(
				q.question_type,
				cache.get(question.id) ??
					answers.find(
						(a) =>
							a.answer.question_version_id === Demographics.getVersion(question)?.id
					)?.answer.value
			)}
			<input type="hidden" name="question_type" value={q.question_type} />
			{#if q.special_category}
				<Callout>
					<input
						id="special-consent"
						name="consent"
						type="checkbox"
						required
						value="accepted"
					/>
					<label for="special-consent" class="pl-3">
						{q.consent_text ??
							'This question requires additional content as it contains sensitive information'}
					</label>
				</Callout>
			{/if}
			{#if q.question_type === 'single_select' || q.question_type === 'multi_select'}
				<SelectList
					title={q.text}
					name={question.id}
					multiple={q.question_type === 'multi_select'}
					options={getOptions(q)}
					defaultSelected={defaultValue as string[] | undefined}
				/>
			{:else if q.question_type === 'location'}
				<Input label={q.text} name={question.id} {defaultValue} />
			{:else}
				<Input
					label={q.text}
					name={question.id}
					type={getInputType(q.question_type)}
					{defaultValue}
				/>
			{/if}
			<Submit bind:ref={submit} class="hidden" text="Submit" />
		</Form>
	{/key}
{/if}
