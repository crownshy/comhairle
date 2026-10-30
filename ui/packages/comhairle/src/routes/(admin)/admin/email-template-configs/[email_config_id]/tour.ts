import type { Tour } from '$lib/tours/types';

export const emailTemplateConfigTour: Tour = {
	id: 'admin-email-template-config',
	steps: [
		{
			element: '[data-tour="section-header"]',
			title: 'Edit this custom email',
			description:
				'Change the wording and check the preview, then save. Remove it to go back to the default.',
			side: 'bottom'
		}
	]
};
