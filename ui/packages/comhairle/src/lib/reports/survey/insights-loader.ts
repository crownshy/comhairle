import { tryCatchAsync } from '$lib/utils/errorHandling';
import {
	isHeyFormChoiceFieldKind,
	isHeyFormFieldKind,
	isHeyFormNonChoiceFieldKind,
	isHeyFormOtherFieldKind
} from '$lib/tools/heyform/guards';
import type {
	HeyFormAddressValue,
	HeyFormChoiceFieldKind,
	HeyFormDateRangeValue,
	HeyFormFileUploadValue,
	HeyFormFullNameValue,
	HeyFormLegalTermsValue,
	HeyFormMatrixValue,
	HeyFormNonChoiceFieldKind,
	HeyFormRankedValue,
	Properties
} from '$lib/tools/heyform/utils';
import type { ApiClient, InsightQuestion } from '@crownshy/api-client/api';
import { typedObj } from '$lib/utils/types';

type Choice = {
	id: string;
	label: string;
	count: number;
};

interface Question {
	id: string;
	title: string;
	answered: number;
	total: number;
}

export interface ChoiceQuestion extends Question {
	properties?: Properties | null;
	kind: HeyFormChoiceFieldKind;
	answers: Choice[];
}

export interface NonChoiceQuestion extends Question {
	properties?: Properties;
	kind: HeyFormNonChoiceFieldKind;
	answers: unknown[];
}

export type SurveyQuestion = ChoiceQuestion | NonChoiceQuestion;

function transformChoiceData(insight: InsightQuestion): ChoiceQuestion['answers'] {
	switch (insight.kind) {
		case 'yes_no':
		case 'multiple_choice':
		case 'picture_choice':
			return insight.choices ?? [];

		case 'ranking': {
			const answers: Choice[] = [];

			if (!insight.properties || !insight.submissions) {
				return [];
			}

			for (const submission of insight.submissions) {
				const s = submission.value as HeyFormRankedValue;
				if (s.value.length === 0) {
					continue;
				}

				for (let i = 0; i < s.value.length; i++) {
					const choiceId = s.value[i];
					// Calculate the amount of ranking, so just the length minus the index, e.g. for 5 choices, the top would receieve 5, last would receieve 1
					const amount = s.value.length - i;
					const answerIndex = answers.findIndex((a) => a.id === choiceId);

					if (answerIndex > -1) {
						answers[answerIndex].count += amount;
						continue;
					}

					const label =
						(insight.properties?.choices as Omit<Choice, 'count'>[]).find(
							(c) => c.id === choiceId
						)?.label ?? '';

					answers.push({
						id: choiceId,
						label,
						count: amount
					});
				}
			}

			return answers;
		}

		case 'matrix': {
			const answers: Choice[] = [];

			if (!insight.properties || !insight.submissions) {
				return [];
			}

			for (const submission of insight.submissions) {
				const s = submission.value as HeyFormMatrixValue;
				for (const [choiceId, amount] of Object.entries(s)) {
					const answerIndex = answers.findIndex((a) => a.id === choiceId);

					if (answerIndex > -1) {
						answers[answerIndex].count += amount;
						continue;
					}

					const label =
						(insight.properties?.choices as Omit<Choice, 'count'>[]).find(
							(c) => c.id === choiceId
						)?.label ?? '';

					answers.push({
						id: choiceId,
						label,
						count: amount
					});
				}
			}

			return answers;
		}

		case 'legal_terms': {
			if (!insight.submissions) {
				return [];
			}

			const answers: Choice[] = [
				{
					id: 'yes',
					label: 'Yes',
					count: 0
				},
				{
					id: 'no',
					label: 'No',
					count: 0
				}
			];

			for (const submission of insight.submissions) {
				const value = submission.value as HeyFormLegalTermsValue;
				// value `true` = 1
				// (1 + 1) % 2 = 0
				// index 0 of answers = "yes"
				answers[(Number(value) + 1) % 2].count += 1;
			}

			return answers;
		}

		default:
			return [];
	}
}

function transformNonChoiceData(insight: InsightQuestion): NonChoiceQuestion['answers'] {
	switch (insight.kind) {
		case 'rating':
		case 'opinion_scale':
		case 'number':
		case 'short_text':
		case 'long_text':
		case 'email':
		case 'url':
		case 'phone_number':
		case 'country_selector':
		case 'date':
			return (
				insight.submissions
					?.map((s) => s.value)
					.filter((s) => (typeof s === 'string' ? !!s.trim() : true)) ?? []
			);

		case 'full_name':
			return (
				insight.submissions
					?.map((s) => {
						const fullName = s.value as HeyFormFullNameValue;
						if (fullName === '') return '';
						return `${fullName.firstName} ${fullName.lastName}`;
					})
					.filter((s) => !!s.trim()) ?? []
			);

		case 'address':
			return (
				insight.submissions
					?.map((s) => {
						const address = s.value as HeyFormAddressValue;
						if (address === '') return '';
						return Object.values(address).join(', ');
					})
					.filter((s) => !!s.trim()) ?? []
			);

		case 'date_range':
			return (
				insight.submissions
					?.map((s) => {
						const dateRange = s.value as HeyFormDateRangeValue;
						if (dateRange === '') return '';
						return `${dateRange.start} - ${dateRange.end}`;
					})
					.filter((s) => !!s.trim()) ?? []
			);

		case 'file_upload':
			return (
				insight.submissions
					?.map((s) => {
						const file = s.value as HeyFormFileUploadValue;
						if (file === '') return '';
						return `${file.filename} - ${file.url}`;
					})
					.filter((s) => !!s.trim()) ?? []
			);

		default:
			return [];
	}
}

function normalise(insight: InsightQuestion): SurveyQuestion | undefined {
	if (
		!insight.kind ||
		!isHeyFormFieldKind(insight.kind) ||
		isHeyFormOtherFieldKind(insight.kind)
	) {
		return undefined;
	}

	const question: Question = {
		id: insight.id,
		title: insight.title,
		answered: insight.answered,
		total: insight.total
	};

	if (isHeyFormChoiceFieldKind(insight.kind)) {
		return typedObj<ChoiceQuestion>({
			...question,
			properties: insight.properties ?? undefined,
			kind: insight.kind,
			answers: transformChoiceData(insight)
		});
	}

	if (isHeyFormNonChoiceFieldKind(insight.kind)) {
		return typedObj<NonChoiceQuestion>({
			...question,
			properties: insight.properties ?? undefined,
			kind: insight.kind,
			answers: transformNonChoiceData(insight)
		});
	}

	return undefined;
}

export async function surveyInsightsLoader(
	apiClient: ApiClient,
	workflowStepId: string
): Promise<{ survey: SurveyQuestion[] }> {
	const response = await tryCatchAsync(() =>
		apiClient.HeyFormGetInsights({
			params: { workflow_step_id: workflowStepId }
		})
	);

	if (response.err !== null) {
		return { survey: [] as SurveyQuestion[] };
	}

	return { survey: response.ok.questions.map(normalise).filter((q) => q !== undefined) };
}
