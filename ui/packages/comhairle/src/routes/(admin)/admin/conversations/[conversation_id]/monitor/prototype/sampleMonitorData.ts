/**
 * Sample data for the Monitor prototype route. Nothing here comes from the API yet:
 * the shapes describe the data each new section would need, so they double as a
 * wish list for the backend (see the notes on each type).
 */

/** Already available today via `WorkflowStepStats` (started/completed); timing is new. */
export type StepProgress = {
	name: string;
	/** Short label for tight spaces such as the At a glance bar chart. */
	shortName: string;
	reached: number;
	finished: number;
	/** New: median minutes spent on the step by people who finished it. */
	medianMinutes: number;
	/** New: started the step but no activity for 3+ days. */
	inactiveThreeDays: number;
};

/** Running total of signups per day. Derivable from today's `DailySignupStats`. */
export type SignupPoint = { day: Date; total: number };

/** New: needs the signup source (invite, open link, QR) stored per participant. */
export type RecruitmentChannel = {
	type: 'Email' | 'Open link' | 'Physical';
	name: string;
	joined: number;
	conversion: string | null;
};

/** New: needs a population baseline per demographic group, set by the admin. */
export type RepresentationRow = {
	group: string;
	participantPercent: number;
	populationPercent: number;
};

export type ActivityItem = { text: string; when: string; kind: 'progress' | 'neutral' | 'drop' };

export const SAMPLE_DAY_OF_CONVERSATION = 14;
export const SAMPLE_CONVERSATION_LENGTH_DAYS = 28;
export const SAMPLE_JOINED = 86;
export const SAMPLE_FINISHED = 41;
export const SAMPLE_TARGET = 200;
export const SAMPLE_CLOSING_DATE = new Date('2026-10-26');
export const SAMPLE_PROJECTED_AT_CLOSE = 155;
export const SAMPLE_ACTIVE_NOW = 3;
export const USED_THINKING_SPACE = 15;

export const SAMPLE_STEPS: StepProgress[] = [
	{
		name: 'Learn: air quality basics',
		shortName: 'Learn',
		reached: 86,
		finished: 80,
		medianMinutes: 5,
		inactiveThreeDays: 2
	},
	{
		name: 'Survey',
		shortName: 'Survey',
		reached: 80,
		finished: 74,
		medianMinutes: 4,
		inactiveThreeDays: 1
	},
	{
		name: 'Thinking space',
		shortName: 'Thinking',
		reached: 74,
		finished: 52,
		medianMinutes: 14,
		inactiveThreeDays: 15
	},
	{
		name: 'Participant-led poll',
		shortName: 'Poll',
		reached: 52,
		finished: 46,
		medianMinutes: 6,
		inactiveThreeDays: 3
	},
	{
		name: 'Wrap-up',
		shortName: 'Wrap-up',
		reached: 46,
		finished: 41,
		medianMinutes: 3,
		inactiveThreeDays: 2
	}
];

const FIRST_DAY = new Date('2026-09-29');
export const SAMPLE_LAUNCH_DATE = FIRST_DAY;
const RUNNING_TOTALS = [3, 10, 18, 22, 25, 27, 29, 45, 52, 57, 60, 70, 78, 86];

function dayOffset(offset: number): Date {
	const day = new Date(FIRST_DAY);
	day.setDate(day.getDate() + offset);
	return day;
}

export const SAMPLE_SIGNUPS: SignupPoint[] = RUNNING_TOTALS.map((total, index) => ({
	day: dayOffset(index),
	total
}));

/** Running total of people who finished every step, one point per day. */
const COMPLETED_TOTALS = [0, 1, 4, 7, 9, 11, 13, 18, 22, 26, 29, 33, 37, 41];

export const SAMPLE_COMPLETIONS: SignupPoint[] = COMPLETED_TOTALS.map((total, index) => ({
	day: dayOffset(index),
	total
}));

/** Straight line from zero on day one to the target on the closing day. */
export const SAMPLE_TARGET_PACE: SignupPoint[] = [
	{ day: dayOffset(-1), total: 0 },
	{ day: SAMPLE_CLOSING_DATE, total: SAMPLE_TARGET }
];

export const SAMPLE_PROJECTION: SignupPoint[] = [
	SAMPLE_SIGNUPS[SAMPLE_SIGNUPS.length - 1],
	{ day: SAMPLE_CLOSING_DATE, total: SAMPLE_PROJECTED_AT_CLOSE }
];

export const SAMPLE_CHANNELS: RecruitmentChannel[] = [
	{ type: 'Email', name: 'Invite list (240 sent)', joined: 38, conversion: '16% of invited' },
	{ type: 'Open link', name: 'Council newsletter', joined: 21, conversion: null },
	{
		type: 'Physical',
		name: 'Community centre event QR',
		joined: 18,
		conversion: '58% of 31 scans'
	},
	{ type: 'Open link', name: 'Social media post', joined: 9, conversion: null }
];

export const SAMPLE_AGE_REPRESENTATION: RepresentationRow[] = [
	{ group: '16–24', participantPercent: 4, populationPercent: 13 },
	{ group: '25–34', participantPercent: 19, populationPercent: 17 },
	{ group: '35–44', participantPercent: 24, populationPercent: 16 },
	{ group: '45–54', participantPercent: 22, populationPercent: 17 },
	{ group: '55–64', participantPercent: 21, populationPercent: 16 },
	{ group: '65+', participantPercent: 10, populationPercent: 21 }
];

/** One demographic the admin can switch to in the Reach views. */
export type RepresentationDimension = {
	name: string;
	/** Plural noun for the groups, e.g. "age groups". */
	groupNoun: string;
	/** How many participants answered this question. */
	shared: number;
	rows: RepresentationRow[];
};

/** A group counts as under-represented below 70% of its population share. */
export const UNDER_REPRESENTED_RATIO = 0.7;

export function isUnderRepresented(row: RepresentationRow): boolean {
	return row.participantPercent < row.populationPercent * UNDER_REPRESENTED_RATIO;
}

export const SAMPLE_REPRESENTATION: RepresentationDimension[] = [
	{ name: 'Age', groupNoun: 'age groups', shared: 71, rows: SAMPLE_AGE_REPRESENTATION },
	{
		name: 'Gender',
		groupNoun: 'genders',
		shared: 74,
		rows: [
			{ group: 'Women', participantPercent: 58, populationPercent: 51 },
			{ group: 'Men', participantPercent: 39, populationPercent: 48 },
			{ group: 'Another identity', participantPercent: 3, populationPercent: 1 }
		]
	},
	{
		name: 'Area',
		groupNoun: 'areas',
		shared: 80,
		rows: [
			{ group: 'Northfield', participantPercent: 34, populationPercent: 22 },
			{ group: 'Riverside', participantPercent: 28, populationPercent: 20 },
			{ group: 'Old Town', participantPercent: 18, populationPercent: 18 },
			{ group: 'Eastgate', participantPercent: 12, populationPercent: 21 },
			{ group: 'Hillcrest', participantPercent: 8, populationPercent: 19 }
		]
	},
	{
		name: 'Ethnicity',
		groupNoun: 'ethnic groups',
		shared: 62,
		rows: [
			{ group: 'White', participantPercent: 86, populationPercent: 79 },
			{ group: 'Asian', participantPercent: 6, populationPercent: 9 },
			{ group: 'Black', participantPercent: 2, populationPercent: 5 },
			{ group: 'Mixed', participantPercent: 4, populationPercent: 4 },
			{ group: 'Other', participantPercent: 2, populationPercent: 3 }
		]
	},
	{
		name: 'Disability',
		groupNoun: 'groups',
		shared: 58,
		rows: [
			{ group: 'Disabled', participantPercent: 11, populationPercent: 22 },
			{ group: 'Not disabled', participantPercent: 89, populationPercent: 78 }
		]
	}
];

export const SAMPLE_ACTIVITY: ActivityItem[] = [
	{ text: 'Someone completed the whole conversation', when: '2 min ago', kind: 'progress' },
	{
		text: '3 people joined via the Council newsletter link',
		when: '25 min ago',
		kind: 'progress'
	},
	{ text: 'New statement added in Participant-led poll', when: '40 min ago', kind: 'neutral' },
	{ text: 'Someone left Thinking space after 2 minutes', when: '1 h ago', kind: 'drop' },
	{ text: 'Reminder email sent to 31 people', when: 'Yesterday', kind: 'neutral' },
	{ text: '11 people joined from the community centre QR code', when: '6 Oct', kind: 'progress' }
];

export const HEATMAP_DAYS = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'];
export const HEATMAP_SLOTS = ['6–9am', '9–12', '12–3pm', '3–6pm', '6–9pm', '9–12pm'];
/** New: actions taken (step starts, answers, comments) per weekday and time slot. */
export const SAMPLE_HEATMAP: number[][] = [
	[2, 6, 9, 5, 14, 4],
	[1, 5, 8, 7, 31, 9],
	[3, 7, 6, 6, 27, 8],
	[2, 4, 10, 5, 12, 3],
	[1, 6, 7, 4, 6, 2],
	[0, 9, 12, 8, 5, 1],
	[0, 5, 14, 9, 7, 2]
];

export const SAMPLE_MEDIAN_MINUTES_TO_FINISH = 7;

/**
 * New: one participant's activity today, step by step. Needs per-step start and end
 * timestamps for each participant. Participants are shown by an anonymous code only.
 */
export type JourneyStepVisit = {
	/** Minutes spent on the step today. 0 when not reached. */
	minutes: number;
	status: 'done' | 'in_progress' | 'left' | 'not_reached';
	/** Clock time the step was opened, e.g. "09:14". */
	startedAt?: string;
};

export type ParticipantJourney = {
	code: string;
	/** One entry per step, in the same order as SAMPLE_STEPS. */
	visits: JourneyStepVisit[];
};

export const SAMPLE_TODAY_JOURNEYS: ParticipantJourney[] = [
	{
		code: 'P-0371',
		visits: [
			{ minutes: 4, status: 'done', startedAt: '07:48' },
			{ minutes: 2, status: 'done', startedAt: '07:52' },
			{ minutes: 7, status: 'done', startedAt: '07:54' },
			{ minutes: 3, status: 'done', startedAt: '08:01' },
			{ minutes: 2, status: 'done', startedAt: '08:04' }
		]
	},
	{
		code: 'P-0377',
		visits: [
			{ minutes: 2, status: 'done', startedAt: '08:15' },
			{ minutes: 2, status: 'done', startedAt: '08:17' },
			{ minutes: 1, status: 'left', startedAt: '08:19' },
			{ minutes: 0, status: 'not_reached' },
			{ minutes: 0, status: 'not_reached' }
		]
	},
	{
		code: 'P-0380',
		visits: [
			{ minutes: 3, status: 'done', startedAt: '08:40' },
			{ minutes: 3, status: 'done', startedAt: '08:43' },
			{ minutes: 10, status: 'done', startedAt: '08:46' },
			{ minutes: 2, status: 'left', startedAt: '08:56' },
			{ minutes: 0, status: 'not_reached' }
		]
	},
	{
		code: 'P-0402',
		visits: [
			{ minutes: 5, status: 'done', startedAt: '10:10' },
			{ minutes: 2, status: 'done', startedAt: '10:15' },
			{ minutes: 6, status: 'done', startedAt: '10:17' },
			{ minutes: 3, status: 'done', startedAt: '10:23' },
			{ minutes: 2, status: 'done', startedAt: '10:26' }
		]
	},
	{
		code: 'P-0412',
		visits: [
			{ minutes: 3, status: 'done', startedAt: '09:14' },
			{ minutes: 2, status: 'done', startedAt: '09:17' },
			{ minutes: 8, status: 'done', startedAt: '09:19' },
			{ minutes: 4, status: 'done', startedAt: '09:27' },
			{ minutes: 2, status: 'done', startedAt: '09:31' }
		]
	},
	{
		code: 'P-0388',
		visits: [
			{ minutes: 3, status: 'done', startedAt: '11:02' },
			{ minutes: 3, status: 'done', startedAt: '11:05' },
			{ minutes: 1, status: 'left', startedAt: '11:08' },
			{ minutes: 0, status: 'not_reached' },
			{ minutes: 0, status: 'not_reached' }
		]
	},
	{
		code: 'P-0457',
		visits: [
			{ minutes: 2, status: 'done', startedAt: '13:40' },
			{ minutes: 2, status: 'done', startedAt: '13:42' },
			{ minutes: 9, status: 'done', startedAt: '13:44' },
			{ minutes: 3, status: 'in_progress', startedAt: '13:53' },
			{ minutes: 0, status: 'not_reached' }
		]
	},
	{
		code: 'P-0461',
		visits: [
			{ minutes: 4, status: 'done', startedAt: '15:31' },
			{ minutes: 2, status: 'done', startedAt: '15:35' },
			{ minutes: 6, status: 'in_progress', startedAt: '15:37' },
			{ minutes: 0, status: 'not_reached' },
			{ minutes: 0, status: 'not_reached' }
		]
	},
	{
		code: 'P-0399',
		visits: [
			{ minutes: 2, status: 'done', startedAt: '16:05' },
			{ minutes: 1, status: 'left', startedAt: '16:07' },
			{ minutes: 0, status: 'not_reached' },
			{ minutes: 0, status: 'not_reached' },
			{ minutes: 0, status: 'not_reached' }
		]
	},
	{
		code: 'P-0466',
		visits: [
			{ minutes: 3, status: 'done', startedAt: '16:20' },
			{ minutes: 2, status: 'done', startedAt: '16:23' },
			{ minutes: 8, status: 'done', startedAt: '16:25' },
			{ minutes: 4, status: 'done', startedAt: '16:33' },
			{ minutes: 1, status: 'in_progress', startedAt: '16:37' }
		]
	}
];

/** Kinds of admin activity logged on the At a glance calendar. */
export type ActivityKind = 'launch' | 'email' | 'qr' | 'event' | 'reminder' | 'close';

/**
 * New: a log of what the team did and when (launch, invites, QR codes, events,
 * reminders, closing). Most of these already happen in Recruit and Notify; they'd need
 * to be recorded with a date so they can be shown here.
 */
export type ActivityLogEntry = {
	date: Date;
	kind: ActivityKind;
	title: string;
	detail?: string;
	/** True for things planned but not happened yet. */
	scheduled?: boolean;
};

export const SAMPLE_TODAY = new Date('2026-10-12');

export const SAMPLE_ACTIVITY_LOG: ActivityLogEntry[] = [
	{ date: new Date('2026-09-29'), kind: 'launch', title: 'Conversation launched' },
	{
		date: new Date('2026-09-30'),
		kind: 'email',
		title: 'Email invite sent',
		detail: 'To 240 people on the invite list'
	},
	{
		date: new Date('2026-10-05'),
		kind: 'qr',
		title: 'QR codes generated',
		detail: 'For the community centre event'
	},
	{
		date: new Date('2026-10-06'),
		kind: 'event',
		title: 'Community event',
		detail: '31 QR scans, 18 people joined'
	},
	{
		date: new Date('2026-10-10'),
		kind: 'reminder',
		title: 'Reminder sent',
		detail: 'To 31 people who had not completed'
	},
	{
		date: new Date('2026-10-20'),
		kind: 'reminder',
		title: 'Reminder scheduled',
		detail: 'To everyone still in progress',
		scheduled: true
	},
	{ date: new Date('2026-10-26'), kind: 'close', title: 'Conversation closes', scheduled: true }
];
