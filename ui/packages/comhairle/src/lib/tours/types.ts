import type { Alignment, Side } from 'driver.js';

/**
 * One beat of a tour: a control on the screen and the sentence that explains it.
 *
 * The control is named, not selected. A tour says `target: 'brief'` and the markup carries
 * `data-tour="brief"`, so a route declares its tour without knowing anything about how the
 * chrome is built, and moving a control does not break the tour that points at it.
 */
export type TourStop = {
	/** The `data-tour` name of the element this stop circles. */
	target: string;
	/**
	 * The caption. A function, not a string, so paraglide resolves it when the tour runs and
	 * a definition can live at module scope.
	 */
	text: () => string;
	/** Which side of the control the card sits on. Left out, driver.js picks. */
	side?: Side;
	align?: Alignment;
	/**
	 * Opens whatever the stop points inside, e.g. the step menu for a control that only
	 * exists once the menu is up. Paired with `waitMs`, since the target mounts a frame or
	 * two later.
	 */
	before?: () => void;
	/**
	 * This beat's control mounts after the tour starts, so the runner must not look for it
	 * before drawing. Whether the beat belongs at all is the caller's decision, made from the
	 * same config that decides whether the control renders, rather than read off the DOM: a
	 * tour that polls for a late control cannot draw its first beat until the polling gives
	 * up, and that gap is long enough to start doing something and be interrupted.
	 */
	mountsLate?: boolean;
};

export type Tour = {
	/**
	 * Stable across releases: it is half of the storage key that remembers a dismissal, so
	 * renaming it shows the tour again to everyone who has already seen it.
	 */
	id: string;
	stops: TourStop[];
};
