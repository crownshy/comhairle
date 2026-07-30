import { tryCatchAsync } from '$lib/utils/errorHandling';
import { typedObj } from '$lib/utils/types';
import { apiClient } from '@crownshy/api-client/client';

export type ChartData = {
	label: string;
	value: number;
};
type BarplotVariant = 'label' | 'value';
export type Barplot = {
	type: 'BarChart';
	variant: BarplotVariant;
	data: ChartData[];
};

export type Doughnut = {
	type: 'Doughnut';
	data: ChartData[];
};

export type Line = {
	type: 'Line';
	data: Record<string, number[]>;
};

type Text = {
	type: 'Text';
	data: string[];
};

type Title = string;
export type Insight = {
	title: Title;
	chart: Barplot | Doughnut | Line | Text;
};

export async function surveyInsightsLoader(workflowStepId: string) {
	// TODO: Undo when the backend data is ready
	const r1 = await tryCatchAsync(() =>
		apiClient.HeyFormGetFormReport({
			params: { workflow_step_id: workflowStepId }
		})
	);
	const r2 = await tryCatchAsync(() =>
		apiClient.HeyFormGetSubmissions({
			params: { workflow_step_id: workflowStepId }
		})
	);

	console.log('r1:', r1);
	console.log('r2:', r2);

	const insights: Insight[] = [
		{
			title: 'Engagement',
			chart: typedObj<Barplot>({
				type: 'BarChart',
				variant: 'value',
				data: [
					{
						label: 'Online petition platform',
						value: 39
					},
					{
						label: 'Pol.is discussion',
						value: 29
					},
					{
						label: 'In-person hearing',
						value: 24
					},
					{
						label: 'Deliberation workshop',
						value: 14
					},
					{
						label: 'Other',
						value: 5
					}
				]
			})
		},
		{
			title: 'Age',
			chart: typedObj<Barplot>({
				type: 'BarChart',
				variant: 'label',
				data: [
					{
						label: '18-20',
						value: 52
					},
					{
						label: '21-30',
						value: 90
					},
					{
						label: '31-40',
						value: 70
					},
					{
						label: '41-50',
						value: 20
					},
					{
						label: '51-60',
						value: 60
					},
					{
						label: '61+',
						value: 63
					}
				]
			})
		},
		{
			title: 'Can we contact you?',
			chart: typedObj<Doughnut>({
				type: 'Doughnut',
				data: [
					{
						label: 'Yes',
						value: 2
					},
					{
						label: 'No',
						value: 1
					}
				]
			})
		},
		{
			title: 'Responses over time',
			chart: typedObj<Line>({
				type: 'Line',
				data: {
					Frequency: [
						1.5, 2, 1.8, 2.2, 1.6, 1.9, 2.1, 1.7, 2.3, 1.4, 8, 7.6, 8.3, 7.9, 8.5, 7.7,
						8.1, 8.4, 7.8, 8.2
					]
				}
			})
		},
		{
			title: 'Do you prefer cats or dogs?',
			chart: typedObj<Text>({
				type: 'Text',
				data: [
					"Honestly, I'm a cat person. They're low-maintenance and have a quiet sort of elegance.",
					'Dogs all the way. Nothing beats coming home to a wagging tail and pure excitement.',
					'Why choose? Both have their charm — cats for calm, dogs for energy.',
					'Cats win for me. I love that they do their own thing and still occasionally grace you with affection.',
					'Dogs are better companions if you want loyalty and constant interaction, but cats are perfect introverts.',
					"Cats. No contest. You don't have to walk them in the rain.",
					'I prefer dogs because they actually seem to like people. Cats just tolerate us.',
					"Cats are underrated. They're affectionate in their own subtle way, just not needy about it.",
					'Dogs force you outside and keep you active. That alone makes them the better pet for me.',
					"I've had both and honestly each fills a completely different role in your life.",
					"Cats. They're basically small, fluffy roommates who pay rent in purring.",
					'Dogs. The bond you build with a dog over years is unlike anything else.',
					"Cats because they're cleaner and quieter. My apartment stays peaceful.",
					'Dogs win — try teaching a cat to fetch. Or to come when called. Exactly.',
					"I lean cats. There's something special about earning a cat's trust.",
					"Dogs are extroverted pets for extroverted people. I'm an introvert, so cats.",
					"Cats are better for busy people. They don't need constant attention or walks.",
					'Dogs are like having a permanent cheerleader. Cats are like having a moody roommate.',
					'Both are great, but cats fit my lifestyle better right now.',
					'Dogs, because the outdoors is better with one, and walks become a ritual.',
					'Cats. A purring cat on your lap while reading a book is peak comfort.',
					'Dogs for the unconditional love. Cats make you work for every bit of affection.',
					'I think it really depends on your personality more than anything.',
					"Cats. They're independent but still love you, which feels more genuine somehow.",
					"Dogs. They greet you like you've been gone for years even if it was five minutes.",
					'Cats win on logistics — no walks, no barking, no begging at the table.',
					'Dogs are better with kids in my experience. More patient, more interactive.',
					'Cats are smarter than people give them credit for. They just choose when to show it.',
					'Dogs hands down. The joy they bring is just unmatched.',
					"I've always been a cat person. Dogs are too much energy for me.",
					"Dogs are social creatures. If you're lonely, a dog can literally change your life.",
					"Cats are mysterious and elegant. Dogs are goofy and lovable. I'll take goofy.",
					'Cats, because a litter box beats walking a dog at 6am in January.',
					'Dogs. You can actually take them places — hikes, road trips, cafés.',
					'Cats are perfect working-from-home companions. They nap near you quietly.',
					'Dogs are better motivators. They need exercise, which means you get exercise too.',
					'Cats are like living with a tiny lion. Dogs are like living with your biggest fan.',
					'Dogs because they protect your home and make you feel safer.',
					"Cats because they're cheaper to care for and generally healthier long-term.",
					"Dogs win for me. Training them builds a real partnership that cats just can't match.",
					'Cats. The way they knead and purr is scientifically proven to lower stress.',
					"Dogs. Ever seen a cat try to be a service animal? Didn't think so.",
					'I love both but cats are just more convenient for apartment living.',
					'Dogs because they actually listen. Well, sometimes. More than cats at least.',
					'Cats are more environmentally friendly — smaller carbon footprint overall.',
					'Dogs all the way. Hiking with a trail buddy is unbeatable.',
					'Cats. Their independence means they love you because they choose to.',
					"Dogs. There's a reason they're called man's best friend.",
					"Cats because they're quiet and don't disturb the neighbors.",
					'Dogs are better for mental health — they get you out and socializing.',
					'Cats. They judge you silently, which keeps you humble.',
					'Dogs because you can train them to do almost anything. Cats train you instead.',
					'Cats are better for night owls — no early morning potty breaks required.',
					'Dogs because the human-dog bond goes back thousands of years for a reason.',
					"Cats. They're fluffy, warm, and self-sufficient. What more do you need?",
					'Dogs because they give you a reason to go outside every single day.',
					'Cats win for me — lower vet bills, less space, zero walking required.',
					"Dogs. Nothing compares to the look in a dog's eyes when you come home.",
					"Cats are the introvert's dream pet. Minimal demands, maximum coziness.",
					'Dogs because adventure is better shared, and dogs are always ready.'
				]
			})
		}
	] as const;

	return { survey: { insights } };
}
