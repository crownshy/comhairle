import type { Tour } from '$lib/tours/types';

export const stepConfigureTour: Tour = {
	id: 'admin-conversation-step-configure',
	steps: [
		{
			element: '[data-tour="step-name-field"]',
			title: 'Name',
			description: 'The name participants see for this step.',
			side: 'top'
		},
		{
			element: '[data-tour="step-description-field"]',
			title: 'Description',
			description: 'Tells participants what this step is for.',
			side: 'top'
		},
		{
			element: '[data-tour="step-data-protocol-field"]',
			title: 'Data protocol',
			description:
				"Explains to participants how their answers in this step are used. It starts as the tool's default, so edit it to write your own or leave it as is.",
			side: 'top'
		},
		{
			element: '[data-tour="step-danger-zone"]',
			title: 'Deleting a step',
			description:
				'Permanently removes the step and its configuration. This cannot be undone.',
			side: 'top'
		}
	]
};
