export const DEFAULT_WIDTH = 288;
export const MIN_WIDTH = 240;
export const MAX_WIDTH = 480;
export const COLLAPSE_THRESHOLD = 100;
export const EXPAND_WIDTH = 320;

const STORAGE_KEY = 'comhairle:sidebarWidth';

class SidebarWidthStore {
	width = $state(DEFAULT_WIDTH);
	resizing = $state(false);
	initializing = $state(false);

	hydrate() {
		if (typeof window === 'undefined') return;
		this.initializing = true;
		try {
			const raw = window.localStorage.getItem(STORAGE_KEY);
			if (raw !== null) {
				const parsed = Number(raw);
				if (Number.isFinite(parsed)) this.set(parsed);
			}
		} catch {
			// localStorage unavailable (private mode, quota, etc.) — fall back to default
		}
		requestAnimationFrame(() => {
			this.initializing = false;
		});
	}

	set(px: number) {
		this.width = Math.max(MIN_WIDTH, Math.min(MAX_WIDTH, px));
	}

	persist() {
		if (typeof window === 'undefined') return;
		try {
			window.localStorage.setItem(STORAGE_KEY, String(this.width));
		} catch {
			// localStorage write failed — non-fatal, width still in memory
		}
	}
}

export const sidebarWidth = new SidebarWidthStore();
