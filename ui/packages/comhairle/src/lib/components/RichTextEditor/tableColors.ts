/**
 * Fixed palette for table cell backgrounds. Cells store a colour KEY (not a raw
 * hex), rendered as `data-cell-color="<key>"`. The actual colour is resolved in
 * editor-content.css via `color-mix(... var(--color-background))`, so each key is
 * a soft tint in light mode and a muted dark tint in dark mode - readable in both,
 * without storing theme-specific values. Keep these keys in sync with the CSS.
 */
export type TableCellColorKey =
	| 'gray'
	| 'red'
	| 'orange'
	| 'yellow'
	| 'green'
	| 'blue'
	| 'purple'
	| 'pink';

export type TableCellColorOption = {
	/** null clears any colour (the "None" swatch). */
	key: TableCellColorKey | null;
	label: string;
};

export const TABLE_CELL_COLORS: TableCellColorOption[] = [
	{ key: null, label: 'None' },
	{ key: 'gray', label: 'Gray' },
	{ key: 'red', label: 'Red' },
	{ key: 'orange', label: 'Orange' },
	{ key: 'yellow', label: 'Yellow' },
	{ key: 'green', label: 'Green' },
	{ key: 'blue', label: 'Blue' },
	{ key: 'purple', label: 'Purple' },
	{ key: 'pink', label: 'Pink' }
];
