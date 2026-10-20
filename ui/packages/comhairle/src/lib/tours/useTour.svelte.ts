import { tourStore } from './tourStore.svelte';
import type { Tour } from './types';

/** Call from a page's `<script>` to register its tour: auto-plays on first visit, and makes it
 * available to the "replay tour" button for as long as the page is mounted.
 *
 * Pass a getter (`() => tour`) rather than a plain value when the tour depends on reactive state
 * (e.g. which engagement tool a step uses) — a step route doesn't remount on a param change, so
 * only a getter re-activates the right tour when that state changes. A getter that returns
 * null/undefined deactivates any tour this call previously registered. */
export function useTour(getTour: () => Tour | null | undefined) {
	$effect(() => {
		const tour = getTour();
		if (!tour) return;
		tourStore.activate(tour);
		return () => tourStore.deactivate(tour.id);
	});
}
