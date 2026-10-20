import type { Tour } from '$lib/tours/types';

export const detailsConfigTour: Tour = {
	id: 'admin-conversation-configure-details',
	steps: [
		{
			element: '[data-tour="page-header"]',
			title: 'Details',
			description:
				'The title, description, supported languages and banner image participants see.',
			side: 'bottom'
		},
		{
			element: '[data-tour="details-fields"]',
			title: 'Fields save as you type',
			description:
				'Add a language under Language options to unlock translation for every field below it.',
			side: 'top'
		}
	]
};
