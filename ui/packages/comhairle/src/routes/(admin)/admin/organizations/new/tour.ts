import type { Tour } from '$lib/tours/types';

export const newOrganizationTour: Tour = {
	id: 'admin-organization-new',
	steps: [
		{
			element: '[data-tour="page-header"]',
			title: 'Create an organisation',
			description:
				'Set its name, type and regions. You can invite team members and adjust details afterwards.',
			side: 'bottom'
		}
	]
};
