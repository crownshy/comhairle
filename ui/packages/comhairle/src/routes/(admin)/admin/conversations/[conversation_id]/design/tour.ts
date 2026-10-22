import type { Tour } from '$lib/tours/types';

export const processDesignTour: Tour = {
	id: 'admin-conversation-design',
	steps: [
		{
			element: '[data-tour="section-header"]',
			title: 'Process design',
			description:
				'Build the ordered list of steps participants go through, one engagement tool at a time.',
			side: 'bottom'
		},
		{
			element: '[data-tour="choose-template"]',
			title: 'Start from a template',
			description:
				'Apply a ready-made set of steps instead of building from blank. This replaces the current steps, so use it before you have much configured.',
			side: 'bottom'
		}
	]
};
