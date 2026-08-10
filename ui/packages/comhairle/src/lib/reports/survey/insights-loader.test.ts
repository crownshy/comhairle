import { describe, expect, it } from 'vitest';
import type { SurveyInsights } from '@crownshy/api-client/api';
import { transformSurveyInsightsToInsights } from './insights-loader';

describe('transformSurveyInsightsToInsights', () => {
	it('maps choice-based questions to bar charts', () => {
		const response: SurveyInsights = {
			questions: [
				{
					id: 'q-1',
					title: 'Favourite colour',
					kind: 'multiple_choice',
					total: 2,
					choices: [
						{ id: 'blue', label: 'Blue', count: 2 },
						{ id: 'red', label: 'Red', count: 0 }
					]
				}
			]
		};

		const insights = transformSurveyInsightsToInsights(response);

		expect(insights).toHaveLength(1);
		expect(insights[0]?.chart.type).toBe('BarChart');
		expect(insights[0]?.chart.data).toEqual([
			{ label: 'Blue', value: 100 },
			{ label: 'Red', value: 0 }
		]);
	});

	it('normalizes choice counts so bars share the available width', () => {
		const response: SurveyInsights = {
			questions: [
				{
					id: 'q-2',
					title: 'Preferred pet',
					kind: 'multiple_choice',
					total: 4,
					choices: [
						{ id: 'cats', label: 'Cats', count: 3 },
						{ id: 'dogs', label: 'Dogs', count: 1 }
					]
				}
			]
		};

		const insights = transformSurveyInsightsToInsights(response);

		expect(insights[0]?.chart.type).toBe('BarChart');
		expect(insights[0]?.chart.data).toEqual([
			{ label: 'Cats', value: 75 },
			{ label: 'Dogs', value: 25 }
		]);
	});

	it('maps free-text submissions to text insights', () => {
		const response: SurveyInsights = {
			questions: [
				{
					id: 'q-3',
					title: 'What do you think?',
					kind: 'short_text',
					total: 2,
					submissions: [
						{ submission_id: 's1', value: 'Cats are great' },
						{ submission_id: 's2', value: 'Dogs are better' }
					]
				}
			]
		};

		const insights = transformSurveyInsightsToInsights(response);

		expect(insights[0]?.chart.type).toBe('Text');
		expect(insights[0]?.chart.data).toEqual(['Cats are great', 'Dogs are better']);
	});

	it('bins numeric submissions into categorical counts', () => {
		const response: SurveyInsights = {
			questions: [
				{
					id: 'q-3',
					title: 'Age range',
					kind: 'number',
					total: 3,
					submissions: [
						{ submission_id: 's1', value: 18 },
						{ submission_id: 's2', value: 24 },
						{ submission_id: 's3', value: 35 }
					]
				}
			]
		};

		const insights = transformSurveyInsightsToInsights(response);

		expect(insights[0]?.chart.type).toBe('BarChart');
		expect(insights[0]?.chart.data).toEqual([
			{ label: '18-24', value: 1 },
			{ label: '24-29', value: 1 },
			{ label: '29-35', value: 1 }
		]);
	});
});
