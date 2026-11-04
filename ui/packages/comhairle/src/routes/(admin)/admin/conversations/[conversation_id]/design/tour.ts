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
		},
		{
			element: '[data-tour="step-flags-key"]',
			title: 'Required and revisitable',
			description:
				"Set these from each step's ⋮ menu. Required steps are marked * and can't be skipped. Revisitable lets participants go back to a step. Surveys can't be revisited.",
			side: 'bottom'
		}
	]
};
