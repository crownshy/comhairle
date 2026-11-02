import type { Tour } from '$lib/tours/types';

export const eventTour: Tour = {
	id: 'admin-conversation-event',
	steps: [
		{
			element: '[data-tour="page-header"]',
			title: 'Manage this event',
			description:
				'Use the tabs above to edit details, build the facilitator agenda, set up the location, and manage facilitators, invites, breakout rooms and recordings.',
			side: 'bottom'
		}
	]
};
