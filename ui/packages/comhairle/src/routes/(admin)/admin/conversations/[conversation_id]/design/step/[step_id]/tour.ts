import type { Tour } from '$lib/tours/types';

export const stepTour: Tour = {
	id: 'admin-conversation-step',
	steps: [
		{
			element: '[data-tour="step-subtabs"]',
			title: 'A step has four tabs',
			description:
				'Setup holds the content participants see. Configure sets the step name and navigation rules. Moderation and Insights only apply to tools that collect contributions.',
			side: 'bottom'
		}
	]
};
