import type { Tour } from '$lib/tours/types';

export const notificationsTour: Tour = {
	id: 'admin-conversation-notifications',
	steps: [
		{
			element: '[data-tour="page-header"]',
			title: 'Notify',
			description:
				'Send a one-off message to everyone who has taken part. There is no way to pick individual recipients here, so check it carefully before sending.',
			side: 'bottom'
		}
	]
};
