import { driver } from 'driver.js';
import 'driver.js/dist/driver.css';
import type { Tour } from './types';

const SEEN_KEY_PREFIX = 'comhairle-tour-seen:';

function hasSeen(id: string): boolean {
	if (typeof window === 'undefined') return true;
	return localStorage.getItem(SEEN_KEY_PREFIX + id) === '1';
}

function markSeen(id: string) {
	if (typeof window === 'undefined') return;
	localStorage.setItem(SEEN_KEY_PREFIX + id, '1');
}

function runDriver(tour: Tour) {
	driver({
		showProgress: tour.steps.length > 1,
		allowClose: true,
		overlayColor: 'black',
		steps: tour.steps.map((step) => ({
			element: step.element,
			popover: {
				title: step.title,
				description: step.description,
				side: step.side,
				align: step.align
			}
		}))
	}).drive();
}

/**
 * Tracks which page's tour is current, and whether it's been played before. A page registers
 * its tour on mount via `useTour`; the "replay tour" button in the admin top bar reads it back
 * to know what to play and whether there's anything to show at all.
 */
class TourStore {
	private _active = $state<Tour | null>(null);

	get active(): Tour | null {
		return this._active;
	}

	/** Called once from a page's `useTour(tour)`. Auto-plays the first time this browser sees it. */
	activate(tour: Tour) {
		this._active = tour;
		if (!hasSeen(tour.id)) {
			markSeen(tour.id);
			// Wait a tick so the page has finished rendering before driver.js measures elements.
			requestAnimationFrame(() => runDriver(tour));
		}
	}

	/** Called on unmount. Guards on id so a fast route change can't clear the next page's tour. */
	deactivate(id: string) {
		if (this._active?.id === id) this._active = null;
	}

	replay() {
		if (this._active) runDriver(this._active);
	}
}

export const tourStore = new TourStore();
