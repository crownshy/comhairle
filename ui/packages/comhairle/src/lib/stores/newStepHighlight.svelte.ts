/**
 * Cross-component signal naming the freshly-created step to highlight on the board.
 *
 * The add-step dialog lives in the design layout, but the step cards it creates render
 * on the board child route. After a successful create, the dialog flags the new step's
 * id here; the board scrolls to it, applies a transient highlight ring, then clears it.
 */
let id = $state<string | null>(null);

export const newStepHighlight = {
	/** The id of the step to highlight, or `null` when nothing is pending. */
	get id() {
		return id;
	},
	/** Flag a newly-created step for the board to highlight. */
	flag(stepId: string) {
		id = stepId;
	},
	/** Clear the highlight once the board has acknowledged it. */
	clear() {
		id = null;
	}
};
