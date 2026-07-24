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
	import { TABLE_CELL_COLORS, type TableCellColorOption } from './tableColors';

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

	function setColor(option: TableCellColorOption) {
		// Applies to every cell in the current selection (or the caret's cell).
		editor?.chain().focus().setCellAttribute('cellColor', option.key).run();
	}

	// Preview swatch: the same mix used in editor-content.css, so it matches the cell.
	function swatchBackground(option: TableCellColorOption): string {
		const hue: Record<string, string> = {
			gray: '#6b7280',
			red: '#ef4444',
			orange: '#f97316',
			yellow: '#eab308',
			green: '#22c55e',
			blue: '#3b82f6',
			purple: '#a855f7',
			pink: '#ec4899'
		};
		if (!option.key) return 'var(--color-background)';
		return `color-mix(in srgb, ${hue[option.key]} 16%, var(--color-background))`;
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

			<p class="text-muted-foreground px-2 py-1 text-xs font-medium">Cell colour</p>
			<div class="flex flex-wrap gap-1 px-2 py-1">
				{#each TABLE_CELL_COLORS as option (option.label)}
					<button
						type="button"
						class="border-border flex h-6 w-6 items-center justify-center rounded-md border"
						style="background: {swatchBackground(option)};"
						title={option.label}
						aria-label={option.key ? `${option.label} background` : 'No background'}
						onclick={() => setColor(option)}
					>
						{#if !option.key}
							<span class="text-muted-foreground text-xs leading-none">/</span>
						{/if}
					</button>
				{/each}
			</div>

			<div class="bg-border my-1 h-px"></div>

			{@render menuItem(tableHeaderAction)}
			{@render menuItem(tableDeleteAction)}
		</div>
	</Popover.Content>
</Popover.Root>
