<script lang="ts">
	import * as Popover from '$lib/components/ui/popover';
	import type { Editor } from '@tiptap/core';
	import type { Snippet } from 'svelte';
	import {
		Table as TableIcon,
		ArrowUpToLine,
		ArrowDownToLine,
		ArrowLeftToLine,
		ArrowRightToLine,
		Heading,
		Trash2,
		type Icon
	} from 'lucide-svelte';
	import type { ComponentType } from 'svelte';

	type Props = {
		editor: Editor | undefined;
		/** Whether the selection is currently inside a table (drives insert vs. manage view). */
		inTable: boolean;
		children: Snippet;
	};

	let { editor, inTable, children }: Props = $props();

	let open = $state(false);

	function insertTable() {
		editor?.chain().focus().insertTable({ rows: 3, cols: 3, withHeaderRow: true }).run();
		open = false;
	}

	function deleteTable() {
		editor?.chain().focus().deleteTable().run();
		open = false;
	}

	// Row/column edits keep the popover open so several can be applied in a row.
	type MenuItem = { label: string; Icon: ComponentType<Icon>; run: () => void };

	const rowItems: MenuItem[] = [
		{
			label: 'Row above',
			Icon: ArrowUpToLine,
			run: () => editor?.chain().focus().addRowBefore().run()
		},
		{
			label: 'Row below',
			Icon: ArrowDownToLine,
			run: () => editor?.chain().focus().addRowAfter().run()
		},
		{ label: 'Delete row', Icon: Trash2, run: () => editor?.chain().focus().deleteRow().run() }
	];

	const columnItems: MenuItem[] = [
		{
			label: 'Column left',
			Icon: ArrowLeftToLine,
			run: () => editor?.chain().focus().addColumnBefore().run()
		},
		{
			label: 'Column right',
			Icon: ArrowRightToLine,
			run: () => editor?.chain().focus().addColumnAfter().run()
		},
		{
			label: 'Delete column',
			Icon: Trash2,
			run: () => editor?.chain().focus().deleteColumn().run()
		}
	];
</script>

{#snippet menuItem(item: MenuItem)}
	<button
		type="button"
		class="hover:bg-accent hover:text-accent-foreground flex w-full items-center gap-2 rounded-md px-2 py-1.5 text-left text-sm"
		onclick={item.run}
	>
		<item.Icon class="text-muted-foreground h-4 w-4 shrink-0" />
		{item.label}
	</button>
{/snippet}

<Popover.Root bind:open>
	<Popover.Trigger>
		{@render children()}
	</Popover.Trigger>
	<Popover.Content class="w-56" side="bottom" align="start">
		{#if inTable}
			<div class="flex flex-col gap-0.5">
				<p class="text-muted-foreground px-2 py-1 text-xs font-medium">Rows</p>
				{#each rowItems as item (item.label)}
					{@render menuItem(item)}
				{/each}

				<div class="bg-border my-1 h-px"></div>

				<p class="text-muted-foreground px-2 py-1 text-xs font-medium">Columns</p>
				{#each columnItems as item (item.label)}
					{@render menuItem(item)}
				{/each}

				<div class="bg-border my-1 h-px"></div>

				{@render menuItem({
					label: 'Toggle header row',
					Icon: Heading,
					run: () => editor?.chain().focus().toggleHeaderRow().run()
				})}
				<button
					type="button"
					class="text-destructive hover:bg-destructive/10 flex w-full items-center gap-2 rounded-md px-2 py-1.5 text-left text-sm"
					onclick={deleteTable}
				>
					<Trash2 class="h-4 w-4 shrink-0" />
					Delete table
				</button>
			</div>
		{:else}
			<button
				type="button"
				class="hover:bg-accent hover:text-accent-foreground flex w-full items-center gap-2 rounded-md px-2 py-1.5 text-left text-sm"
				onclick={insertTable}
			>
				<TableIcon class="text-muted-foreground h-4 w-4 shrink-0" />
				Insert table
			</button>
		{/if}
	</Popover.Content>
</Popover.Root>
