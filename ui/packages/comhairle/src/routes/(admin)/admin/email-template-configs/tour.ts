import type { Tour } from '$lib/tours/types';

export const emailTemplateConfigsTour: Tour = {
	id: 'admin-email-template-configs',
	steps: [
		{
			element: '[data-tour="section-header"]',
			title: 'Custom emails',
			description:
				"Override Comhairle's default emails (invites, notifications, etc) with your own wording per organisation or conversation.",
			side: 'bottom'
		},
		{
			element: '[data-tour="email-configs-list"]',
			title: 'Every email Comhairle sends',
			description:
				'Each email type shows the version participants receive. Customise one to use your own wording, for all conversations or just one.',
			side: 'top'
		}
	]
};
