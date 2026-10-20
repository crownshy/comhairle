import { resolve } from '$app/paths';
import { ADMIN_GUIDES } from '$lib/admin_guides';
import { conversationSections, type SectionStage } from '$lib/config/conversation-steps';
import type { GuideLink, SectionHeaderContent } from '$lib/components/SectionHeader.svelte';

type HeaderConversation = { id: string; isLive: boolean; isComplete: boolean };

type GuideRef = { guide: string; topic: string };

const STAGE_LABELS: Record<SectionStage, string> = {
	'set-up': 'Set up',
	engage: 'Engage',
	results: 'Results'
};

/** Shown when a page has no matching how-to topic yet. */
export const HANDBOOK_HOME: GuideLink = {
	href: resolve('/(admin)/admin/info/welcome'),
	label: 'The Comhairle Handbook'
};

function isEditorPage(segment: string, sub: string | undefined): boolean {
	if (segment === 'design') return sub === 'step';
	if (segment === 'events') return sub !== undefined && sub !== 'new';
	return false;
}

const SECTIONS_WITH_TABS = new Set(['configure', 'design', 'invites', 'events']);

/** Section copy, keyed by the route segment after /admin/conversations/<id>/. */
const SECTION_DESCRIPTIONS: Record<string, string> = {
	configure: "Set up the conversation's details, content and who can take part.",
	design: 'Design and configure your engagement, one step at a time.',
	'learning-assistant': 'Enable the Learning Assistant and manage the documents it answers from.',
	events: "Manage your conversation's events.",
	invites: 'Bring people into the conversation by email, open link or in person.',
	notifications: 'Send notifications to everyone who has taken part in this conversation.',
	moderate: 'Review and moderate what participants contribute.',
	monitor: 'See how your conversation is working: recruitment, progress and reach.',
	report: 'Write and edit the report for this conversation.'
};

/**
 * Which Handbook topic each page links to. Keys are the section segment, optionally
 * followed by the next segment (`configure/access`) for pages that need a more specific
 * topic. `design/step` covers every page under a workflow step. Pages with no entry fall
 * back to the section's entry, then to the Handbook home.
 */
export const PAGE_GUIDES: Record<string, GuideRef> = {
	configure: { guide: 'create-a-conversation', topic: 'name-a-conversation' },
	'configure/content': { guide: 'create-a-conversation', topic: 'create-a-conversation' },
	'configure/access': { guide: 'launch-a-conversation', topic: 'configure-access' },
	'configure/moderation-policy': { guide: 'advanced', topic: 'moderation-in-depth' },
	'configure/team': { guide: 'getting-started', topic: 'roles-and-permissions' },
	design: { guide: 'design-the-engagement-process', topic: 'design-steps' },
	'design/step': { guide: 'design-the-engagement-process', topic: 'configure-a-step' },
	launch: { guide: 'launch-a-conversation', topic: 'launch-a-conversation' },
	invites: { guide: 'manage-a-conversation', topic: 'recruit' },
	notifications: { guide: 'manage-a-conversation', topic: 'notify' },
	moderate: { guide: 'manage-a-conversation', topic: 'moderate' },
	monitor: { guide: 'manage-a-conversation', topic: 'monitor' },
	report: { guide: 'manage-a-conversation', topic: 'report' }
};

function guideLink(ref: GuideRef | undefined): GuideLink {
	const topic = ref && ADMIN_GUIDES[ref.guide]?.topics.find((t) => t.key === ref.topic);
	if (!ref || !topic) return HANDBOOK_HOME;
	return {
		href: resolve('/(admin)/admin/info/how-to/[guide_id]/[topic_id]', {
			guide_id: ref.guide,
			topic_id: ref.topic
		}),
		label: topic.title
	};
}

/**
 * The header shown above a conversation section's tabs: the stage it belongs to, the
 * section's name, a one-line description and a link to the matching Handbook topic.
 * Returns null outside a conversation's admin pages.
 *
 * @param pathname - The route being shown (use the navigation target while switching so the
 *   header changes with the skeleton, not after the load resolves).
 */
export function sectionHeader(
	pathname: string,
	conversation: HeaderConversation
): SectionHeaderContent | null {
	const base = `/admin/conversations/${conversation.id}`;
	const path = pathname.replace(/\/+$/, '');
	if (!path.startsWith(`${base}/`)) return null;

	const [segment, sub] = path.slice(base.length + 1).split('/');
	const guide = guideLink(PAGE_GUIDES[`${segment}/${sub}`] ?? PAGE_GUIDES[segment]);

	// Launch and Close are milestones, not sections, so they have no stage label.
	if (segment === 'launch') {
		return {
			title: conversation.isLive ? 'Launched' : 'Launch',
			description: conversation.isLive
				? undefined
				: 'Check everything is ready, then make the conversation live for participants.',
			guide,
			hasTabs: false,
			compact: false
		};
	}
	if (segment === 'close') {
		return {
			title: conversation.isComplete ? 'Closed' : 'Close',
			description: conversation.isComplete
				? 'Participants can no longer take part in this conversation.'
				: 'Wrap up the conversation when participation is finished.',
			guide,
			hasTabs: false,
			compact: false
		};
	}

	const section = conversationSections.find((s) => s.path === segment);
	if (!section) return null;

	return {
		eyebrow: STAGE_LABELS[section.stage],
		title: section.name,
		description: SECTION_DESCRIPTIONS[segment],
		guide,
		hasTabs: SECTIONS_WITH_TABS.has(segment),
		compact: isEditorPage(segment, sub)
	};
}
