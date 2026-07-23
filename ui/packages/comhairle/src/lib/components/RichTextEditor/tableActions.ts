import type { Editor } from '@tiptap/core';
import {
	ArrowUpToLine,
	ArrowDownToLine,
	ArrowLeftToLine,
	ArrowRightToLine,
	Heading,
	Trash2,
	type Icon
} from 'lucide-svelte';
import type { ComponentType } from 'svelte';

/**
 * The set of table operations shared by the toolbar dropdown and the floating
 * (bubble) menu, so the two surfaces never drift. Each `run` takes the editor
 * so the components stay presentational.
 */
export type TableAction = {
	label: string;
	Icon: ComponentType<Icon>;
	run: (editor: Editor) => void;
	/** Renders in a warning colour and, in menus, typically closes the surface. */
	destructive?: boolean;
};

export const tableRowActions: TableAction[] = [
	{
		label: 'Row above',
		Icon: ArrowUpToLine,
		run: (editor) => editor.chain().focus().addRowBefore().run()
	},
	{
		label: 'Row below',
		Icon: ArrowDownToLine,
		run: (editor) => editor.chain().focus().addRowAfter().run()
	},
	{
		label: 'Delete row',
		Icon: Trash2,
		run: (editor) => editor.chain().focus().deleteRow().run()
	}
];

export const tableColumnActions: TableAction[] = [
	{
		label: 'Column left',
		Icon: ArrowLeftToLine,
		run: (editor) => editor.chain().focus().addColumnBefore().run()
	},
	{
		label: 'Column right',
		Icon: ArrowRightToLine,
		run: (editor) => editor.chain().focus().addColumnAfter().run()
	},
	{
		label: 'Delete column',
		Icon: Trash2,
		run: (editor) => editor.chain().focus().deleteColumn().run()
	}
];

export const tableHeaderAction: TableAction = {
	label: 'Toggle header row',
	Icon: Heading,
	run: (editor) => editor.chain().focus().toggleHeaderRow().run()
};

export const tableDeleteAction: TableAction = {
	label: 'Delete table',
	Icon: Trash2,
	run: (editor) => editor.chain().focus().deleteTable().run(),
	destructive: true
};

/** Default table on insert: 3x3 with a header row. */
export function insertDefaultTable(editor: Editor) {
	editor.chain().focus().insertTable({ rows: 3, cols: 3, withHeaderRow: true }).run();
}
