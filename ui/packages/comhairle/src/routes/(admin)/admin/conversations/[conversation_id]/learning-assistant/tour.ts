import type { Tour } from '$lib/tours/types';

export const learningAssistantTour: Tour = {
	id: 'admin-conversation-learning-assistant',
	steps: [
		{
			element: '[data-tour="page-header"]',
			title: 'Learning Assistant',
			description:
				'A Q&A chatbot that answers participant questions from documents you upload here.',
			side: 'bottom'
		}
	]
};
