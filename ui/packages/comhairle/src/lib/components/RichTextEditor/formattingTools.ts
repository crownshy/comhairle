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
import Undo2 from 'lucide-svelte/icons/undo-2';
import Redo2 from 'lucide-svelte/icons/redo-2';

/** One toolbar button's command, shared by every rich text toolbar. */
export type FormattingTool = {
	label: string;
	Icon: ComponentType<LucideIcon>;
	isActive: (editor: Editor) => boolean;
	run: (editor: Editor) => void;
	/** Greys the tool out while it has nothing to act on. */
	isDisabled?: (editor: Editor) => boolean;
};

function alignTool(
	alignment: 'left' | 'center' | 'right' | 'justify',
	label: string,
	Icon: ComponentType<LucideIcon>
): FormattingTool {
	return {
		label,
		Icon,
		isActive: (editor) => editor.isActive({ textAlign: alignment }),
		run: (editor) => editor.chain().focus().setTextAlign(alignment).run()
	};
}

export const FORMATTING_TOOLS = {
	undo: {
		label: 'Undo',
		Icon: Undo2,
		isActive: () => false,
		isDisabled: (editor) => !editor.can().undo(),
		run: (editor) => editor.chain().focus().undo().run()
	},
	redo: {
		label: 'Redo',
		Icon: Redo2,
		isActive: () => false,
		isDisabled: (editor) => !editor.can().redo(),
		run: (editor) => editor.chain().focus().redo().run()
	},
	paragraph: {
		label: 'Text',
		Icon: Pilcrow,
		isActive: (editor) => editor.isActive('paragraph'),
		run: (editor) => editor.chain().focus().setParagraph().run()
	},
	heading: {
		label: 'Heading',
		Icon: Heading2,
		isActive: (editor) => editor.isActive('heading', { level: 2 }),
		run: (editor) => editor.chain().focus().toggleHeading({ level: 2 }).run()
	},
	subheading: {
		label: 'Subheading',
		Icon: Heading3,
		isActive: (editor) => editor.isActive('heading', { level: 3 }),
		run: (editor) => editor.chain().focus().toggleHeading({ level: 3 }).run()
	},
	bold: {
		label: 'Bold',
		Icon: Bold,
		isActive: (editor) => editor.isActive('bold'),
		run: (editor) => editor.chain().focus().toggleBold().run()
	},
	italic: {
		label: 'Italic',
		Icon: Italic,
		isActive: (editor) => editor.isActive('italic'),
		run: (editor) => editor.chain().focus().toggleItalic().run()
	},
	underline: {
		label: 'Underline',
		Icon: Underline,
		isActive: (editor) => editor.isActive('underline'),
		run: (editor) => editor.chain().focus().toggleUnderline().run()
	},
	strike: {
		label: 'Strikethrough',
		Icon: Strikethrough,
		isActive: (editor) => editor.isActive('strike'),
		run: (editor) => editor.chain().focus().toggleStrike().run()
	},
	bulletList: {
		label: 'Bullet list',
		Icon: List,
		isActive: (editor) => editor.isActive('bulletList'),
		run: (editor) => editor.chain().focus().toggleBulletList().run()
	},
	orderedList: {
		label: 'Numbered list',
		Icon: ListOrdered,
		isActive: (editor) => editor.isActive('orderedList'),
		run: (editor) => editor.chain().focus().toggleOrderedList().run()
	},
	blockquote: {
		label: 'Quote',
		Icon: Quote,
		isActive: (editor) => editor.isActive('blockquote'),
		run: (editor) => editor.chain().focus().toggleBlockquote().run()
	},
	alignLeft: alignTool('left', 'Align left', AlignLeft),
	alignCenter: alignTool('center', 'Align centre', AlignCenter),
	alignRight: alignTool('right', 'Align right', AlignRight),
	alignJustify: alignTool('justify', 'Justify', AlignJustify)
} satisfies Record<string, FormattingTool>;
