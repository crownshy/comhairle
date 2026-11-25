export type TourStep = {
	/** CSS selector for the element this step points at. Must match something that's always
	 * on screen when the page finishes loading; don't target content behind an await or a tab. */
	element: string;
	title: string;
	description: string;
	side?: 'top' | 'right' | 'bottom' | 'left';
	align?: 'start' | 'center' | 'end';
};

export type Tour = {
	/** Unique across the app. Used as the "seen" key, so changing it replays the tour for everyone. */
	id: string;
	steps: TourStep[];
	/** Label for the button that ends the tour. Defaults to driver.js's "Done". */
	doneLabel?: string;
};

export type TourOptions = {
	/** Off when the page has its own first-visit welcome, so the tour waits for a replay. */
	autoplay?: boolean;
};
