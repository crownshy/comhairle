import type { Tour } from '$lib/tours/types';

export const teamConfigTour: Tour = {
	id: 'admin-conversation-configure-team',
	steps: [
		{
			element: '[data-tour="page-header"]',
			title: 'Team',
			description:
				'Only the owner sees this tab. Add a colleague by email to give them content editor access.',
			side: 'bottom'
		}
	]
};
