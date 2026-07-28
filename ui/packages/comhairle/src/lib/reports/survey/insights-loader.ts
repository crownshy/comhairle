import { typedObj } from '$lib/utils/types';
import { apiClient } from '@crownshy/api-client/client';

export type ChartData = {
	label: string;
	value: number;
};
type BarplotVariant = 'label' | 'value';
export type Barplot = {
	type: 'Barplot';
	variant: BarplotVariant;
	data: ChartData[];
};

export type Doughnut = {
	type: 'Doughnut';
	data: ChartData[];
};

export type Line = {
	type: 'Line';
	data: ChartData[];
};

type Title = string;
export type Insight = {
	title: Title;
	chart: Barplot | Doughnut | Line;
};

export async function surveyInsightsLoader(workflowStepId: string) {
	// TODO: Undo when the backend data is ready
	// const insights = await apiClient.GetPrioritizationInsights({
	// 	queries: { workflow_step_id: workflowStepId }
	// });

	const insights: Insight[] = [
		{
			title: 'Engagement',
			chart: typedObj<Barplot>({
				type: 'Barplot',
				variant: 'value',
				data: [
					{
						label: 'Online petition platform',
						value: 39
					},
					{
						label: 'Pol.is discussion',
						value: 29
					},
					{
						label: 'In-person hearing',
						value: 24
					},
					{
						label: 'Deliberation workshop',
						value: 14
					},
					{
						label: 'Other',
						value: 5
					}
				]
			})
		}
	] as const;

	return { survey: { insights } };
}
