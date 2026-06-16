export const DEFAULT_WIDTH = 288;
export const MIN_WIDTH = 240;
export const MAX_WIDTH = 480;
export const COLLAPSE_THRESHOLD = 100;
export const EXPAND_WIDTH = 320;

class SidebarWidthStore {
	width = $state(DEFAULT_WIDTH);
	resizing = $state(false);
	initializing = $state(false);

	hydrate() {}

	set(px: number) {
		this.width = Math.max(MIN_WIDTH, Math.min(MAX_WIDTH, px));
	}

	persist() {}
}

export const sidebarWidth = new SidebarWidthStore();
