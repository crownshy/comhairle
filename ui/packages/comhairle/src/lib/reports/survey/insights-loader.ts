import { typedObj } from '$lib/utils/types';
import { apiClient } from '@crownshy/api-client/client';

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
	data: ChartData[];
};

type Title = string;
export type Insight = {
	title: Title;
	chart: Barplot | Doughnut | Line;
};

export async function surveyInsightsLoader(workflowStepId: string) {
	// TODO: Undo when the backend data is ready
	// const r1 = await apiClient.HeyFormGetFormReport({
	// 	params: { workflow_step_id: workflowStepId }
	// });
	// const r2 = await apiClient.HeyFormGetSubmissions({
	// 	params: { workflow_step_id: workflowStepId }
	// });
	//
	// console.log("r1:", r1);
	// console.log("r2:", r2);

	const insights: Insight[] = [
		{
			title: 'Engagement',
			chart: typedObj<Barplot>({
				type: 'BarChart',
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
		},
		{
			title: 'Age',
			chart: typedObj<Barplot>({
				type: 'BarChart',
				variant: 'label',
				data: [
					{
						label: '18-20',
						value: 52
					},
					{
						label: '21-30',
						value: 90
					},
					{
						label: '31-40',
						value: 70
					},
					{
						label: '41-50',
						value: 20
					},
					{
						label: '51-60',
						value: 60
					},
					{
						label: '61+',
						value: 63
					}
				]
			})
		}
	] as const;

	return { survey: { insights } };
}
