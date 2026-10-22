import type { Tour } from '$lib/tours/types';

export const emailTemplateConfigTour: Tour = {
	id: 'admin-email-template-config',
	steps: [
		{
			element: '[data-tour="section-header"]',
			title: 'Edit this custom email',
			description:
				'Preview your changes before saving, or delete it to fall back to the default.',
			side: 'bottom'
		}
	]
};
