import type { Tour } from '$lib/tours/types';

export const organizationTour: Tour = {
	id: 'admin-organization',
	steps: [
		{
			element: '[data-tour="organization-tabs"]',
			title: 'Details and Team',
			description: "Edit the organisation's details, or manage who has access under Team.",
			side: 'bottom'
		}
	]
};
