import type { Tour } from '$lib/tours/types';

export const newEventTour: Tour = {
	id: 'admin-conversation-event-new',
	steps: [
		{
			element: '[data-tour="page-header"]',
			title: 'New event',
			description:
				'Set the date, time zone and signup mode. You can add facilitators here, and more detail (agenda, location, invites) once it is created.',
			side: 'bottom'
		}
	]
};
