import SeparatorHorizontal from 'lucide-svelte/icons/separator-horizontal';
import {
	FORMATTING_TOOLS as TOOLS,
	type FormattingTool
} from '$lib/components/RichTextEditor/formattingTools';

export type Tool = FormattingTool;

export const HISTORY_TOOLS: Tool[] = [TOOLS.undo, TOOLS.redo];

/** Tools that work in the summary and the body: text styles and formatting. */
export const TEXT_TOOL_GROUPS: Tool[][] = [
	[TOOLS.paragraph, TOOLS.heading, TOOLS.subheading],
	[TOOLS.bold, TOOLS.italic, TOOLS.underline, TOOLS.strike],
	[TOOLS.bulletList, TOOLS.orderedList, TOOLS.blockquote],
	[TOOLS.alignLeft, TOOLS.alignCenter, TOOLS.alignRight, TOOLS.alignJustify]
];

/** A horizontal rule starts a new page, named by its first line (ADR-0069). */
export const PAGE_BREAK_TOOL: Tool = {
	label: 'Page break (a new page)',
	Icon: SeparatorHorizontal,
	isActive: () => false,
	run: (editor) => editor.chain().focus().setHorizontalRule().run()
};
