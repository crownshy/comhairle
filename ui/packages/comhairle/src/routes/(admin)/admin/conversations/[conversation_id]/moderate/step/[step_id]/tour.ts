import type { Tour, TourStep } from '$lib/tours/types';

const rulesStep: TourStep = {
	element: '[data-tour="moderation-rules"]',
	title: 'Moderation rules',
	description:
		'What matters most, what to do and never do, your reasons for rejecting, and a practice round with made-up statements.',
	side: 'bottom'
};

/** The Moderation rules button only shows for people who can moderate, so only they get its step. */
export const stepModerationTour = (canModerate: boolean): Tour => ({
	id: 'admin-step-moderation',
	steps: [
		{
			element: '[data-tour="moderation-actions"]',
			title: 'Download the report',
			description:
				'Every statement and decision, with reasons and notes, as a CSV. New statements arrive on their own when you open or return to this page.',
			side: 'bottom'
		},
		...(canModerate ? [rulesStep] : []),
		{
			element: '[data-tour="moderation-filters"]',
			title: 'Work through the queue',
			description:
				'The page opens on statements waiting for a decision. Switch to accepted, rejected or all statements, or search them.',
			side: 'bottom'
		},
		{
			element: '[data-tour="moderation-statements"]',
			title: 'Accept, reject or split',
			description:
				'Accept a statement to publish it as written, split or reword it if it makes more than one point, or reject it with a reason. To undo a decision, use Move back to waiting in the statement’s more menu.',
			side: 'top'
		}
	]
});
