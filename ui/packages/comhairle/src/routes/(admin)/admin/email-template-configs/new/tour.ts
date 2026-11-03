import type { Tour } from '$lib/tours/types';

export const newEmailTemplateConfigTour: Tour = {
	id: 'admin-email-template-config-new',
	steps: [
		{
			element: '[data-tour="section-header"]',
			title: 'Create a custom email',
			description:
				'Pick the email and who it applies to, then change the wording. The preview updates as you type.',
			side: 'bottom'
		}
	]
};
