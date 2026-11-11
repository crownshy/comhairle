import type { Editor } from '@tiptap/core';
import type { ComponentType } from 'svelte';
import type { Icon as LucideIcon } from 'lucide-svelte';
import Pilcrow from 'lucide-svelte/icons/pilcrow';
import Heading2 from 'lucide-svelte/icons/heading-2';
import Heading3 from 'lucide-svelte/icons/heading-3';
import Bold from 'lucide-svelte/icons/bold';
import Italic from 'lucide-svelte/icons/italic';
import Underline from 'lucide-svelte/icons/underline';
import Strikethrough from 'lucide-svelte/icons/strikethrough';
import List from 'lucide-svelte/icons/list';
import ListOrdered from 'lucide-svelte/icons/list-ordered';
import Quote from 'lucide-svelte/icons/quote';
import AlignLeft from 'lucide-svelte/icons/align-left';
import AlignCenter from 'lucide-svelte/icons/align-center';
import AlignRight from 'lucide-svelte/icons/align-right';
import AlignJustify from 'lucide-svelte/icons/align-justify';
import SeparatorHorizontal from 'lucide-svelte/icons/separator-horizontal';
import Undo2 from 'lucide-svelte/icons/undo-2';
import Redo2 from 'lucide-svelte/icons/redo-2';

export type Tool = {
	label: string;
	Icon: ComponentType<LucideIcon>;
	isActive: (editor: Editor) => boolean;
	run: (editor: Editor) => void;
	/** Greys the tool out while it has nothing to act on. */
	isDisabled?: (editor: Editor) => boolean;
};

export const HISTORY_TOOLS: Tool[] = [
	{
		label: 'Undo',
		Icon: Undo2,
		isActive: () => false,
		isDisabled: (e) => !e.can().undo(),
		run: (e) => e.chain().focus().undo().run()
	},
	{
		label: 'Redo',
		Icon: Redo2,
		isActive: () => false,
		isDisabled: (e) => !e.can().redo(),
		run: (e) => e.chain().focus().redo().run()
	}
];

/** Tools that work in the summary and the body: text styles and formatting. */
export const TEXT_TOOL_GROUPS: Tool[][] = [
	[
		{
			label: 'Text',
			Icon: Pilcrow,
			isActive: (e) => e.isActive('paragraph'),
			run: (e) => e.chain().focus().setParagraph().run()
		},
		{
			label: 'Heading',
			Icon: Heading2,
			isActive: (e) => e.isActive('heading', { level: 2 }),
			run: (e) => e.chain().focus().toggleHeading({ level: 2 }).run()
		},
		{
			label: 'Subheading',
			Icon: Heading3,
			isActive: (e) => e.isActive('heading', { level: 3 }),
			run: (e) => e.chain().focus().toggleHeading({ level: 3 }).run()
		}
	],
	[
		{
			label: 'Bold',
			Icon: Bold,
			isActive: (e) => e.isActive('bold'),
			run: (e) => e.chain().focus().toggleBold().run()
		},
		{
			label: 'Italic',
			Icon: Italic,
			isActive: (e) => e.isActive('italic'),
			run: (e) => e.chain().focus().toggleItalic().run()
		},
		{
			label: 'Underline',
			Icon: Underline,
			isActive: (e) => e.isActive('underline'),
			run: (e) => e.chain().focus().toggleUnderline().run()
		},
		{
			label: 'Strikethrough',
			Icon: Strikethrough,
			isActive: (e) => e.isActive('strike'),
			run: (e) => e.chain().focus().toggleStrike().run()
		}
	],
	[
		{
			label: 'Bullet list',
			Icon: List,
			isActive: (e) => e.isActive('bulletList'),
			run: (e) => e.chain().focus().toggleBulletList().run()
		},
		{
			label: 'Numbered list',
			Icon: ListOrdered,
			isActive: (e) => e.isActive('orderedList'),
			run: (e) => e.chain().focus().toggleOrderedList().run()
		},
		{
			label: 'Quote',
			Icon: Quote,
			isActive: (e) => e.isActive('blockquote'),
			run: (e) => e.chain().focus().toggleBlockquote().run()
		}
	],
	(
		[
			['left', 'Align left', AlignLeft],
			['center', 'Align centre', AlignCenter],
			['right', 'Align right', AlignRight],
			['justify', 'Justify', AlignJustify]
		] as const
	).map(([alignment, label, Icon]) => ({
		label,
		Icon,
		isActive: (e: Editor) => e.isActive({ textAlign: alignment }),
		run: (e: Editor) => e.chain().focus().setTextAlign(alignment).run()
	}))
];

/** A horizontal rule starts a new page, named by its first line (ADR-0069). */
export const PAGE_BREAK_TOOL: Tool = {
	label: 'Page break (a new page)',
	Icon: SeparatorHorizontal,
	isActive: () => false,
	run: (e) => e.chain().focus().setHorizontalRule().run()
};
