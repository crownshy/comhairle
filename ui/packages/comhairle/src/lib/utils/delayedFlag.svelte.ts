/**
 * A boolean that mirrors `source` but only flips to `true` after `source` has
 * stayed truthy for `delayMs`. Flipping back to `false` is immediate.
 *
 * The point is loading skeletons: a navigation that resolves faster than
 * `delayMs` never trips the flag, so its skeleton is never rendered and can't
 * flash. A genuinely slow load still trips it and shows the skeleton, where the
 * feedback is wanted. Must be called during component init (it owns an `$effect`).
 *
 * @param source - getter for the underlying flag (e.g. `() => switchingSection`)
 * @param delayMs - how long `source` must stay truthy before the flag flips
 * @returns an object whose `current` getter is the delayed flag
 */
export function delayedFlag(source: () => boolean, delayMs = 150) {
	let value = $state(false);

	$effect(() => {
		if (!source()) {
			value = false;
			return;
		}
		const timer = setTimeout(() => (value = true), delayMs);
		return () => clearTimeout(timer);
	});

	return {
		get current() {
			return value;
		}
	};
}
