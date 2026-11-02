import type { Tour } from '$lib/tours/types';

export const moderateTour: Tour = {
	id: 'admin-conversation-moderate',
	steps: [
		{
			element: '[data-tour="moderate-steps"]',
			title: 'Moderate contributions',
			description:
				'Each Polis step gets a row here. Open one to accept, reject or split its statements.',
			side: 'top'
		}
	]
};
