<script lang="ts" generics="T extends { id: string }">
	import type { Snippet } from 'svelte';

	//this is a generic sortable component that can be used to sort any list of items. Will extend it a little bit to later support the draggable workflow steps

	type Props = {
		items: T[];
		onReorder: (orderedIds: string[]) => void;
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
		onReorder(ids);
		reset();
	}

	function reset() {
		draggedIndex = null;
		dragOverIndex = null;
	}
</script>

<div class={className} role="list">
	{#each items as it, i (it.id)}
		<div
			role="listitem"
			draggable={!disabled}
			ondragstart={(e) => handleDragStart(e, i)}
			ondragover={(e) => handleDragOver(e, i)}
			ondragleave={() => handleDragLeave(i)}
			ondrop={(e) => handleDrop(e, i)}
			ondragend={reset}
			class="transition-opacity"
			class:opacity-40={draggedIndex === i}
			class:ring-2={dragOverIndex === i && draggedIndex !== i}
			class:ring-primary={dragOverIndex === i && draggedIndex !== i}
			class:rounded-md={dragOverIndex === i && draggedIndex !== i}
		>
			{@render itemSnippet({ item: it, index: i, isDragging: draggedIndex === i })}
		</div>
	{/each}
</div>
