import type { Tour } from '$lib/tours/types';

/** Setup's content is entirely driven by the step's engagement tool, so there's one tour per
 * tool type rather than one for the page. `null` (e.g. "stories") means nothing to point at yet. */
const TOURS: Record<string, Tour> = {
	learn: {
		id: 'admin-step-setup-learn',
		steps: [
			{
				element: '[data-tour="learn-pages"]',
				title: 'Pages',
				description: 'Add, reorder and switch between the pages participants read through.',
				side: 'bottom'
			},
			{
				element: '[data-tour="learn-editor"]',
				title: 'Page content',
				description: 'The content of the page currently selected above.',
				side: 'top'
			}
		]
	},
	polis: {
		id: 'admin-step-setup-polis',
		steps: [
			{
				element: '[data-tour="polis-content"]',
				title: 'Content',
				description: 'The topic and description participants see before voting.',
				side: 'right'
			},
			{
				element: '[data-tour="polis-settings"]',
				title: 'Settings',
				description:
					'Required votes before moving on, whether to show the remaining statement count, and whether new statements need moderator approval before anyone sees them.',
				side: 'right'
			}
		]
	},
	heyform: {
		id: 'admin-step-setup-heyform',
		steps: [
			{
				element: '[data-tour="heyform-builder"]',
				title: 'Survey builder',
				description:
					'Build the survey questions here, just like the standalone HeyForm editor.',
				side: 'top'
			}
		]
	},
	thinkingspace: {
		id: 'admin-step-setup-thinkingspace',
		steps: [
			{
				element: '[data-tour="thinkingspace-topic"]',
				title: 'Topic',
				description: 'The overall subject participants reflect on.',
				side: 'bottom'
			},
			{
				element: '[data-tour="thinkingspace-followups"]',
				title: 'Follow-ups',
				description:
					'How many AI-generated follow-up questions participants must answer per main question before they can move on.',
				side: 'bottom'
			},
			{
				element: '[data-tour="thinkingspace-questions"]',
				title: 'Main questions',
				description: 'The questions participants answer one at a time. Drag to reorder.',
				side: 'top'
			}
		]
	},
	elicitationbot: {
		id: 'admin-step-setup-elicitationbot',
		steps: [
			{
				element: '[data-tour="elicitationbot-topic"]',
				title: 'Topic',
				description: 'What the bot should help participants share their opinions about.',
				side: 'bottom'
			}
		]
	},
	prioritization: {
		id: 'admin-step-setup-prioritization',
		steps: [
			{
				element: '[data-tour="prioritization-questions"]',
				title: 'Questions',
				description: 'Questions participants answer about every proposal.',
				side: 'bottom'
			},
			{
				element: '[data-tour="prioritization-section-questions"]',
				title: 'Per-section questions',
				description: 'Extra questions that only apply within one section of proposals.',
				side: 'top'
			},
			{
				element: '[data-tour="prioritization-proposals"]',
				title: 'Proposals',
				description: 'The options participants prioritise.',
				side: 'top'
			}
		]
	}
};

export function stepSetupTour(toolType: string | undefined): Tour | null {
	if (!toolType) return null;
	return TOURS[toolType] ?? null;
}
