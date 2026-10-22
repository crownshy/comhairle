import type { Tour } from '$lib/tours/types';

export const systemRolesTour: Tour = {
	id: 'admin-system-roles',
	steps: [
		{
			element: '[data-tour="section-header"]',
			title: 'System roles',
			description:
				'Platform-wide roles, separate from the roles assigned within a single conversation or organisation.',
			side: 'bottom'
		}
	]
};
