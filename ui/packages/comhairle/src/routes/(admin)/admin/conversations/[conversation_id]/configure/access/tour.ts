import type { Tour } from '$lib/tours/types';

export const accessConfigTour: Tour = {
	id: 'admin-conversation-configure-access',
	steps: [
		{
			element: '[data-tour="page-header"]',
			title: 'Access',
			description: 'Decide who can see this conversation and who can take part in it.',
			side: 'bottom'
		},
		{
			element: '[data-tour="access-settings"]',
			title: 'Visibility and participation settings',
			description:
				'Each toggle saves immediately. Review these before you launch, since some change what participants can see or do.',
			side: 'top'
		}
	]
};
