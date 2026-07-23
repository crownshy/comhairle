<script lang="ts">
	import * as Popover from '$lib/components/ui/popover';
	import type { Editor } from '@tiptap/core';
	import type { Snippet } from 'svelte';
	import {
		tableRowActions,
		tableColumnActions,
		tableHeaderAction,
		tableDeleteAction,
		type TableAction
	} from './tableActions';

	type Props = {
		editor: Editor | undefined;
		/**
		 * The trigger button. It only opens this popover when the caret is already
		 * inside a table; the not-in-a-table click (which inserts) is handled by the
		 * caller stopping propagation before the popover sees it.
		 */
		children: Snippet;
	};

	let { editor, children }: Props = $props();

	let open = $state(false);

	function run(action: TableAction) {
		if (editor) action.run(editor);
		// Row/column edits keep the menu open for repeated tweaks; deleting the
		// table leaves nothing to manage, so close.
		if (action.destructive) open = false;
	}
</script>

{#snippet menuItem(action: TableAction)}
	<button
		type="button"
		class="flex w-full items-center gap-2 rounded-md px-2 py-1.5 text-left text-sm"
		class:hover:bg-accent={!action.destructive}
		class:hover:text-accent-foreground={!action.destructive}
		class:text-destructive={action.destructive}
		class:hover:bg-destructive={action.destructive}
		class:hover:text-destructive-foreground={action.destructive}
		onclick={() => run(action)}
	>
		<action.Icon class="h-4 w-4 shrink-0 {action.destructive ? '' : 'text-muted-foreground'}" />
		{action.label}
	</button>
{/snippet}

<Popover.Root bind:open>
	<Popover.Trigger>
		{@render children()}
	</Popover.Trigger>
	<Popover.Content class="w-56" side="bottom" align="start">
		<div class="flex flex-col gap-0.5">
			<p class="text-muted-foreground px-2 py-1 text-xs font-medium">Rows</p>
			{#each tableRowActions as action (action.label)}
				{@render menuItem(action)}
			{/each}

			<div class="bg-border my-1 h-px"></div>

			<p class="text-muted-foreground px-2 py-1 text-xs font-medium">Columns</p>
			{#each tableColumnActions as action (action.label)}
				{@render menuItem(action)}
			{/each}

			<div class="bg-border my-1 h-px"></div>

			{@render menuItem(tableHeaderAction)}
			{@render menuItem(tableDeleteAction)}
		</div>
	</Popover.Content>
</Popover.Root>
