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
		},
		{
			element: '[data-tour="glossary-toolbar"]',
			title: 'Glossary terms',
			description:
				'Define terms and their explanation appears as a hover tooltip wherever the term is used in Learn steps. Add a term by hand, or import a list from a two-column CSV.',
			side: 'bottom'
		}
	]
};
