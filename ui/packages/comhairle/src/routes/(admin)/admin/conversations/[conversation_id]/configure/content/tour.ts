import type { Tour } from '$lib/tours/types';

export const contentConfigTour: Tour = {
	id: 'admin-conversation-configure-content',
	steps: [
		{
			element: '[data-tour="page-header"]',
			title: 'Content',
			description:
				"Participant-facing copy: privacy policy, FAQs, thank-you message and call to action. Leave any field blank to use Comhairle's default text.",
			side: 'bottom'
		},
		{
			element: '[data-tour="content-fields"]',
			title: 'Fields save as you type',
			description:
				'Each field autosaves. If you support more than one language, translate it from Configure → Details.',
			side: 'top'
		}
	]
};
