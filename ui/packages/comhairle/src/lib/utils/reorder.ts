/**
 * Return a new array with the item at `index` moved one slot in `direction`
 * (`-1` = towards the start, `+1` = towards the end). If the move would fall off
 * either end (or `index` is out of range) the array is returned unchanged (as a copy).
 * Pure, so it powers the board's button/keyboard reorder without touching the DOM.
 */
export function moveItem<T>(items: readonly T[], index: number, direction: -1 | 1): T[] {
	const next = [...items];
	const target = index + direction;
	if (index < 0 || index >= next.length || target < 0 || target >= next.length) {
		return next;
	}
	[next[index], next[target]] = [next[target], next[index]];
	return next;
}
