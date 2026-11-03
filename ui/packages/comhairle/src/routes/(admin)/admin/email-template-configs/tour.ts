import type { Tour } from '$lib/tours/types';

export const emailTemplateConfigsTour: Tour = {
	id: 'admin-email-template-configs',
	steps: [
		{
			element: '[data-tour="section-header"]',
			title: 'Email templates',
			description:
				'The emails Comhairle sends to participants. Pick an email type from the tabs to see and change its wording.',
			side: 'bottom'
		},
		{
			element: '[data-tour="email-configs-list"]',
			title: 'One default, a few exceptions',
			description:
				'The default goes to every conversation. Give a conversation its own version only when it needs different wording. Use Preview to check any email.',
			side: 'top'
		}
	]
};
