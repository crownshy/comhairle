import { tryCatchAsync } from '$lib/utils/errorHandling';
import { isHeyFormFieldKind } from '$lib/tools/heyform/guards';
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

export interface ChoiceQuestion {
	id: string;
	title: string;
	total: number;
	properties?: Properties | null;
	kind: HeyFormChoiceFieldKind;
	answers: Choice[];
}

export interface NonChoiceQuestion {
	id: string;
	title: string;
	total: number;
	properties?: Properties;
	kind: HeyFormNonChoiceFieldKind;
	answers: unknown[];
}

export type SurveyQuestion = ChoiceQuestion | NonChoiceQuestion;

function transform(insight: InsightQuestion): SurveyQuestion | undefined {
	if (!insight.kind || !isHeyFormFieldKind(insight.kind)) {
		return undefined;
	}

	switch (insight.kind) {
		case 'yes_no':
		case 'multiple_choice':
		case 'picture_choice':
			return typedObj<ChoiceQuestion>({
				id: insight.id,
				title: insight.title,
				total: insight.total,
				kind: insight.kind,
				properties: insight.properties,
				answers: insight.choices ?? []
			});
		case 'ranking': {
			const answers: Choice[] = [];

			if (!insight.properties || !insight.submissions) {
				return undefined;
			}

			for (const submission of insight.submissions) {
				const s = submission.value as HeyFormRankedValue;
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

			return typedObj<ChoiceQuestion>({
				id: insight.id,
				title: insight.title,
				total: insight.total,
				kind: insight.kind,
				properties: insight.properties,
				answers
			});
		}
		case 'matrix': {
			const answers: Choice[] = [];

			if (!insight.properties || !insight.submissions) {
				return undefined;
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

			return typedObj<ChoiceQuestion>({
				id: insight.id,
				title: insight.title,
				total: insight.total,
				kind: insight.kind,
				properties: insight.properties,
				answers
			});
		}
		case 'legal_terms': {
			if (!insight.submissions) {
				return undefined;
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
				answers[Number(value)].count += 1;
			}

			return typedObj<ChoiceQuestion>({
				id: insight.id,
				title: insight.title,
				total: insight.total,
				kind: insight.kind,
				properties: insight.properties ?? undefined,
				answers
			});
		}
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
			return typedObj<NonChoiceQuestion>({
				id: insight.id,
				title: insight.title,
				total: insight.total,
				kind: insight.kind,
				properties: insight.properties ?? undefined,
				answers:
					insight.submissions
						?.map((s) => s.value)
						.filter((s) => (typeof s === 'string' ? !!s.trim() : !!s)) ?? []
			});
		case 'full_name':
			return typedObj<NonChoiceQuestion>({
				id: insight.id,
				title: insight.title,
				total: insight.total,
				kind: insight.kind,
				properties: insight.properties ?? undefined,
				answers:
					insight.submissions
						?.map((s) => {
							const fullName = s.value as HeyFormFullNameValue;
							return `${fullName.firstName} ${fullName.lastName}`;
						})
						.filter((s) => !!s.trim()) ?? []
			});
		case 'address':
			return typedObj<NonChoiceQuestion>({
				id: insight.id,
				title: insight.title,
				total: insight.total,
				kind: insight.kind,
				properties: insight.properties ?? undefined,
				answers:
					insight.submissions
						?.map((s) => {
							const address = s.value as HeyFormAddressValue;
							return Object.values(address).join(', ');
						})
						.filter((s) => !!s.trim()) ?? []
			});
		case 'date_range':
			return typedObj<NonChoiceQuestion>({
				id: insight.id,
				title: insight.title,
				total: insight.total,
				kind: insight.kind,
				properties: insight.properties ?? undefined,
				answers:
					insight.submissions
						?.map((s) => {
							const dateRange = s.value as HeyFormDateRangeValue;
							return `${dateRange.start} - ${dateRange.end}`;
						})
						.filter((s) => !!s.trim()) ?? []
			});
		case 'file_upload':
			return typedObj<NonChoiceQuestion>({
				id: insight.id,
				title: insight.title,
				total: insight.total,
				kind: insight.kind,
				properties: insight.properties ?? undefined,
				answers:
					insight.submissions
						?.map((s) => {
							const file = s.value as HeyFormFileUploadValue;
							return `${file.filename} - ${file.url}`;
						})
						.filter((s) => !!s.trim()) ?? []
			});
		case 'group':
		case 'statement':
		case 'time':
		case 'input_table':
		case 'payment':
		case 'signature':
		case 'submit_date':
		case 'custom_text':
		case 'custom_single':
		case 'custom_multiple':
		case 'custom_date':
		case 'custom_number':
		case 'custom_checkbox':
		case 'hidden_fields':
		case 'variable':
		case 'hidden_checkbox':
		case 'welcome':
		case 'thank_you':
		default:
			return undefined;
	}
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

	return { survey: response.ok.questions.map(transform).filter((q) => q !== undefined) };
}
