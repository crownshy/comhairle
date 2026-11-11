// Sample content for the home-lab experiment. Nothing here comes from the API.
// Records use the real ConversationDto shape so staging components render them as-is.
import type { ConversationDto } from '@crownshy/api-client/api';
import seattleImage from '$lib/assets/seattle_usa.jpg';
import climateImage from '$lib/assets/climageuk.jpg';
import taiwanImage from '$lib/assets/vtaiwan.jpg';
// Free photos from Unsplash (Unsplash License), downloaded for this prototype only.
import librariesImage from './media/libraries.jpg';
import schoolStreetsImage from './media/school-streets.jpg';
import canalImage from './media/canal.jpg';
import tramHousingImage from './media/tram-housing.jpg';

// Image imports are typed loosely in this repo; they are URLs at runtime.
const seattle = seattleImage as string;
const climate = climateImage as string;
const taiwan = taiwanImage as string;

const libraries = librariesImage as string;
const schoolStreets = schoolStreetsImage as string;
const canal = canalImage as string;
const tramHousing = tramHousingImage as string;

const PHOTO_POOL = [seattle, climate, taiwan, libraries, schoolStreets, canal, tramHousing];

export type LabState = 'live' | 'closing' | 'upcoming' | 'results';

export const STATE_LABELS: Record<LabState, string> = {
	live: 'Live now',
	closing: 'Closing soon',
	upcoming: 'Coming soon',
	results: 'Results ready'
};

export type Quote = { text: string; agreePercent: number };

/** Moving media for a featured slide. A muted looping video is much lighter than a GIF. */
export type FeaturedMedia = { kind: 'gif' | 'video'; src: string };

/** Fields the redesign wants that the API does not have yet. */
type ExperimentFields = {
	state: LabState;
	isFeatured: boolean;
	topic: string;
	estimatedMinutes: number;
	/** Days until it closes. For upcoming: days until it opens. For results: days since it closed. */
	days: number;
	/** How long ago it opened, for the Latest sort. Negative while upcoming. */
	openedDaysAgo: number;
	participantCount: number;
	quotes: Quote[];
	featuredMedia?: FeaturedMedia;
};

export type LabConversation = ConversationDto & ExperimentFields;

type Seed = Pick<ConversationDto, 'id' | 'title' | 'shortDescription' | 'imageUrl'> &
	ExperimentFields;

function toConversation(seed: Seed): LabConversation {
	return {
		allowRevisitAfterFinishing: false,
		showThankYouPageAnnonInstructions: false,
		showThankYouPageNextSteps: false,
		showThankyouPageFeedbackButton: false,
		description: seed.shortDescription,
		enableQaChatBot: false,
		enableSignupPrompts: false,
		isComplete: seed.state === 'results',
		isInviteOnly: false,
		isLive: seed.state === 'live' || seed.state === 'closing',
		isPublic: true,
		primaryLocale: 'en',
		supportedLanguages: ['en'],
		tags: [seed.topic],
		slug: seed.id,
		...seed
	};
}

const SEEDS: Seed[] = [
	{
		id: 'cycling-budget',
		title: 'How should the city spend its cycling budget?',
		shortDescription: 'Help decide where next year’s new cycle lanes go.',
		imageUrl: seattle,
		state: 'live',
		isFeatured: true,
		topic: 'Transport',
		estimatedMinutes: 10,
		days: 19,
		openedDaysAgo: 12,
		participantCount: 1240,
		quotes: [
			{
				text: 'Protected lanes near schools first. Parents won’t let kids cycle otherwise.',
				agreePercent: 82
			},
			{
				text: 'Fix the potholes on existing lanes before building new ones.',
				agreePercent: 67
			},
			{ text: 'Connect the lanes up, gaps are where people get hurt.', agreePercent: 79 }
		]
	},
	{
		id: 'greener-centre',
		title: 'A greener, quieter city centre',
		shortDescription: 'Trees, seating and fewer cars: what would make the centre better?',
		imageUrl: climate,
		state: 'live',
		isFeatured: true,
		topic: 'Environment',
		estimatedMinutes: 12,
		days: 27,
		openedDaysAgo: 5,
		participantCount: 860,
		quotes: [
			{
				text: 'More benches and shade. Older people need somewhere to rest.',
				agreePercent: 88
			},
			{ text: 'Close one street to cars on Saturdays as a trial.', agreePercent: 64 }
		]
	},
	{
		id: 'late-night-transport',
		title: 'Late-night transport after 11pm',
		shortDescription: 'Tell us how you get home late and what would help.',
		imageUrl: taiwan,
		state: 'closing',
		isFeatured: true,
		topic: 'Transport',
		estimatedMinutes: 8,
		days: 2,
		openedDaysAgo: 26,
		participantCount: 2105,
		quotes: [
			{ text: 'A 1am bus to the suburbs would change my whole week.', agreePercent: 74 },
			{ text: 'Shift workers get forgotten. Hospitals run all night.', agreePercent: 81 }
		]
	},
	{
		id: 'libraries-teens',
		title: 'Making libraries work for teenagers',
		shortDescription: 'What would get you into your local library?',
		imageUrl: libraries,
		state: 'live',
		isFeatured: false,
		topic: 'Community',
		estimatedMinutes: 12,
		days: 33,
		openedDaysAgo: 2,
		participantCount: 312,
		quotes: [
			{
				text: 'Keep it open on Sundays, it’s the only quiet place to study.',
				agreePercent: 91
			}
		]
	},
	{
		id: 'school-streets',
		title: 'School streets pilot: what next?',
		shortDescription: 'The pilot closed roads outside four schools. Should it continue?',
		imageUrl: schoolStreets,
		state: 'live',
		isFeatured: false,
		topic: 'Transport',
		estimatedMinutes: 6,
		days: 39,
		openedDaysAgo: 8,
		participantCount: 540,
		quotes: []
	},
	{
		id: 'canal-rewilding',
		title: 'Rewilding the canal banks',
		shortDescription: 'Plans to let parts of the canal banks grow wild.',
		imageUrl: canal,
		state: 'upcoming',
		isFeatured: false,
		topic: 'Environment',
		estimatedMinutes: 10,
		days: 20,
		openedDaysAgo: -20,
		participantCount: 0,
		quotes: []
	},
	{
		id: 'tram-housing',
		title: 'Housing near the new tram line',
		shortDescription: 'Where should new homes go along the route?',
		imageUrl: tramHousing,
		state: 'results',
		isFeatured: false,
		topic: 'Housing',
		estimatedMinutes: 10,
		days: 40,
		openedDaysAgo: 70,
		participantCount: 3020,
		quotes: [
			{ text: 'New homes should come with a GP surgery and school places.', agreePercent: 88 }
		]
	}
];

const BASE = SEEDS.map(toConversation);

const MANY_TOPICS = ['Transport', 'Housing', 'Environment', 'Community', 'Health', 'Budget'];
const MANY_STATES: LabState[] = ['live', 'live', 'closing', 'upcoming', 'results'];

const GENERATED = Array.from({ length: 150 }, (_, i) => {
	const topic = MANY_TOPICS[i % MANY_TOPICS.length];
	return toConversation({
		id: `generated-${i + 1}`,
		title: `${topic} conversation ${i + 1}`,
		shortDescription: 'A generated conversation for testing long lists.',
		imageUrl: PHOTO_POOL[i % PHOTO_POOL.length],
		state: MANY_STATES[i % MANY_STATES.length],
		isFeatured: false,
		topic,
		estimatedMinutes: 5 + (i % 4) * 3,
		days: (i % 28) + 1,
		openedDaysAgo:
			MANY_STATES[i % MANY_STATES.length] === 'upcoming' ? -((i % 20) + 1) : (i * 3) % 90,
		participantCount: (i * 37) % 900,
		quotes: []
	});
});

export type Dataset = 'normal' | 'empty' | 'many';

export function conversationsFor(dataset: Dataset): LabConversation[] {
	if (dataset === 'empty') return [];
	if (dataset === 'many') return [...BASE, ...GENERATED];
	return BASE;
}

export function findConversation(id: string): LabConversation | undefined {
	return conversationsFor('many').find((c) => c.id === id);
}

/** Past conversations with a published report, offered on the empty state. */
export const PAST_REPORTS = BASE.filter((c) => c.state === 'results');

export const SAMPLE_USER = {
	firstName: 'Aoife',
	initials: 'AM',
	joinedCount: 3,
	inProgress: { conversationId: 'cycling-budget', stepIndex: 1, stepCount: 3 },
	reportReady: { conversationId: 'tram-housing', yourView: 'You agreed with 72% of people' },
	interests: ['Transport', 'Housing']
};

export const STEPS = [
	{ name: 'Learn the options', minutes: 2 },
	{ name: 'Share your views', minutes: 6 },
	{ name: 'See what others think', minutes: 2 }
];

export const FAQS = [
	{
		question: 'Who is running this?',
		answer: 'The organisation named at the top, using Comhairle.'
	},
	{
		question: 'How will my answers be used?',
		answer: 'Your answers are anonymous and only used for this conversation.'
	},
	{
		question: 'Is AI involved?',
		answer: 'AI helps group similar views. People review every summary.'
	},
	{
		question: 'When will I see results?',
		answer: 'A report is published here a few weeks after the conversation closes.'
	}
];

const plural = (n: number, word: string) => `${n} ${word}${n === 1 ? '' : 's'}`;

/** One wording for timing everywhere: always counted in days from today. */
export function timingLabel(conversation: LabConversation): string {
	const { state, days } = conversation;
	if (state === 'upcoming') return `Opens in ${plural(days, 'day')}`;
	if (state === 'results') return `Closed ${plural(days, 'day')} ago`;
	return days === 0 ? 'Closes today' : `Closes in ${plural(days, 'day')}`;
}

export function participantsLabel(conversation: LabConversation): string {
	return plural(conversation.participantCount, 'participant').replace(
		String(conversation.participantCount),
		conversation.participantCount.toLocaleString()
	);
}
