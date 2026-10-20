import type { Tour } from '$lib/tours/types';

export const eventsListTour: Tour = {
	id: 'admin-conversation-events',
	steps: [
		{
			element: '[data-tour="page-header"]',
			title: 'Events',
			description:
				'Schedule and run live sessions, such as workshops, as part of this conversation.',
			side: 'bottom'
		},
		{
			element: '[data-tour="events-list"]',
			title: 'Your events',
			description: 'Add an event to set its date, time, facilitators and signup mode.',
			side: 'top'
		}
	]
};
