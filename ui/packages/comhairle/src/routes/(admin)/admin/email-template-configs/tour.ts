import type { Tour } from '$lib/tours/types';

export const emailTemplateConfigsTour: Tour = {
	id: 'admin-email-template-configs',
	steps: [
		{
			element: '[data-tour="page-header"]',
			title: 'Custom emails',
			description:
				"Override Comhairle's default emails (invites, notifications, etc) with your own wording per organisation or conversation.",
			side: 'bottom'
		},
		{
			element: '[data-tour="email-configs-list"]',
			title: 'Your custom emails',
			description: 'Add a custom email to override one of the default templates.',
			side: 'top'
		}
	]
};
