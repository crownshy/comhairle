import { resolve } from '$app/paths';
import type { ConversationDto } from '@crownshy/api-client/api';
import { ADMIN_GUIDES } from '$lib/admin_guides';
import { toolMeta } from '$lib/tool_meta';
import {
	conversationSections,
	type ConversationSection,
	type HandbookTopicRef,
	type SectionStage
} from '$lib/config/conversation-steps';
import type { GuideLink, SectionHeaderContent } from '$lib/components/SectionHeader.svelte';

type HeaderConversation = Pick<ConversationDto, 'id' | 'isLive' | 'isComplete'>;

/** What the header needs to name a workflow step: its name and the tool behind it. */
export type HeaderStep = { id: string; name: string; toolType?: string };

export type ConversationHeader = {
	content: SectionHeaderContent;
	hasTabs: boolean;
	compact: boolean;
};

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

/** Launch and Close are milestones, not sections, so they keep their topics here. */
export const MILESTONE_GUIDES: Record<'launch' | 'close', HandbookTopicRef> = {
	launch: { guide: 'launch-a-conversation', topic: 'launch-a-conversation' },
	close: { guide: 'manage-a-conversation', topic: 'close-a-conversation' }
};

export function guideLink(ref: HandbookTopicRef | undefined): GuideLink {
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
 * The most specific topic for a page. A workflow step's own tab (`step/insights`) wins over
 * every step page (`step`), which wins over the whole section.
 */
function sectionGuide(section: ConversationSection, pageSegments: string[]): GuideLink {
	const [subSegment, , stepTab] = pageSegments;
	const tabs = section.guide?.tabs ?? {};
	const stepTabRef = subSegment === 'step' && stepTab ? tabs[`step/${stepTab}`] : undefined;
	return guideLink(
		stepTabRef ?? (subSegment ? tabs[subSegment] : undefined) ?? section.guide?.section
	);
}

function isEditorPage(segment: string, subSegment: string | undefined): boolean {
	return segment === 'events' && subSegment !== undefined && subSegment !== 'new';
}

/**
 * A workflow step names itself rather than the section: its tool, its name and a way back to
 * the board. Its own tabs are the only strip under it.
 */
function stepHeader(
	conversationId: string,
	step: HeaderStep | undefined,
	guide: GuideLink
): ConversationHeader {
	const tool = toolMeta(step?.toolType);
	return {
		content: {
			back: {
				href: resolve('/(admin)/admin/conversations/[conversation_id]/design', {
					conversation_id: conversationId
				}),
				label: 'Process design'
			},
			eyebrow: tool?.displayName ?? 'Step',
			title: step?.name || 'Unnamed step',
			description: tool?.tagline,
			guide
		},
		hasTabs: true,
		compact: false
	};
}

function milestoneHeader(
	milestone: 'launch' | 'close',
	conversation: HeaderConversation
): ConversationHeader {
	const guide = guideLink(MILESTONE_GUIDES[milestone]);
	const content: SectionHeaderContent =
		milestone === 'launch'
			? {
					title: conversation.isLive ? 'Launched' : 'Launch',
					description: conversation.isLive
						? 'The conversation is live and participants can take part.'
						: 'Check everything is ready, then make the conversation live for participants.',
					guide
				}
			: {
					title: conversation.isComplete ? 'Closed' : 'Close',
					description: conversation.isComplete
						? 'Participants can no longer take part in this conversation.'
						: 'Wrap up the conversation when participation is finished.',
					guide
				};
	return { content, hasTabs: false, compact: false };
}

/**
 * The header shown above a conversation section's tabs: the stage it belongs to, the
 * section's name, a one-line description and a link to the matching Handbook topic.
 * Returns null outside a conversation's admin pages.
 *
 * @param pathname - The route being shown (use the navigation target while switching so the
 *   header changes with the skeleton, not after the load resolves).
 * @param steps - The conversation's workflow steps, so a step page can show its own name.
 */
export function sectionHeader(
	pathname: string,
	conversation: HeaderConversation,
	steps: HeaderStep[] = []
): ConversationHeader | null {
	const base = `/admin/conversations/${conversation.id}`;
	const path = pathname.replace(/\/+$/, '');
	if (!path.startsWith(`${base}/`)) return null;

	const [segment, ...pageSegments] = path.slice(base.length + 1).split('/');
	if (segment === 'launch' || segment === 'close') return milestoneHeader(segment, conversation);

	const section = conversationSections.find((s) => s.path === segment);
	if (!section) return null;

	if (segment === 'design' && pageSegments[0] === 'step') {
		const step = steps.find((s) => s.id === pageSegments[1]);
		return stepHeader(conversation.id, step, sectionGuide(section, pageSegments));
	}

	return {
		content: {
			eyebrow: STAGE_LABELS[section.stage],
			title: section.name,
			description: section.description,
			guide: sectionGuide(section, pageSegments)
		},
		hasTabs: section.hasTabs ?? false,
		compact: isEditorPage(segment, pageSegments[0])
	};
}
