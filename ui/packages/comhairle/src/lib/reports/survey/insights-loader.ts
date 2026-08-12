import { tryCatchAsync } from '$lib/utils/errorHandling';
import { apiClient } from '@crownshy/api-client/client';
import { isHeyFormFieldKind } from '$lib/tools/heyform/guards';
import type {
	HeyFormAddressValue,
	HeyFormChoiceFieldKind,
	HeyFormDateRangeValue,
	HeyFormFullNameValue,
	HeyFormNonChoiceFieldKind
} from '$lib/tools/heyform/utils';
import type { InsightQuestion } from '@crownshy/api-client/api';
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
	kind: HeyFormChoiceFieldKind;
	properties: Record<string, string>;
	answers: Choice[];
}

export interface NonChoiceQuestion {
	id: string;
	title: string;
	total: number;
	kind: HeyFormNonChoiceFieldKind;
	properties: Record<string, string>;
	answers: unknown[];
}

export type SurveyQuestion = ChoiceQuestion | NonChoiceQuestion;

function transform(insight: InsightQuestion): SurveyQuestion | undefined {
	if (!insight.kind || !isHeyFormFieldKind(insight.kind)) {
		return undefined;
	}

	switch (insight.kind) {
		case 'rating':
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
				properties: insight.properties,
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
				properties: insight.properties,
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
				properties: insight.properties,
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
				properties: insight.properties,
				answers:
					insight.submissions
						?.map((s) => {
							const dateRange = s.value as HeyFormDateRangeValue;
							return `${dateRange.start} - ${dateRange.end}`;
						})
						.filter((s) => !!s.trim()) ?? []
			});
		case 'group':
		case 'statement':
		case 'file_upload':
		case 'time':
		case 'input_table':
		case 'payment':
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
