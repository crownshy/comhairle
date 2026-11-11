import { Image as BaseImage } from '@tiptap/extension-image';

/**
 * The stock Image node plus `align` and `width`. Both are stored as keys and rendered as
 * `data-align` / `data-width`, which editor-content.css lays out, so the document holds no raw
 * CSS. Keep these lists in sync with that stylesheet.
 */
export type ImageAlign = 'left' | 'center' | 'right';
export type ImageWidth = '25' | '50' | '75' | '100';

export const IMAGE_ALIGN_OPTIONS: { key: ImageAlign; label: string }[] = [
	{ key: 'left', label: 'Align left' },
	{ key: 'center', label: 'Align centre' },
	{ key: 'right', label: 'Align right' }
];

export const IMAGE_WIDTH_OPTIONS: { key: ImageWidth; label: string }[] = [
	{ key: '25', label: '25%' },
	{ key: '50', label: '50%' },
	{ key: '75', label: '75%' },
	{ key: '100', label: '100%' }
];

const ALIGN_KEYS = new Set<string>(IMAGE_ALIGN_OPTIONS.map((option) => option.key));
const WIDTH_KEYS = new Set<string>(IMAGE_WIDTH_OPTIONS.map((option) => option.key));

// Stored JSON skips parseHTML, so rendering checks the value against the list too.
function knownKey(value: unknown, keys: Set<string>): string | null {
	return typeof value === 'string' && keys.has(value) ? value : null;
}

export const Image = BaseImage.extend({
	addAttributes() {
		return {
			...this.parent?.(),
			align: {
				default: null as ImageAlign | null,
				parseHTML: (element: HTMLElement) =>
					knownKey(element.getAttribute('data-align'), ALIGN_KEYS),
				renderHTML: (attributes: Record<string, unknown>) => {
					const align = knownKey(attributes.align, ALIGN_KEYS);
					return align ? { 'data-align': align } : {};
				}
			},
			width: {
				default: null as ImageWidth | null,
				parseHTML: (element: HTMLElement) =>
					knownKey(element.getAttribute('data-width'), WIDTH_KEYS),
				renderHTML: (attributes: Record<string, unknown>) => {
					const width = knownKey(attributes.width, WIDTH_KEYS);
					return width ? { 'data-width': width } : {};
				}
			}
		};
	}
});
