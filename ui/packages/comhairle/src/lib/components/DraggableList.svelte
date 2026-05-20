<script lang="ts" generics="T extends { id: string }">
	import { dndzone, type DndEvent } from 'svelte-dnd-action';
	import { flip } from 'svelte/animate';
	import type { Snippet } from 'svelte';

	let {
		items,
		onReorder,
		onCommit,
		children,
		flipDurationMs = 200,
		dragDisabled = false,
		dropTargetStyle,
		class: className = ''
	}: {
		/** Source of truth from the parent. */
		items: T[];
		/** Called continuously while dragging — parent should set its local state to `next` so the lib can render the in-flight order. */
		onReorder: (next: T[]) => void;
		/** Called once on drop. Use this to persist the new order. */
		onCommit?: (next: T[]) => void;
		/** Renders one row. Receives the item and its current index. */
		children: Snippet<[T, number]>;
		flipDurationMs?: number;
		dragDisabled?: boolean;
		dropTargetStyle?: Record<string, string>;
		class?: string;
	} = $props();

	function consider(e: CustomEvent<DndEvent<T>>) {
		onReorder(e.detail.items);
	}

	function finalize(e: CustomEvent<DndEvent<T>>) {
		onReorder(e.detail.items);
		onCommit?.(e.detail.items);
	}
</script>

<ul
	class={className}
	use:dndzone={{ items, flipDurationMs, dragDisabled, dropTargetStyle: dropTargetStyle ?? {} }}
	onconsider={consider}
	onfinalize={finalize}
>
	{#each items as item, i (item.id)}
		<li animate:flip={{ duration: flipDurationMs }}>
			{@render children(item, i)}
		</li>
	{/each}
</ul>
