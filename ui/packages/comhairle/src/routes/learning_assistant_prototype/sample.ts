// PROTOTYPE - throwaway. Static sample data so the variants can show a populated
// Q&A state without a live chat session. Mirrors the shape LearningAssistant builds.

export type SampleSource = { name: string };
export type SampleQA = {
	id: string;
	question: string;
	answer: string;
	sources: SampleSource[];
	timeLabel: string;
};

export const SAMPLE_QAS: SampleQA[] = [
	{
		id: 'qa-1',
		question: 'Will the new cycle lanes remove any parking on the high street?',
		answer: 'Yes. The proposal reallocates the eastern kerb between Bridge Street and the market square, removing 18 on-street parking bays to make room for a two-way protected cycle lane. The report notes the nearest replacement capacity is the Mill Road car park, a 4-minute walk away.',
		sources: [
			{ name: 'High Street Redesign - Options Report.pdf' },
			{ name: 'Parking Impact Assessment.pdf' }
		],
		timeLabel: '2m ago'
	},
	{
		id: 'qa-2',
		question: 'How was the budget for this decided?',
		answer: 'The £2.4m envelope comes from the Active Travel capital grant, ring-fenced for walking and cycling infrastructure. It cannot be moved to other services such as road resurfacing.',
		sources: [{ name: 'Funding Background Note.pdf' }],
		timeLabel: '6m ago'
	}
];

// Suggested starter prompts (used by variant C's example grid).
export const STARTER_PROMPTS: string[] = [
	'What is this consultation actually deciding?',
	'Summarise the main proposal in plain language',
	'What are the arguments against it?',
	'How will this affect me if I live nearby?'
];
