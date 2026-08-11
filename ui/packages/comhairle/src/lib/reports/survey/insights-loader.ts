import { tryCatchAsync } from '$lib/utils/errorHandling';
import { apiClient } from '@crownshy/api-client/client';
import { isHeyFormFieldKind } from '$lib/tools/heyform/utils';
import type { InsightQuestion } from '@crownshy/api-client/api';
import type { HeyFormFieldKind } from '$lib/tools/heyform/types';
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
	kind: Extract<
		HeyFormFieldKind,
		'opinion_scale' | 'rating' | 'multiple_choice' | 'picture_choice' | 'yes_no'
	>;
	answers: Choice[];
}

export interface NonChoiceQuestion {
	id: string;
	title: string;
	total: number;
	kind: Extract<HeyFormFieldKind, 'number' | 'short_text' | 'long_text'>;
	answers: unknown[];
}

export type SurveyQuestion = ChoiceQuestion | NonChoiceQuestion;

function transform(insight: InsightQuestion): SurveyQuestion | undefined {
	if (!insight.kind || !isHeyFormFieldKind(insight.kind)) {
		return undefined;
	}

	switch (insight.kind) {
		case 'opinion_scale': {
			const answers: Choice[] = [];

			for (const submission of insight.submissions ?? []) {
				const index = answers.findIndex((a) => a.label === String(submission.value));
				if (index > -1) {
					answers[index].count += 1;
					continue;
				}
				answers.push({
					id: submission.submission_id,
					label: String(submission.value),
					count: 1
				});
			}

			return typedObj<ChoiceQuestion>({
				id: insight.id,
				title: insight.title,
				total: insight.total,
				kind: insight.kind,
				answers
			});
		}
		case 'rating':
		case 'yes_no':
		case 'multiple_choice':
		case 'picture_choice':
			return typedObj<ChoiceQuestion>({
				id: insight.id,
				title: insight.title,
				total: insight.total,
				kind: insight.kind,
				answers: insight.choices ?? []
			});
		case 'number':
		case 'short_text':
		case 'long_text':
			return typedObj<NonChoiceQuestion>({
				id: insight.id,
				title: insight.title,
				total: insight.total,
				kind: insight.kind,
				answers: insight.submissions?.map((s) => s.value) ?? []
			});
		case 'date':
		case 'group':
		case 'statement':
		case 'file_upload':
		case 'date_range':
		case 'time':
		case 'input_table':
		case 'payment':
		case 'full_name':
		case 'address':
		case 'email':
		case 'url':
		case 'phone_number':
		case 'country_selector':
		case 'signature':
		case 'legal_terms':
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
			return undefined;
	}
}

export async function surveyInsightsLoader(
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
