<script lang="ts">
	import type { Editor } from '@tiptap/core';
	import { BubbleMenuPlugin } from '@tiptap/extension-bubble-menu';
	import { PluginKey } from '@tiptap/pm/state';
	import {
		tableRowActions,
		tableColumnActions,
		tableHeaderAction,
		tableDeleteAction,
		type TableAction
	} from './tableActions';

	type Props = {
		editor: Editor;
	};

	let { editor }: Props = $props();

	let element = $state<HTMLElement>();

	// Groups render with a divider between them, mirroring the toolbar dropdown.
	const groups: TableAction[][] = [
		tableRowActions,
		tableColumnActions,
		[tableHeaderAction, tableDeleteAction]
	];

	$effect(() => {
		if (!editor || !element) return;

		// A fresh key per mount so re-created editors don't collide on the same key.
		const pluginKey = new PluginKey('tableBubbleMenu');

		editor.registerPlugin(
			BubbleMenuPlugin({
				pluginKey,
				editor,
				element,
				// Show whenever the caret is inside a table (even with an empty
				// selection), but never in the read-only case.
				shouldShow: ({ editor }) => editor.isEditable && editor.isActive('table'),
				options: { placement: 'top', offset: 8 }
			})
		);

		return () => {
			// Guard: the editor may already be torn down on unmount.
			if (!editor.isDestroyed) editor.unregisterPlugin(pluginKey);
		};
	});

	function run(action: TableAction) {
		if (editor) action.run(editor);
	}
</script>

<!-- The plugin controls visibility/opacity and absolute position; seed them so the
     menu doesn't flash in the layout before the plugin initialises. -->
<div
	bind:this={element}
	class="bg-popover text-popover-foreground border-border z-50 flex w-max items-center gap-0.5 rounded-lg border p-1 shadow-md"
	style="position: absolute; visibility: hidden; opacity: 0;"
	role="toolbar"
	aria-label="Table controls"
>
	{#each groups as group, i (i)}
		{#if i > 0}
			<div class="bg-border mx-0.5 h-5 w-px shrink-0"></div>
		{/if}
		{#each group as action (action.label)}
			<button
				type="button"
				title={action.label}
				aria-label={action.label}
				class="flex h-7 w-7 shrink-0 items-center justify-center rounded"
				class:hover:bg-accent={!action.destructive}
				class:hover:text-accent-foreground={!action.destructive}
				class:text-destructive={action.destructive}
				class:hover:bg-destructive={action.destructive}
				class:hover:text-destructive-foreground={action.destructive}
				onclick={() => run(action)}
			>
				<action.Icon size={16} />
			</button>
		{/each}
	{/each}
</div>
