import type { Tour } from '$lib/tours/types';

export const monitorTour: Tour = {
	id: 'admin-conversation-monitor',
	steps: [
		{
			element: '[data-tour="page-header"]',
			title: 'Monitor',
			description:
				'See how many people are taking part, how far they get, and how long it takes.',
			side: 'bottom'
		}
	]
};
