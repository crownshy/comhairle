import {
	BookOpenText,
	ClipboardList,
	Lightbulb,
	ListOrdered,
	MessagesSquare,
	Mic,
	Video
} from 'lucide-svelte';
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
