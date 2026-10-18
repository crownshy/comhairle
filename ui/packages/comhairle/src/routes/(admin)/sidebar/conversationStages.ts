import type { ComponentType } from 'svelte';
import { Flag, Rocket } from 'lucide-svelte';
import type { ConversationDto } from '@crownshy/api-client/api';
import {
	conversationSections,
	type ConversationSection,
	type SectionStage
} from '$lib/config/conversation-steps';

export type StageState = 'done' | 'current' | 'locked';

export type SectionLock = 'until-launch' | 'admin-only';

export const sectionLockLabels: Record<SectionLock, string> = {
	'until-launch': 'Available after launch',
	'admin-only': 'Admins only'
};

export type SectionLink = {
	name: string;
	path: string;
	href: string;
	icon: ComponentType;
	active: boolean;
	/** Why the link can't be opened, or null when it can. */
	lock: SectionLock | null;
};

export type ConversationStage = {
	id: SectionStage | 'launch' | 'close';
	label: string;
	state: StageState;
	sections: SectionLink[];
	/** A milestone stage (Launch, Close) is one page, so the stage itself is the link. */
	link?: SectionLink;
};

export type StageConversation = Pick<ConversationDto, 'id' | 'isLive' | 'isComplete'>;

type StageInput = {
	conversation: StageConversation;
	pathname: string;
	canAdmin: boolean;
	/** Adds Moderate to Engage. */
	hasPolisStep?: boolean;
};

export function conversationStages({
	conversation,
	pathname,
	canAdmin,
	hasPolisStep = false
}: StageInput): ConversationStage[] {
	const base = `/admin/conversations/${conversation.id}`;
	const sectionsIn = (stage: SectionStage) =>
		conversationSections
			.filter((section) => section.stage === stage)
			.filter((section) => !section.requiresPolis || hasPolisStep)
			.map((section) => sectionLink(section, base, pathname, conversation.isLive, canAdmin));
	const live = conversation.isLive;

	const complete = conversation.isComplete;
	const milestone = (name: string, path: string, icon: ComponentType, lock: SectionLock | null) =>
		sectionLink({ name, path, icon, stage: 'set-up' }, base, pathname, live, canAdmin, lock);

	return [
		{
			id: 'set-up',
			label: 'Set up',
			state: live ? 'done' : 'current',
			sections: sectionsIn('set-up')
		},
		{
			id: 'launch',
			label: live ? 'Launched' : 'Launch',
			state: live ? 'done' : 'current',
			sections: [],
			link: milestone(live ? 'Launched' : 'Launch', 'launch', Rocket, null)
		},
		{
			id: 'engage',
			label: 'Engage',
			state: !live ? 'locked' : complete ? 'done' : 'current',
			sections: sectionsIn('engage')
		},
		{
			id: 'close',
			label: complete ? 'Closed' : 'Close',
			state: !live ? 'locked' : complete ? 'done' : 'current',
			sections: [],
			link: milestone(
				complete ? 'Closed' : 'Close',
				'close',
				Flag,
				live ? null : 'until-launch'
			)
		},
		{
			id: 'results',
			label: 'Results',
			state: !live ? 'locked' : 'current',
			sections: sectionsIn('results')
		}
	];
}

function sectionLink(
	section: ConversationSection,
	base: string,
	pathname: string,
	isLive: boolean,
	canAdmin: boolean,
	lock: SectionLock | null = sectionLock(section, isLive, canAdmin)
): SectionLink {
	const href = `${base}/${section.path}`;
	return {
		name: section.name,
		path: section.path,
		href,
		icon: section.icon,
		active: pathname === href || pathname.startsWith(`${href}/`),
		lock
	};
}

function sectionLock(
	section: ConversationSection,
	isLive: boolean,
	canAdmin: boolean
): SectionLock | null {
	if (section.requiresLive && !isLive) return 'until-launch';
	if (section.requiresAdmin && !canAdmin) return 'admin-only';
	return null;
}
