/**
 * Work that should show the global progress bar but is not itself a navigation:
 * the API calls a button makes before it can `goto` anywhere. Navigation is
 * already covered by SvelteKit's `navigating`.
 */
class RouteProgress {
	private _pending = $state(0);

	get busy(): boolean {
		return this._pending > 0;
	}

	start() {
		this._pending += 1;
	}

	stop() {
		this._pending = Math.max(0, this._pending - 1);
	}
}

export const routeProgress = new RouteProgress();
