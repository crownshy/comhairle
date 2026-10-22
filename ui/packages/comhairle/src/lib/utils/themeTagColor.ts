/**
 * Deterministic categorical color per tag label (theme title, or any other short
 * tag string), so repeated labels read as the same color everywhere they appear.
 * Palette is the validated 8-slot categorical set (dataviz skill reference palette):
 * fixed hue order, CVD-safe adjacent pairs in both light and dark. Past 8 distinct
 * labels, slots cycle — an acceptable trade for scanability in a tag pill, not a
 * precision series legend.
 */
const PALETTE: { light: string; dark: string }[] = [
	{ light: '#2a78d6', dark: '#3987e5' }, // blue
	{ light: '#eb6834', dark: '#d95926' }, // orange
	{ light: '#1baf7a', dark: '#199e70' }, // aqua
	{ light: '#eda100', dark: '#c98500' }, // yellow
	{ light: '#e87ba4', dark: '#d55181' }, // magenta
	{ light: '#008300', dark: '#008300' }, // green
	{ light: '#4a3aa7', dark: '#9085e9' }, // violet
	{ light: '#e34948', dark: '#e66767' } // red
];

function hashString(value: string): number {
	let hash = 0;
	for (let i = 0; i < value.length; i++) {
		hash = (hash * 31 + value.charCodeAt(i)) | 0;
	}
	return Math.abs(hash);
}

export function themeTagColor(label: string, isDark: boolean): string {
	const slot = PALETTE[hashString(label) % PALETTE.length];
	return isDark ? slot.dark : slot.light;
}
