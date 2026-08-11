import { tryCatchAsync } from '$lib/utils/errorHandling';
import { apiClient } from '@crownshy/api-client/client';
import { isHeyFormFieldKind } from '$lib/tools/heyform/utils';
import type { InsightQuestion } from '@crownshy/api-client/api';

interface ChoiceQuestion extends Omit<InsightQuestion, 'choices'> {
	choices: {
		id: string;
		label: string;
		count: number;
	}[];
}

interface NonChoiceQuestion extends Omit<InsightQuestion, 'choices'> {
	answers: number[] | string[];
}

function transformer(insights: InsightQuestion): ChoiceQuestion | NonChoiceQuestion | undefined {
	if (!isHeyFormFieldKind(insights.kind)) {
		return undefined;
	}

	switch (insights.kind) {
		case 'number':
		case 'date':
		case 'group':
		case 'statement':
		case 'short_text':
		case 'long_text':
		case 'yes_no':
		case 'multiple_choice':
		case 'picture_choice':
		case 'file_upload':
		case 'opinion_scale':
		case 'rating':
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

export async function surveyInsightsLoader(workflowStepId: string) {
	const response = await tryCatchAsync(() =>
		apiClient.HeyFormGetInsights({
			params: { workflow_step_id: workflowStepId }
		})
	);

	if (response.err !== null) {
		return { survey: { insights: [] as InsightQuestion[] } };
	}

	return { survey: response.ok.questions };
}
