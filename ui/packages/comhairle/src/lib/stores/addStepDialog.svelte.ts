/**
 * Cross-component open state for the add-step dialog.
 *
 * The dialog is rendered once by the design layout (`design/+layout.svelte`), but it is
 * opened from several places in the design route (the board's empty state / footer
 * button, the workflow step strip, or an `?addStep=true` deep link). Rather than thread
 * a callback through the tab-extras context, this store owns the `open` boolean as the
 * single source of truth: the layout binds the dialog to it, and any opener just sets
 * `addStepDialog.open = true`. The dialog closes itself back through the same binding.
 */
let open = $state(false);

export const addStepDialog = {
	/** Two-way open state; bound by the layout's `<AddStepDialog bind:open>`. */
	get open() {
		return open;
	},
	set open(value: boolean) {
		open = value;
	}
};
