import type { Tour } from '$lib/tours/types';

export const moderationPolicyConfigTour: Tour = {
	id: 'admin-conversation-configure-moderation-policy',
	steps: [
		{
			element: '[data-tour="moderation-policy"]',
			title: 'Moderation policy',
			description:
				'The list of reasons moderators pick from when rejecting a statement. Edit the defaults or add your own; any change makes this conversation use its own list.',
			side: 'top'
		}
	]
};
