import type { Tour } from '$lib/tours/types';

export const reportTour: Tour = {
	id: 'admin-conversation-report',
	steps: [
		{
			element: '[data-tour="page-header"]',
			title: 'Report',
			description:
				'Summarise what came out of the conversation, record its impact, and decide whether to publish it.',
			side: 'bottom'
		},
		{
			element: '[data-tour="report-publish"]',
			title: 'Publish when ready',
			description: 'View shows the report as others will see it. Publish makes it public.',
			side: 'bottom'
		}
	]
};
