import type { Tour } from '$lib/tours/types';

export const regionsTour: Tour = {
	id: 'admin-regions',
	steps: [
		{
			element: '[data-tour="regions-tabs"]',
			title: 'Geographic areas and Regions',
			description:
				'Geographic areas are drawn boundaries (import them or draw your own); Regions group areas together for organisations to target.',
			side: 'bottom'
		}
	]
};
