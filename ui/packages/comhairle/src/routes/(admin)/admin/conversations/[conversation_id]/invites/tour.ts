import type { Tour } from '$lib/tours/types';

const INTRO: Tour['steps'] = [
	{
		element: '[data-tour="page-header"]',
		title: 'Recruit',
		description:
			'Invite participants by email, generate open links to share anywhere, or print QR codes for an in-person event. Available once the conversation is launched.',
		side: 'bottom'
	},
	{
		element: '[data-tour="invite-subtabs"]',
		title: 'Email, Open Links and Physical',
		description: 'Switch between the ways you invite people.',
		side: 'bottom'
	}
];

/** Invites is one page whose content swaps with the `?subtab=` query param, so the tour
 * depends on which subtab is active rather than being fixed. */
export function invitesTour(activeTab: string): Tour {
	if (activeTab === 'open-links') {
		return {
			id: 'admin-conversation-invites-open-links',
			steps: [
				...INTRO,
				{
					element: '[data-tour="open-invite-actions"]',
					title: 'Create a link',
					description:
						'Give each link a label, such as "Newsletter" or "Town hall", so you can tell them apart later.',
					side: 'bottom'
				},
				{
					element: '[data-tour="open-invite-list"]',
					title: 'Your links',
					description:
						'Copy a link or use its QR code. The Accepted column shows how many people joined through each one, so you can see which channels work best.',
					side: 'top'
				}
			]
		};
	}

	if (activeTab === 'physical') {
		return { id: 'admin-conversation-invites-physical', steps: INTRO };
	}

	return {
		id: 'admin-conversation-invites-email',
		steps: [
			...INTRO,
			{
				element: '[data-tour="email-invite-form"]',
				title: 'Invite by email',
				description:
					'Enter one or more email addresses to send a personal invite link to each.',
				side: 'bottom'
			},
			{
				element: '[data-tour="email-invite-list"]',
				title: 'Sent invites',
				description: 'Track who has accepted and who is still pending.',
				side: 'top'
			}
		]
	};
}
