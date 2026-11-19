import { Image as BaseImage } from '@tiptap/extension-image';

export type ImageAlign = 'left' | 'center' | 'right';
export type ImageWidth = '25' | '50' | '75' | '100';

/** The layout keys an image node carries. `null` means the default layout. */
export type ImageLayout = { align: ImageAlign | null; width: ImageWidth | null };

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

function knownKey<Key extends string>(value: unknown, keys: Set<string>): Key | null {
	return typeof value === 'string' && keys.has(value) ? (value as Key) : null;
}

/** Reads an image's layout from its node attributes, dropping values outside the option lists. */
export function readImageLayout(attributes: Record<string, unknown>): ImageLayout {
	return {
		align: knownKey<ImageAlign>(attributes.align, ALIGN_KEYS),
		width: knownKey<ImageWidth>(attributes.width, WIDTH_KEYS)
	};
}

// Stored JSON skips parseHTML, so rendering checks the value against the list too.
function layoutAttribute(name: keyof ImageLayout, keys: Set<string>) {
	const dataAttribute = `data-${name}`;
	return {
		default: null,
		parseHTML: (element: HTMLElement) => knownKey(element.getAttribute(dataAttribute), keys),
		renderHTML: (attributes: Record<string, unknown>) => {
			const value = knownKey(attributes[name], keys);
			return value ? { [dataAttribute]: value } : {};
		}
	};
}

/**
 * The stock Image node plus `align` and `width`. Both are stored as keys and rendered as
 * `data-align` / `data-width`, which editor-content.css lays out, so the document holds no raw
 * CSS. Keep the option lists in sync with that stylesheet.
 */
export const Image = BaseImage.extend({
	addAttributes() {
		return {
			...this.parent?.(),
			align: layoutAttribute('align', ALIGN_KEYS),
			width: layoutAttribute('width', WIDTH_KEYS)
		};
	}
});
