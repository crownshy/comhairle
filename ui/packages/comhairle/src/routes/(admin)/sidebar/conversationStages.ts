import type { ComponentType } from 'svelte';
import { resolve } from '$app/paths';
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
	const base = resolve('/(admin)/admin/conversations/[conversation_id]', {
		conversation_id: conversation.id
	});
	const live = conversation.isLive;
	const complete = conversation.isComplete;
	const sectionsIn = (stage: SectionStage) =>
		conversationSections
			.filter((section) => section.stage === stage)
			.filter((section) => !section.requiresPolis || hasPolisStep)
			.map((section) =>
				sectionLink(section, base, pathname, sectionLock(section, live, canAdmin))
			);
	const milestone = (target: LinkTarget, lock: SectionLock | null) =>
		sectionLink(target, base, pathname, lock);

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
			link: milestone(
				{ name: live ? 'Launched' : 'Launch', path: 'launch', icon: Rocket },
				null
			)
		},
		{
			id: 'engage',
			label: 'Engage',
			state: afterLaunchState(live, complete),
			sections: sectionsIn('engage')
		},
		{
			id: 'close',
			label: complete ? 'Closed' : 'Close',
			state: afterLaunchState(live, complete),
			sections: [],
			link: milestone(
				{ name: complete ? 'Closed' : 'Close', path: 'close', icon: Flag },
				live ? null : 'until-launch'
			)
		},
		{
			id: 'results',
			label: 'Results',
			state: live ? 'current' : 'locked',
			sections: sectionsIn('results')
		}
	];
}

/** Engage and Close: locked before launch, current while live, done once closed. */
function afterLaunchState(isLive: boolean, isComplete: boolean): StageState {
	if (!isLive) return 'locked';
	if (isComplete) return 'done';
	return 'current';
}

type LinkTarget = Pick<ConversationSection, 'name' | 'path' | 'icon'>;

function sectionLink(
	target: LinkTarget,
	base: string,
	pathname: string,
	lock: SectionLock | null
): SectionLink {
	const href = `${base}/${target.path}`;
	return {
		name: target.name,
		path: target.path,
		href,
		icon: target.icon,
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
