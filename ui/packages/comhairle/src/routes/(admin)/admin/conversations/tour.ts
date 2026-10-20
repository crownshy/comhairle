import type { Tour } from '$lib/tours/types';

export const conversationsListTour: Tour = {
	id: 'admin-conversations-list',
	steps: [
		{
			element: '[data-tour="new-conversation"]',
			title: 'Create a conversation',
			description: 'Start a new conversation from scratch, or from a ready-made template.',
			side: 'bottom'
		},
		{
			element: '[data-tour="status-filters"]',
			title: 'Filter by status',
			description: 'Switch between draft, live and ended conversations.',
			side: 'bottom'
		},
		{
			element: '[data-tour="conversation-search"]',
			title: 'Search',
			description: 'Find a conversation by name.',
			side: 'bottom'
		},
		{
			element: '[data-tour="conversation-list"]',
			title: 'Your conversations',
			description:
				'Conversations you own and conversations shared with you both appear here, with their current status.',
			side: 'top'
		}
	]
};
