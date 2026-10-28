import type { Tour } from '$lib/tours/types';

export const reportTour: Tour = {
	id: 'admin-conversation-report',
	steps: [
		{
			element: '[data-tour="section-header"]',
			title: 'Report',
			description:
				'Summarise what came out of the conversation, record its impact, and decide whether to publish it.',
			side: 'bottom'
		},
		{
			element: '[data-tour="report-publish"]',
			title: 'Publish when ready',
			description:
				'This bar always says what publishing shares. Preview shows the report as readers see it, and Review and publish lists exactly what becomes public.',
			side: 'bottom'
		}
	]
};
