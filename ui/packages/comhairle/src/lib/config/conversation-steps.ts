import {
	SlidersHorizontal,
	GitBranch,
	BookOpen,
	CalendarDays,
	UsersRound,
	Bell,
	Binoculars,
	NotebookText,
	ShieldCheck,
	ChartColumn
} from 'lucide-svelte';
import type { ComponentType } from 'svelte';

/** The Conversation Stages that hold sections. Launch is a milestone with no sections. */
export type SectionStage = 'set-up' | 'engage' | 'results';

/** A topic in the admin Handbook (`ADMIN_GUIDES`). */
export type HandbookTopicRef = { guide: string; topic: string };

export interface ConversationSection {
	name: string;
	path: string;
	icon: ComponentType;
	stage: SectionStage;
	/** One line shown under the section's title in its header. */
	description: string;
	/** A tab strip sits under the section's header. */
	hasTabs?: boolean;
	/**
	 * The Handbook topic the section's header links to. `section` covers every page in the
	 * section; `tabs` overrides it for a page by the next route segment, e.g. `access`.
	 */
	guide?: { section?: HandbookTopicRef; tabs?: Record<string, HandbookTopicRef> };
	/**
	 * If true, the section is disabled until the conversation is launched. Not for
	 * edit protection, but because the section's content depends on
	 * participant activity that doesn't exist yet (recruiting, monitoring,
	 * moderation queues, notifications, reports). See ADR-0001.
	 */
	requiresLive?: boolean;
	/** Only conversation Admins can open it. */
	requiresAdmin?: boolean;
	/** Only listed when the conversation has a Polis step to moderate. */
	requiresPolis?: boolean;
}

export const conversationSections: ConversationSection[] = [
	{
		name: 'Configure',
		path: 'configure',
		icon: SlidersHorizontal,
		stage: 'set-up',
		description: "Set up the conversation's details, who can take part, and its content.",
		hasTabs: true,
		guide: {
			section: { guide: 'create-a-conversation', topic: 'name-a-conversation' },
			tabs: {
				content: { guide: 'create-a-conversation', topic: 'create-a-conversation' },
				'moderation-policy': { guide: 'advanced', topic: 'moderation-in-depth' },
				team: { guide: 'getting-started', topic: 'roles-and-permissions' }
			}
		}
	},
	{
		name: 'Process design',
		path: 'design',
		icon: GitBranch,
		stage: 'set-up',
		description: 'Design and configure your engagement, one step at a time.',
		hasTabs: true,
		guide: {
			section: { guide: 'design-the-engagement-process', topic: 'design-steps' },
			tabs: {
				step: { guide: 'design-the-engagement-process', topic: 'configure-a-step' }
			}
		}
	},
	{
		name: 'Learning Assistant',
		path: 'learning-assistant',
		icon: BookOpen,
		stage: 'set-up',
		description: 'Enable the Learning Assistant and manage the documents it answers from.',
		guide: {
			section: {
				guide: 'design-the-engagement-process',
				topic: 'set-up-the-learning-assistant'
			}
		}
	},
	{
		name: 'Events',
		path: 'events',
		icon: CalendarDays,
		stage: 'set-up',
		description: "Manage your conversation's events.",
		hasTabs: true,
		guide: { section: { guide: 'design-the-engagement-process', topic: 'run-events' } }
	},
	{
		name: 'Recruit',
		path: 'invites',
		icon: UsersRound,
		stage: 'engage',
		requiresLive: true,
		description: 'Bring people into the conversation by email, open link or in person.',
		hasTabs: true,
		guide: { section: { guide: 'manage-a-conversation', topic: 'recruit' } }
	},
	{
		name: 'Notify',
		path: 'notifications',
		icon: Bell,
		stage: 'engage',
		requiresLive: true,
		requiresAdmin: true,
		description:
			'Send a notification to everyone who has taken part in any step of this conversation.',
		guide: { section: { guide: 'manage-a-conversation', topic: 'notify' } }
	},
	{
		name: 'Moderate',
		path: 'moderate',
		icon: ShieldCheck,
		stage: 'engage',
		requiresLive: true,
		requiresPolis: true,
		description: 'Review and moderate what participants contribute.',
		guide: { section: { guide: 'manage-a-conversation', topic: 'moderate' } }
	},
	{
		name: 'Insights',
		path: 'insights',
		icon: ChartColumn,
		stage: 'engage',
		requiresLive: true,
		description: 'See the responses and results collected in each step.',
		guide: { section: { guide: 'manage-a-conversation', topic: 'see-insights' } }
	},
	{
		name: 'Monitor',
		path: 'monitor',
		icon: Binoculars,
		stage: 'engage',
		requiresLive: true,
		description: 'See how your conversation is working: recruitment, progress and reach.',
		guide: { section: { guide: 'manage-a-conversation', topic: 'monitor' } }
	},
	{
		name: 'Report',
		path: 'report',
		icon: NotebookText,
		stage: 'results',
		requiresLive: true,
		description: 'Write and edit the report for this conversation.',
		guide: { section: { guide: 'manage-a-conversation', topic: 'report' } }
	}
];
