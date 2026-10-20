import type { Tour } from '$lib/tours/types';

export const glossaryConfigTour: Tour = {
	id: 'admin-conversation-configure-glossary',
	steps: [
		{
			element: '[data-tour="page-header"]',
			title: 'Glossary',
			description:
				'Define terms and their explanation appears as a hover tooltip wherever the term is used in Learn steps.',
			side: 'bottom'
		},
		{
			element: '[data-tour="glossary-toolbar"]',
			title: 'Add, import and search terms',
			description: 'Add a term by hand, or import a list from a two-column CSV.',
			side: 'bottom'
		}
	]
};
