import type { Tour } from '$lib/tours/types';

export const stepModerationTour: Tour = {
	id: 'admin-step-moderation',
	steps: [
		{
			element: '[data-tour="moderation-actions"]',
			title: 'Download the report',
			description:
				'Every statement and decision, with reasons and notes, as a CSV. New statements arrive on their own when you open or return to this page.',
			side: 'bottom'
		},
		{
			element: '[data-tour="moderation-guidance"]',
			title: 'Moderation rules',
			description:
				'What to do, what never to do, and your reasons for rejecting. It opens on your first visit and stays one click away.',
			side: 'bottom'
		},
		{
			element: '[data-tour="moderation-filters"]',
			title: 'Work through the queue',
			description:
				'The page opens on statements waiting for a decision. Switch to accepted, rejected or all statements, or search them.',
			side: 'bottom'
		},
		{
			element: '[data-tour="moderation-statements"]',
			title: 'Accept, edit or reject',
			description:
				'Accept a statement to publish it as written, edit it to split out more than one point, or reject it with a reason. To undo a decision, use Move back to waiting in the statement’s more menu.',
			side: 'top'
		}
	]
};
