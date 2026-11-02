import type { Tour } from '$lib/tours/types';

export const detailsConfigTour: Tour = {
	id: 'admin-conversation-configure-details',
	steps: [
		{
			element: '[data-tour="page-header"]',
			title: 'Details',
			description:
				'The title, description, supported languages and banner image participants see, plus who can take part.',
			side: 'bottom'
		},
		{
			element: '[data-tour="details-fields"]',
			title: 'Fields save as you type',
			description:
				'Add a language under Language options to unlock translation for every field below it.',
			side: 'top'
		},
		{
			element: '[data-tour="access-settings"]',
			title: 'Visibility and participation settings',
			description:
				'Decide who can see this conversation and who can take part. Each toggle saves immediately, so review these before you launch.',
			side: 'top'
		}
	]
};
