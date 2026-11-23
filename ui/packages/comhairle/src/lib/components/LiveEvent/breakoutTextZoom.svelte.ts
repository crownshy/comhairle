const ZOOM_CLASSES = ['[zoom:0.875]', '[zoom:1]', '[zoom:1.25]', '[zoom:1.5]'] as const;
const DEFAULT_STEP = 1;

/** The text size a participant picked for breakout questions. */
export class BreakoutTextZoom {
	step = $state(DEFAULT_STEP);

	canShrink = $derived(this.step > 0);
	canGrow = $derived(this.step < ZOOM_CLASSES.length - 1);
	zoomClass = $derived(ZOOM_CLASSES[this.step]);

	shrink() {
		if (this.canShrink) this.step--;
	}

	grow() {
		if (this.canGrow) this.step++;
	}
}

/** One shared instance, so the size holds between breakout sessions and across the desktop
 *  panel and mobile sheet. Only client clicks change it, so the server always renders the default. */
export const breakoutTextZoom = new BreakoutTextZoom();
