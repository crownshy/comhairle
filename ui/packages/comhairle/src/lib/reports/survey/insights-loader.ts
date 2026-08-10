import { tryCatchAsync } from '$lib/utils/errorHandling';
import { typedObj } from '$lib/utils/types';
import { apiClient } from '@crownshy/api-client/client';
import type { SurveyInsights as SurveyInsightsResponse } from '@crownshy/api-client/api';

export type ChartData = {
	label: string;
	value: number;
};
type BarplotVariant = 'label' | 'value';
export type Barplot = {
	type: 'BarChart';
	variant: BarplotVariant;
	data: ChartData[];
};

export type Doughnut = {
	type: 'Doughnut';
	data: ChartData[];
};

export type Line = {
	type: 'Line';
	data: Record<string, number[]>;
};

type Text = {
	type: 'Text';
	data: string[];
};

type Title = string;
export type Insight = {
	title: Title;
	chart: Barplot | Doughnut | Line | Text;
};

function isChoiceQuestion(question: SurveyInsightsResponse['questions'][number]) {
	return Boolean(question.choices && question.choices.length > 0);
}

function isNumericQuestion(question: SurveyInsightsResponse['questions'][number]) {
	return Boolean(
		question.submissions &&
		question.submissions.length > 0 &&
		question.submissions.every((submission) => isNumericValue(submission.value))
	);
}

function isOpinionScaleQuestion(question: SurveyInsightsResponse['questions'][number]) {
	return question.kind === 'opinion_scale';
}

function isNumericValue(value: unknown): value is number {
	return typeof value === 'number' && Number.isFinite(value);
}

function buildNumericBins(question: SurveyInsightsResponse['questions'][number]) {
	const values = (question.submissions ?? [])
		.map((submission) => (isNumericValue(submission.value) ? submission.value : null))
		.filter((value): value is number => value !== null);

	if (values.length === 0) {
		return null;
	}

	const sortedValues = [...values].sort((a, b) => a - b);
	const minValue = sortedValues[0] ?? 0;
	const maxValue = sortedValues[sortedValues.length - 1] ?? minValue;
	const bucketCount = Math.max(1, Math.min(8, sortedValues.length));
	const binSize = maxValue === minValue ? 1 : (maxValue - minValue) / bucketCount;
	const bins = new Map<string, number>();

	for (const value of sortedValues) {
		const bucketIndex =
			maxValue === minValue
				? 0
				: Math.min(bucketCount - 1, Math.floor((value - minValue) / binSize));
		const bucketLabel = `${Math.round(minValue + bucketIndex * binSize)}-${Math.round(minValue + (bucketIndex + 1) * binSize)}`;
		bins.set(bucketLabel, (bins.get(bucketLabel) ?? 0) + 1);
	}

	return Array.from(bins.entries()).map(([label, count]) => ({ label, value: count }));
}

function asTextArray(
	submissions: SurveyInsightsResponse['questions'][number]['submissions']
): string[] {
	return (submissions ?? [])
		.map((submission) => {
			if (typeof submission.value === 'string') {
				return submission.value;
			}
			if (submission.value && typeof submission.value === 'object') {
				const value = submission.value as Record<string, unknown>;
				if (typeof value.text === 'string') {
					return value.text;
				}
				if (typeof value.value === 'string') {
					return value.value;
				}
			}
			return '';
		})
		.filter((value) => value.length > 0);
}

export function transformSurveyInsightsToInsights(response: SurveyInsightsResponse): Insight[] {
	return (response.questions ?? []).map((question) => {
		if (isChoiceQuestion(question)) {
			const totalCount = (question.choices ?? []).reduce(
				(sum, choice) => sum + choice.count,
				0
			);
			const chartData = (question.choices ?? []).map((choice) => ({
				label: choice.label,
				value: totalCount > 0 ? (choice.count / totalCount) * 100 : 0
			}));

			return {
				title: question.title,
				chart: typedObj<Barplot>({
					type: 'BarChart',
					variant: 'value',
					data: chartData
				})
			};
		}

		if (isOpinionScaleQuestion(question) && isNumericQuestion(question)) {
			const values = (question.submissions ?? []).map(
				(submission) => submission.value as number
			);

			if (values.length > 0) {
				return {
					title: question.title,
					chart: typedObj<Line>({
						type: 'Line',
						data: { [question.title]: values }
					})
				};
			}
		}

		const numericBins = isNumericQuestion(question) ? buildNumericBins(question) : null;
		if (numericBins) {
			return {
				title: question.title,
				chart: typedObj<Barplot>({
					type: 'BarChart',
					variant: 'value',
					data: numericBins
				})
			};
		}

		const textValues = asTextArray(question.submissions);
		if (textValues.length > 0) {
			return {
				title: question.title,
				chart: typedObj<Text>({
					type: 'Text',
					data: textValues
				})
			};
		}

		return {
			title: question.title,
			chart: typedObj<Text>({
				type: 'Text',
				data: []
			})
		};
	});
}

export async function surveyInsightsLoader(workflowStepId: string) {
	const response = await tryCatchAsync(() =>
		apiClient.HeyFormGetInsights({
			params: { workflow_step_id: workflowStepId }
		})
	);

	if (response.err !== null) {
		return { survey: { insights: [] as Insight[] } };
	}

	const insights = transformSurveyInsightsToInsights(response.ok);

	return { survey: { insights } };
}
