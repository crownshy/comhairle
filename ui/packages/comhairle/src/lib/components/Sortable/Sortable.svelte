<script lang="ts" generics="T extends { id: string }">
	import type { Snippet } from 'svelte';
	import { flip } from 'svelte/animate';
	import { cubicOut } from 'svelte/easing';

	type Props = {
		items: T[];
		onReorder: (
			orderedIds: string[],
			move: { movedId: string; fromIndex: number; toIndex: number }
		) => void;
		item: Snippet<[{ item: T; index: number; isDragging: boolean }]>;
		class?: string;
		disabled?: boolean;
	};

	let {
		items,
		onReorder,
		item: itemSnippet,
		class: className = '',
		disabled = false
	}: Props = $props();

	let draggedIndex = $state<number | null>(null);
	let dragOverIndex = $state<number | null>(null);

	function handleDragStart(e: DragEvent, index: number) {
		if (disabled) return;
		draggedIndex = index;
		if (e.dataTransfer) {
			e.dataTransfer.effectAllowed = 'move';
			e.dataTransfer.setData('text/plain', String(index));
		}
	}

	function handleDragOver(e: DragEvent, index: number) {
		if (disabled || draggedIndex === null) return;
		e.preventDefault();
		if (e.dataTransfer) e.dataTransfer.dropEffect = 'move';
		dragOverIndex = index;
	}

	function handleDragLeave(index: number) {
		if (dragOverIndex === index) dragOverIndex = null;
	}

	function handleDrop(e: DragEvent, targetIndex: number) {
		if (disabled) return;
		e.preventDefault();
		if (draggedIndex === null || draggedIndex === targetIndex) {
			reset();
			return;
		}
		const ids = items.map((i) => i.id);
		const [moved] = ids.splice(draggedIndex, 1);
		ids.splice(targetIndex, 0, moved);
		onReorder(ids, { movedId: moved, fromIndex: draggedIndex, toIndex: targetIndex });
		reset();
	}

	function reset() {
		draggedIndex = null;
		dragOverIndex = null;
	}
</script>

<div class={className} role="list">
	{#each items as it, i (it.id)}
		{@const showIndicatorBefore =
			dragOverIndex === i && draggedIndex !== null && draggedIndex > i}
		{@const showIndicatorAfter =
			dragOverIndex === i && draggedIndex !== null && draggedIndex < i}
		<div
			role="listitem"
			draggable={!disabled}
			ondragstart={(e) => handleDragStart(e, i)}
			ondragover={(e) => handleDragOver(e, i)}
			ondragleave={() => handleDragLeave(i)}
			ondrop={(e) => handleDrop(e, i)}
			ondragend={reset}
			animate:flip={{ duration: 300, easing: cubicOut }}
			class="relative transition-all duration-200"
			class:opacity-40={draggedIndex === i}
			class:scale-95={draggedIndex === i}
			class:shadow-2xl={draggedIndex === i}
			class:z-10={draggedIndex === i}
		>
			{#if showIndicatorBefore}
				<div
					class="bg-primary pointer-events-none absolute -top-2 right-0 left-0 h-1 rounded-full"
				></div>
			{/if}
			{@render itemSnippet({ item: it, index: i, isDragging: draggedIndex === i })}
			{#if showIndicatorAfter}
				<div
					class="bg-primary pointer-events-none absolute right-0 -bottom-2 left-0 h-1 rounded-full"
				></div>
			{/if}
		</div>
	{/each}
</div>
