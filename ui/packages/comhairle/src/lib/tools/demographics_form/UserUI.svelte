<script lang="ts">
	import { notifications } from '$lib/notifications.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import type {
		ConversationDto,
		MyDemographicAnswer,
		QuestionDto,
		QuestionType
	} from '@crownshy/api-client/api';
	import { apiClient } from '@crownshy/api-client/client';
	import { onMount } from 'svelte';
	import type { OnSequenceChange, ToolSequence } from '../toolSequence';
	import Callout from '$lib/components/Callout.svelte';
	import { Form, Input, SelectList, Submit, type Option } from '$lib/components/EasyForm';
	import Formatter from './Formatter';
	import Demographics from '$lib/interfaces/Demographics';
	import ErrorMessage from '$lib/components/ErrorMessage.svelte';

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
	let specialConsentErrorMessage = $state<string>('');

	// eslint-disable-next-line svelte/prefer-svelte-reactivity
	let cache = new Map();

	let submit = $state<HTMLButtonElement | null>(null);
	let forwards: boolean = true;

	async function handleSubmission(formData: FormData) {
		const question = questions[questionNumber - 1];
		if (!question) {
			return;
		}

		const type = formData.get('question_type') as QuestionType;
		const consent = formData.get('consent') as 'accepted' | null;
		const value = formData.getAll(question.id ?? '') as string[] | null;
		if (!value || (value.length === 1 && value[0] === '')) {
			if (!question.required) {
				questionNumber += forwards ? 1 : -1;
			}
			return;
		}

		const processedValue = Formatter.from(type, value);

		cache.set(question.id, processedValue);

		if (value.length > 0 && question.latest_published?.special_category) {
			if (consent === null) {
				specialConsentErrorMessage = 'Please accept the special consent before continuing';
				return;
			} else {
				const version = Demographics.getVersion(question);
				if (!version) {
					notifications.addFlash({
						message: 'Could not find version number, please try again',
						priority: 'ERROR'
					});
					return;
				}
				const setConsent = await tryCatchAsync(() =>
					apiClient.SetDemographicConsent(
						{ granted: true },
						{ params: { version_id: version.id } }
					)
				);
				if (setConsent.err !== null) {
					notifications.addFlash({
						message: 'Could not update consent, please try again',
						priority: 'ERROR'
					});
					return;
				}
				specialConsentErrorMessage = '';
			}
		}

		if (value.length > 0) {
			const version = Demographics.getVersion(question);
			if (!version) {
				notifications.addFlash({
					message: 'Could not find version number, please try again',
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
				return;
			}
		}

		questionNumber += forwards ? 1 : -1;
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
						forwards = true;
						submit?.click();
					},
		previous:
			questionNumber <= 1
				? undefined
				: () => {
						forwards = false;
						submit?.click();
					},
		progress: questionNumber / questions.length,
		position: `Question ${questionNumber} of ${questions.length}`
	});

	function getInputType(
		questionType: Omit<QuestionType, 'single_select' | 'multi_select'>
	): HTMLInputElement['type'] {
		switch (questionType) {
			case 'number':
				return 'number';
			case 'date':
				return 'date';
			case 'text':
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
			demographicForm.ok.latest_published?.entries.map((entry) =>
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
				<div class="mb-3 flex flex-col gap-y-1">
					<Callout variant={specialConsentErrorMessage ? 'danger' : 'primary'}>
						<input
							id="special-consent"
							name="consent"
							type="checkbox"
							value="accepted"
							onchange={(e) => {
								if (e.currentTarget.checked) {
									specialConsentErrorMessage = '';
								}
							}}
							class="cursor-pointer"
						/>
						<label for="special-consent" class="cursor-pointer pl-3 text-sm">
							{q.consent_text ??
								'This question requires additional content as it contains sensitive information'}
						</label>
					</Callout>
					{#if specialConsentErrorMessage}
						<ErrorMessage size="sm">{specialConsentErrorMessage}</ErrorMessage>
					{/if}
				</div>
			{/if}
			{#if q.question_type === 'single_select' || q.question_type === 'multi_select'}
				<SelectList
					title={q.text}
					name={question.id}
					multiple={q.question_type === 'multi_select'}
					options={getOptions(q)}
					defaultSelected={defaultValue as string[] | undefined}
				/>
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
