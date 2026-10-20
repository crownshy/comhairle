import type { Tour } from '$lib/tours/types';

export const stepModerationTour: Tour = {
	id: 'admin-step-moderation',
	steps: [
		{
			element: '[data-tour="moderation-actions"]',
			title: 'Sync, seed and export',
			description:
				'Sync from Polis pulls in new participant statements; the list does not update on its own. Add seed statements to start the conversation.',
			side: 'bottom'
		},
		{
			element: '[data-tour="moderation-filters"]',
			title: 'Filter by status',
			description: 'Switch between pending, accepted, rejected, seeded and all statements.',
			side: 'bottom'
		},
		{
			element: '[data-tour="moderation-statements"]',
			title: 'Accept, reject or split',
			description:
				'Accept a statement to publish it as written, reject it with a reason, or split it if it makes more than one point.',
			side: 'top'
		}
	]
};
