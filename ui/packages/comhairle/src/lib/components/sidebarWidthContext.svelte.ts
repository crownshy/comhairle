import { getContext, setContext } from 'svelte';
import { serializeWidthCookie } from './sidebarWidth';

/**
 * Getters/setter the `(admin)` layout hands to the width context.
 *
 * `width` is a getter so the context stays in sync with the layout's writable
 * `$derived(data.sidebarWidth)` (seeded from the server-read cookie), the same
 * getter pattern shadcn's sidebar context uses for `open`.
 */
export type SidebarWidthProps = {
	/** Current sidebar width in px. */
	width: () => number;
	/** Set the sidebar width in px. The layout is responsible for clamping. */
	setWidth: (px: number) => void;
};

/**
 * Reactive sidebar-width state shared with `SidebarResizeHandle` and `AdminNav`.
 *
 * The width itself lives in the layout (so SSR and the live value are one reactive
 * expression seeded from load data, with no hydration mismatch). This class only
 * adds the transient `resizing` flag and the cookie write.
 */
class SidebarWidthState {
	#width: () => number;
	setWidth: (px: number) => void;
	/** True while a live drag is in progress; disables the width transition. */
	resizing = $state(false);

	constructor(props: SidebarWidthProps) {
		this.#width = props.width;
		this.setWidth = props.setWidth;
	}

	get width() {
		return this.#width();
	}

	/** Persist the current width to the `sidebar:width` cookie (client only). */
	persist() {
		if (typeof document === 'undefined') return;
		document.cookie = serializeWidthCookie(this.width);
	}
}

export type { SidebarWidthState };

const SYMBOL_KEY = 'comhairle-sidebar-width';

/** Create the sidebar-width context and set it for descendants. */
export function setSidebarWidth(props: SidebarWidthProps): SidebarWidthState {
	return setContext(Symbol.for(SYMBOL_KEY), new SidebarWidthState(props));
}

/** Read the sidebar-width context. Must be called under a `setSidebarWidth` ancestor. */
export function useSidebarWidth(): SidebarWidthState {
	const context = getContext<SidebarWidthState | undefined>(Symbol.for(SYMBOL_KEY));
	if (!context) {
		throw new Error('useSidebarWidth must be used within the (admin) layout');
	}
	return context;
}
