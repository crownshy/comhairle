/**
 * Shared shape of a participant view (ADR-0030): the device sizes a participant screen is
 * rendered at, and how far it is scaled down to fit the space it is shown in.
 */

/**
 * Real device sizes, because the point is that the layout inside computes exactly as it
 * would on the device. Scaling happens afterwards, to the finished box.
 */
export const DEVICE_SIZES = {
	phone: { width: 390, height: 844 },
	desktop: { width: 1400, height: 900 }
} as const;

export type Device = keyof typeof DEVICE_SIZES;

/** How far a phone shrinks in the docked panel, chosen so several slides sit together. */
const DOCK_PHONE_SCALE = 0.45;

/**
 * The scale for one screen.
 *
 * A phone holds a fixed contact-sheet scale in the dock and goes full size when expanded:
 * on Configure the question is where the slide breaks fell, which is about the set rather
 * than any one slide. A desktop screen is always fitted to the width it has, because 1400px
 * never fits anywhere at full size and a fixed fraction of it would be arbitrary.
 */
export function screenScale(options: {
	device: Device;
	available: number;
	expanded: boolean;
}): number {
	const { device, available, expanded } = options;
	if (device === 'phone') return expanded ? 1 : DOCK_PHONE_SCALE;
	if (available <= 0) return DOCK_PHONE_SCALE;
	return Math.min(1, available / DEVICE_SIZES.desktop.width);
}
