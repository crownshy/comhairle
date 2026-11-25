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
// Real banner images with awkward shapes, for the scenario panel's Image row. Not for production.
import testCutoutImage from './media/test-cutout.webp';
import testWidePortraitImage from './media/test-wide-portrait.jpg';
import testLandscapeImage from './media/test-landscape.jpg';
import testGroupImage from './media/test-group.jpg';

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
	/** Who runs the conversation, shown on every card. Made-up teams, not real bodies. */
	organiser: string;
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
		title: 'Active Travel Investment Plan 2027–2032 – Prioritising Safe, Connected Cycling and Walking Routes Across the City',
		shortDescription:
			'Tell us where you think new cycle lanes, crossings and walking routes are needed most. Your views will help decide how the city spends its active travel budget over the next five years.',
		imageUrl: seattle,
		state: 'live',
		isFeatured: true,
		topic: 'Transport',
		organiser: 'Active Travel Team',
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
		title: 'City Centre Transformation – Creating a Greener, Quieter and More Accessible Place to Live, Work and Visit',
		shortDescription:
			'Share your views on proposals for more trees, seating and space for people in the city centre. Your feedback will shape the final design before building work begins.',
		imageUrl: climate,
		state: 'live',
		isFeatured: true,
		topic: 'Environment',
		organiser: 'Parks and Green Spaces',
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
		title: 'Night-Time Economy and Late-Night Public Transport – Improving Safe and Affordable Journeys Home After 11pm',
		shortDescription:
			'Tell us how you travel home late at night and what would make it safer, easier or cheaper. Your answers will inform the next regional bus and rail contracts.',
		imageUrl: taiwan,
		state: 'closing',
		isFeatured: true,
		topic: 'Transport',
		organiser: 'Transport Partnership',
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
		title: 'Libraries for Young People – Reimagining Local Library Spaces, Opening Hours and Services for 12 to 18 Year Olds',
		shortDescription:
			'We want to hear from young people and their families about what would make local libraries more welcoming. Your ideas will help shape new spaces and services from next year.',
		imageUrl: libraries,
		state: 'live',
		isFeatured: false,
		topic: 'Community',
		organiser: 'Library Services',
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
		title: 'School Streets Pilot Review – Deciding the Future of Timed Road Closures Outside Primary Schools',
		shortDescription:
			'The pilot closed roads to traffic at drop-off and pick-up times outside four primary schools. Tell us how it affected you, and whether it should continue or expand.',
		imageUrl: schoolStreets,
		state: 'live',
		isFeatured: false,
		topic: 'Transport',
		organiser: 'Active Travel Team',
		estimatedMinutes: 6,
		days: 39,
		openedDaysAgo: 8,
		participantCount: 540,
		quotes: []
	},
	{
		id: 'canal-rewilding',
		title: 'Canal Corridor Biodiversity Plan – Restoring Natural Habitats Along the Canal Banks While Protecting Access to the Towpath',
		shortDescription:
			'Find out about plans to let parts of the canal banks grow wild to support wildlife. Tell us how you use the towpath so that access for walkers and cyclists can be protected.',
		imageUrl: canal,
		state: 'upcoming',
		isFeatured: false,
		topic: 'Environment',
		organiser: 'Waterways Volunteers',
		estimatedMinutes: 10,
		days: 20,
		openedDaysAgo: -20,
		participantCount: 0,
		quotes: []
	},
	{
		id: 'tram-housing',
		title: 'Tram Line Extension Development Framework – Planning New Homes, Local Services and Green Space Along the Route',
		shortDescription:
			'Tell us where new homes should go along the extended tram route, and which services, such as GP surgeries, schools and parks, are needed to support them.',
		imageUrl: tramHousing,
		state: 'results',
		isFeatured: false,
		topic: 'Housing',
		organiser: 'Housing Services',
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
const ORGANISER_FOR_TOPIC: Record<string, string> = {
	Transport: 'Transport Partnership',
	Housing: 'Housing Services',
	Environment: 'Parks and Green Spaces',
	Community: 'Community Development',
	Health: 'Public Health Team',
	Budget: 'Finance Team'
};
const MANY_STATES: LabState[] = ['live', 'live', 'closing', 'upcoming', 'results'];

// Realistic lengths for titles and descriptions, in the same topic order as MANY_TOPICS.
const GENERATED_COPY: { title: string; shortDescription: string }[] = [
	{
		title: 'Regional Bus Network Review – Redesigning Routes, Timetables and Fares to Better Connect Towns and Rural Communities',
		shortDescription:
			'Tell us which journeys matter most to you and where the current bus network lets you down. Your views will help shape a new network due to start in 2028.'
	},
	{
		title: 'Local Housing Strategy 2027–2032 – Increasing the Supply of Affordable, Energy-Efficient Homes for Every Stage of Life',
		shortDescription:
			'Share your experience of finding and keeping a home in the area. Your views will inform the council’s priorities for new homes, repairs and support over the next five years.'
	},
	{
		title: 'Climate Change Adaptation Plan – Preparing Our Communities, Homes and Infrastructure for Flooding, Heatwaves and Storms',
		shortDescription:
			'Tell us how extreme weather has affected you and what support would help most. Your answers will shape how the council prepares for a changing climate.'
	},
	{
		title: 'Better Joined Up Services – Experiencing Easier Access to Available Financial Support and Services Through Information Sharing',
		shortDescription:
			'Tell us what you think about public services sharing information to help families get the support they need. Your views will inform how Scotland develops its approach to data sharing.'
	},
	{
		title: 'Community Mental Health and Wellbeing Services – Shaping Early Support That Is Easier to Find, Reach and Use',
		shortDescription:
			'We want to understand what helps people look after their mental health and where they go for support. Your views will help design services closer to home.'
	},
	{
		title: 'Setting the Council Budget for 2027/28 – Balancing Rising Costs With the Local Services That Matter Most to You',
		shortDescription:
			'The council must make difficult choices about spending next year. Tell us which services you value most and where you think savings could be made.'
	},
	{
		title: '20mph Speed Limits on Residential Streets – Making Neighbourhoods Safer and More Pleasant for Walking, Cycling and Play',
		shortDescription:
			'Find out about proposals to lower speed limits on residential streets and tell us how they might affect your journeys, your street and your community.'
	},
	{
		title: 'Empty Homes Action Plan – Bringing Long-Term Empty Properties Back Into Use as Homes for Local People',
		shortDescription:
			'Tell us about empty homes in your area and what you think the council should do about them. Your views will help decide the support and powers we use.'
	}
];

// Enough extra to make 15 in all with the 7 above.
const GENERATED = Array.from({ length: 15 - SEEDS.length }, (_, i) => {
	const topic = MANY_TOPICS[i % MANY_TOPICS.length];
	return toConversation({
		id: `generated-${i + 1}`,
		...GENERATED_COPY[i % GENERATED_COPY.length],
		imageUrl: PHOTO_POOL[i % PHOTO_POOL.length],
		state: MANY_STATES[i % MANY_STATES.length],
		isFeatured: false,
		topic,
		organiser: ORGANISER_FOR_TOPIC[topic],
		estimatedMinutes: 5 + (i % 4) * 3,
		days: (i % 28) + 1,
		openedDaysAgo:
			MANY_STATES[i % MANY_STATES.length] === 'upcoming' ? -((i % 20) + 1) : (i * 3) % 90,
		participantCount: (i * 37) % 900,
		quotes: []
	});
});

/** Images the scenario panel can swap into the conversation page banner. */
export const BANNER_IMAGES: { id: string; label: string; url: string }[] = [
	{ id: 'cutout', label: 'Cut-out, space above', url: testCutoutImage as string },
	{ id: 'wide-portrait', label: 'Wide portrait', url: testWidePortraitImage as string },
	{ id: 'landscape', label: 'Landscape, half sky', url: testLandscapeImage as string },
	{ id: 'group', label: 'Group of people', url: testGroupImage as string },
	{ id: 'tall', label: 'Tall (2:3)', url: canal }
];

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
	reportReady: { conversationId: 'tram-housing' }
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
