import type { Tour } from '$lib/tours/types';

export const stepTour: Tour = {
	id: 'admin-conversation-step',
	steps: [
		{
			element: '[data-tour="step-subtabs"]',
			title: 'Step tabs',
			description:
				'Setup holds the content participants see. Configure sets the step name and navigation rules. Insights only applies to tools that collect contributions.',
			side: 'bottom'
		}
	]
};
