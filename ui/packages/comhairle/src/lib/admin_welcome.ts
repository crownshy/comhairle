/**
 * Content for the admin guide's welcome page (/admin/info/welcome): who CrownShy are, what
 * Comhairle is, what an admin does, and where to start depending on your role.
 * Text uses the same formatting as admin_guides.ts: **bold**, [links](/path) and :icons:.
 */

const HOW_TO = '/admin/info/how-to';

export type WelcomeIcon = 'create' | 'design' | 'launch' | 'run' | 'moderate' | 'report';

export type WelcomeResponsibility = {
	title: string;
	text: string;
	icon: WelcomeIcon;
	/** Where the card leads: the first topic of the matching tutorial. */
	href: string;
	linkLabel: string;
};

export type WelcomePath = {
	/** Written as the reader would say it, e.g. "I'm setting up a conversation". */
	title: string;
	text: string;
	links: { label: string; href: string }[];
};

export const ADMIN_WELCOME = {
	title: 'Welcome to Comhairle',
	summary:
		'Start here if you’re new to running conversations on Comhairle. This page explains who we are, what the platform does, and where to go next.',

	crownshy: {
		heading: 'Who are CrownShy?',
		text: 'CrownShy builds tools that help democratic societies listen, reason and act together. Started in 2024, our team brings together experience in facilitation, civic technology, design, artificial intelligence and participatory practice.'
	},

	comhairle: {
		heading: 'What is Comhairle?',
		paragraphs: [
			'Comhairle (pronounced “kuh-ur-lhya”) is CrownShy’s platform for public participation. It makes it easier for people to take part in the decisions that affect them, and for organisations to run engaging, accessible consultations.',
			'Each consultation is a [conversation](/admin/info/glossary#term-conversation): a sequence of [steps](/admin/info/glossary#term-step) that participants go through. Each step uses an [engagement tool](/admin/info/glossary#term-engagement-tool) suited to a different kind of input, and what people say in one step can inform the next. Comhairle supports facilitators rather than replacing them.'
		],
		toolsHeading: 'The engagement tools you can use'
	},

	responsibilities: {
		heading: 'What you’ll do as an admin',
		intro: 'A conversation goes through these stages. Each card leads to the tutorials for that stage.',
		items: [
			{
				title: 'Create a conversation',
				text: 'Set up the topic, name, description, languages and banner image.',
				icon: 'create',
				href: `${HOW_TO}/create-a-conversation/create-a-conversation`,
				linkLabel: 'Create a conversation'
			},
			{
				title: 'Design it with colleagues',
				text: 'Choose engagement tools, arrange the steps, and invite colleagues to help edit.',
				icon: 'design',
				href: `${HOW_TO}/design-the-engagement-process/choose-a-template`,
				linkLabel: 'Design the engagement process'
			},
			{
				title: 'Launch it',
				text: 'Check everything in preview, decide who can take part, then go live.',
				icon: 'launch',
				href: `${HOW_TO}/launch-a-conversation/preview-before-launch`,
				linkLabel: 'Launch a conversation'
			},
			{
				title: 'Run it while it’s live',
				text: 'Invite participants, follow how many take part, and send them updates.',
				icon: 'run',
				href: `${HOW_TO}/manage-a-conversation/recruit`,
				linkLabel: 'Manage a conversation'
			},
			{
				title: 'Moderate contributions',
				text: 'Review statements as they come in, and keep the conversation on topic and safe.',
				icon: 'moderate',
				href: `${HOW_TO}/manage-a-conversation/moderate`,
				linkLabel: 'Moderate contributions'
			},
			{
				title: 'Report on the outcome',
				text: 'When the conversation ends, summarise what came out of it and share the results.',
				icon: 'report',
				href: `${HOW_TO}/manage-a-conversation/report`,
				linkLabel: 'Create a report'
			}
		] satisfies WelcomeResponsibility[]
	},

	paths: {
		heading: 'Where to start',
		intro: 'Pick the one that sounds most like you.',
		items: [
			{
				title: 'I’m setting up a new conversation',
				text: 'You’ll be the conversation’s owner: you can change everything and decide who else has access.',
				links: [
					{
						label: 'Sign up for an admin account',
						href: `${HOW_TO}/getting-started/sign-up`
					},
					{
						label: 'Create a conversation',
						href: `${HOW_TO}/create-a-conversation/create-a-conversation`
					},
					{
						label: 'Choose a template',
						href: `${HOW_TO}/design-the-engagement-process/choose-a-template`
					},
					{
						label: 'Launch a conversation',
						href: `${HOW_TO}/launch-a-conversation/launch-a-conversation`
					}
				]
			},
			{
				title: 'I’ve been added to help edit a conversation',
				text: 'A colleague owns the conversation and has given you content editor access, so it appears under **Permitted Conversations**.',
				links: [
					{
						label: 'Your role and permissions',
						href: `${HOW_TO}/getting-started/roles-and-permissions`
					},
					{
						label: 'Configure a step',
						href: `${HOW_TO}/design-the-engagement-process/configure-a-step`
					},
					{ label: 'Preview your page', href: `${HOW_TO}/create-a-conversation/preview` }
				]
			},
			{
				title: 'I’m moderating a live conversation',
				text: 'You’ll review what participants write and decide what others get to see and vote on.',
				links: [
					{
						label: 'Moderate contributions',
						href: `${HOW_TO}/manage-a-conversation/moderate`
					},
					{
						label: 'Moderation in depth',
						href: `${HOW_TO}/advanced/moderation-in-depth`
					},
					{ label: 'Glossary', href: '/admin/info/glossary' }
				]
			}
		] satisfies WelcomePath[]
	}
};
