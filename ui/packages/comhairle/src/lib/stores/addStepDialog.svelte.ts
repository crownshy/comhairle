/**
 * Cross-component signal for opening the add-step dialog.
 *
 * The dialog is owned by the design layout (`design/+layout.svelte`), but it can be
 * opened from elsewhere in the design route (the board's empty state / footer button,
 * the workflow step strip, or an `?addStep=true` deep link). Rather than thread a
 * callback through the tab-extras context, callers bump a request counter and the
 * layout reacts to it. A monotonically increasing counter (not a boolean) lets the
 * layout re-open the dialog even if it was opened, closed, then requested again.
 */
let requestCount = $state(0);

export const addStepDialog = {
	/** Read by the layout in an `$effect` to know when a new open was requested. */
	get requestCount() {
		return requestCount;
	},
	/** Ask the layout to open the add-step dialog. */
	request() {
		requestCount += 1;
	}
};
