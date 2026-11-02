import type { Tour } from '$lib/tours/types';

export const organizationsTour: Tour = {
	id: 'admin-organizations',
	steps: [
		{
			element: '[data-tour="new-organization"]',
			title: 'Organisations',
			description:
				'Create an organisation to group conversations and manage team access under it.',
			side: 'bottom'
		},
		{
			element: '[data-tour="organizations-list"]',
			title: 'Your organisations',
			description: 'Open one to view details or manage its team.',
			side: 'top'
		}
	]
};
