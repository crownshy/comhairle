import type { Tour } from '$lib/tours/types';

export const moderateTour: Tour = {
	id: 'admin-conversation-moderate',
	steps: [
		{
			element: '[data-tour="moderate-steps"]',
			title: 'Moderate contributions',
			description:
				'Each engagement step that collects participant statements gets a card here. Only Participant-led poll steps can be moderated; other tools show as not applicable.',
			side: 'top'
		}
	]
};
