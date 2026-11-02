// API calls a button makes before it can navigate anywhere, so the global progress bar
// can show for them too. Navigation itself is covered by SvelteKit's `navigating`.
class PendingWork {
	private count = $state(0);

	get busy(): boolean {
		return this.count > 0;
	}

	start() {
		this.count += 1;
	}

	stop() {
		this.count = Math.max(0, this.count - 1);
	}
}

export const pendingWork = new PendingWork();
