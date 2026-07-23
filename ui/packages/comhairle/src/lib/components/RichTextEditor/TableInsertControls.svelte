<script lang="ts">
	import type { Editor } from '@tiptap/core';
	import { Plus } from 'lucide-svelte';

	type Props = {
		editor: Editor;
	};

	let { editor }: Props = $props();

	// A single "+" that snaps to the nearest edge intersection: a column line along
	// the table's top edge (inserts a column) or a row line along the left edge
	// (inserts a row). Snapping to these discrete points keeps it from jumping as the
	// cursor moves. Positioned with `fixed` from live cell rects, so it stays aligned
	// through column resizing and horizontal scroll of the table wrapper.
	type Indicator = {
		axis: 'col' | 'row';
		/** viewport coords of the boundary point the "+" centres on */
		x: number;
		y: number;
		/**
		 * Span of the blue guide line drawn along the affected boundary, so it's
		 * obvious whether a column or a row will be inserted. For a column it's the
		 * table's top..bottom (a vertical line); for a row, left..right (horizontal).
		 */
		lineStart: number;
		lineEnd: number;
		insert: () => void;
	};

	let indicator = $state<Indicator | null>(null);

	// How close (px) the cursor must be to an edge intersection for its "+" to show.
	const RADIUS = 30;

	function firstRowCells(table: HTMLElement): HTMLElement[] {
		const row = table.querySelector('tr');
		return row ? (Array.from(row.children) as HTMLElement[]) : [];
	}

	function rowFirstCells(table: HTMLElement): HTMLElement[] {
		const rows = Array.from(table.querySelectorAll('tr')) as HTMLElement[];
		return rows.map((r) => r.children[0] as HTMLElement).filter(Boolean);
	}

	/** Move the caret into the given cell, then run the insert command. */
	function insertAt(cell: HTMLElement | undefined, before: boolean, axis: 'col' | 'row') {
		if (!cell) return;
		let pos: number;
		try {
			pos = editor.view.posAtDOM(cell, 0);
		} catch {
			return;
		}
		const chain = editor.chain().focus().setTextSelection(pos);
		if (axis === 'col') {
			(before ? chain.addColumnBefore() : chain.addColumnAfter()).run();
		} else {
			(before ? chain.addRowBefore() : chain.addRowAfter()).run();
		}
		indicator = null;
	}

	function update(clientX: number, clientY: number) {
		if (!editor?.isEditable) {
			indicator = null;
			return;
		}

		const tables = Array.from(editor.view.dom.querySelectorAll('table')) as HTMLElement[];

		// Find the single nearest edge intersection across all tables: column lines
		// along the top edge, row lines along the left edge. Snapping to these
		// discrete points (rather than tracking the cursor continuously) is what
		// keeps the "+" from jumping around as the cursor moves.
		let best: Indicator | null = null;
		let bestDist = RADIUS;

		for (const table of tables) {
			const rect = table.getBoundingClientRect();
			if (
				clientX < rect.left - RADIUS ||
				clientX > rect.right + RADIUS ||
				clientY < rect.top - RADIUS ||
				clientY > rect.bottom + RADIUS
			) {
				continue;
			}

			const colCells = firstRowCells(table);
			const rowCells = rowFirstCells(table);
			if (!colCells.length || !rowCells.length) continue;

			const numCols = colCells.length;
			const numRows = rowCells.length;

			// Top-edge intersections (one per column line) -> insert a column.
			const colBoundaries = [
				colCells[0].getBoundingClientRect().left,
				...colCells.map((c) => c.getBoundingClientRect().right)
			];
			colBoundaries.forEach((x, i) => {
				const dist = Math.hypot(clientX - x, clientY - rect.top);
				if (dist < bestDist) {
					bestDist = dist;
					best = {
						axis: 'col',
						x,
						y: rect.top,
						lineStart: rect.top,
						lineEnd: rect.bottom,
						insert: () =>
							insertAt(colCells[Math.min(i, numCols - 1)], i < numCols, 'col')
					};
				}
			});

			// Left-edge intersections (one per row line) -> insert a row.
			const rowBoundaries = [
				rowCells[0].getBoundingClientRect().top,
				...rowCells.map((c) => c.getBoundingClientRect().bottom)
			];
			rowBoundaries.forEach((y, j) => {
				const dist = Math.hypot(clientX - rect.left, clientY - y);
				if (dist < bestDist) {
					bestDist = dist;
					best = {
						axis: 'row',
						x: rect.left,
						y,
						lineStart: rect.left,
						lineEnd: rect.right,
						insert: () =>
							insertAt(rowCells[Math.min(j, numRows - 1)], j < numRows, 'row')
					};
				}
			});
		}

		indicator = best;
	}

	let frame = 0;

	function onPointerMove(event: PointerEvent) {
		const { clientX, clientY } = event;
		if (frame) return;
		frame = requestAnimationFrame(() => {
			frame = 0;
			update(clientX, clientY);
		});
	}

	$effect(() => {
		if (!editor) return;

		const clear = () => (indicator = null);

		// Listen on the document (not just the editor) so the "+" stays put when the
		// cursor moves off the table onto the button itself, which sits on the edge.
		document.addEventListener('pointermove', onPointerMove, { passive: true });
		// Any scroll invalidates the cached rects; hide and let the next move recompute.
		window.addEventListener('scroll', clear, true);

		return () => {
			document.removeEventListener('pointermove', onPointerMove);
			window.removeEventListener('scroll', clear, true);
			if (frame) cancelAnimationFrame(frame);
		};
	});
</script>

{#if indicator}
	<!-- Blue guide line along the boundary the "+" will insert at, so it's clear
	     whether a column (vertical line) or a row (horizontal line) is coming. -->
	<div
		class="table-insert-line"
		style={indicator.axis === 'col'
			? `left: ${indicator.x}px; top: ${indicator.lineStart}px; height: ${indicator.lineEnd - indicator.lineStart}px; width: 2px; transform: translateX(-50%);`
			: `top: ${indicator.y}px; left: ${indicator.lineStart}px; width: ${indicator.lineEnd - indicator.lineStart}px; height: 2px; transform: translateY(-50%);`}
	></div>
	<button
		type="button"
		class="table-insert-plus"
		style="left: {indicator.x}px; top: {indicator.y}px;"
		title={indicator.axis === 'col' ? 'Insert column' : 'Insert row'}
		aria-label={indicator.axis === 'col' ? 'Insert column' : 'Insert row'}
		onclick={() => indicator?.insert()}
	>
		<Plus size={12} strokeWidth={3} />
	</button>
{/if}

<style>
	.table-insert-line {
		position: fixed;
		z-index: 39;
		background: var(--color-primary);
		border-radius: 1px;
		pointer-events: none;
	}

	.table-insert-plus {
		position: fixed;
		z-index: 40;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 18px;
		height: 18px;
		padding: 0;
		border: 2px solid var(--color-background);
		border-radius: 9999px;
		background: var(--color-primary);
		color: var(--color-primary-foreground);
		cursor: pointer;
		transform: translate(-50%, -50%);
		box-shadow: 0 1px 3px rgb(0 0 0 / 0.25);
		transition:
			transform 100ms ease,
			background-color 100ms ease;
	}

	.table-insert-plus:hover {
		transform: translate(-50%, -50%) scale(1.15);
	}
</style>
