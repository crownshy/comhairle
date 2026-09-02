/**
 * How much shorter the visible viewport has to get before we call it a keyboard. Phone
 * keyboards take a third of the screen; the browser's own collapsing toolbars move far less
 * than this, so the gap between the two is wide and the threshold does not need to be clever.
 */
const KEYBOARD_MIN_PX = 150;

/**
 * Whether an on-screen keyboard is currently covering part of the screen.
 *
 * `dvh` follows the browser's collapsing chrome but not the keyboard: on iOS the layout
 * viewport keeps its full height and the keyboard is drawn over it, so a `100dvh` shell
 * silently spends its bottom rows on pixels nobody can see. The visual viewport is the only
 * thing that shrinks, so it is the only thing worth asking.
 *
 * Must be called during component init (it owns an `$effect`). On a desktop, and anywhere
 * `visualViewport` is missing, it stays false.
 *
 * @returns an object whose `current` getter is true while the keyboard is up
 */
export function keyboardOpen() {
	let value = $state(false);

	$effect(() => {
		const viewport = window.visualViewport;
		if (!viewport) return;
		const update = () => {
			value = window.innerHeight - viewport.height > KEYBOARD_MIN_PX;
		};
		update();
		viewport.addEventListener('resize', update);
		return () => viewport.removeEventListener('resize', update);
	});

	return {
		get current() {
			return value;
		}
	};
}
