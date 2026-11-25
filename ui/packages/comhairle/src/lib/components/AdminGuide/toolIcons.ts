import BookOpenText from '@lucide/svelte/icons/book-open-text';
import ClipboardList from '@lucide/svelte/icons/clipboard-list';
import Lightbulb from '@lucide/svelte/icons/lightbulb';
import ListOrdered from '@lucide/svelte/icons/list-ordered';
import MessagesSquare from '@lucide/svelte/icons/messages-square';
import Mic from '@lucide/svelte/icons/mic';
import Video from '@lucide/svelte/icons/video';
import { TOOL_OVERVIEW, type ToolIcon } from '$lib/tool_overview';

/** One icon per engagement tool, shared by the rail and the tools overview page. */
export const TOOL_ICONS: Record<ToolIcon, typeof Video> = {
	poll: MessagesSquare,
	reflect: Lightbulb,
	learn: BookOpenText,
	video: Video,
	survey: ClipboardList,
	prioritise: ListOrdered,
	voice: Mic
};

/** The icon for a tool by its key in TOOL_GUIDES (falls back to the Learn icon). */
export function toolIcon(key: string) {
	return TOOL_ICONS[TOOL_OVERVIEW[key]?.icon ?? 'learn'];
}
