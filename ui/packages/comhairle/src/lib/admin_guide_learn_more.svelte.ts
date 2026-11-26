/**
 * A glossary tooltip's "learn more" link opens that handbook page's content in a dialog
 * over the current page (GuideLearnMoreDialog.svelte), so the reader keeps their place.
 */
import { ADMIN_GUIDES, type AdminGuide, type AdminGuideTopic } from './admin_guides';
import { TOOL_GUIDES, type ToolGuide } from './tool_guides';

/** What a learnMore href shows in the dialog. */
export type LearnMoreTarget =
	| { kind: 'topic'; href: string; guide: AdminGuide; topic: AdminGuideTopic }
	| { kind: 'tool'; href: string; guide: ToolGuide };

/**
 * The topic or tool a learnMore href points at, or undefined for anything else (such as
 * the tools overview), which then opens as an ordinary link.
 */
export function resolveLearnMore(href: string): LearnMoreTarget | undefined {
	const topicMatch = href.match(/^\/admin\/info\/how-to\/([^/]+)\/([^/]+)$/);
	if (topicMatch && Object.hasOwn(ADMIN_GUIDES, topicMatch[1])) {
		const guide = ADMIN_GUIDES[topicMatch[1]];
		const topic = guide.topics.find((t) => t.key === topicMatch[2]);
		return topic ? { kind: 'topic', href, guide, topic } : undefined;
	}
	const toolMatch = href.match(/^\/admin\/info\/tools\/([^/]+)$/);
	if (toolMatch && Object.hasOwn(TOOL_GUIDES, toolMatch[1])) {
		return { kind: 'tool', href, guide: TOOL_GUIDES[toolMatch[1]] };
	}
	return undefined;
}

class LearnMoreDialog {
	open = $state(false);
	/** Kept after closing so the content doesn't vanish mid fade-out. */
	current = $state<LearnMoreTarget>();

	show(target: LearnMoreTarget) {
		this.current = target;
		this.open = true;
	}
}

export const learnMore = new LearnMoreDialog();
