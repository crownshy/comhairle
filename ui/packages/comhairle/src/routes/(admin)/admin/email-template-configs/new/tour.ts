import type { Tour } from '$lib/tours/types';

export const newEmailTemplateConfigTour: Tour = {
	id: 'admin-email-template-config-new',
	steps: [
		{
			element: '[data-tour="section-header"]',
			title: 'Create a custom email',
			description:
				'Pick which built-in email this overrides, then fill in its slots. Use Preview to check it before saving.',
			side: 'bottom'
		}
	]
};
