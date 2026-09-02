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

/**
 * The scale for one screen, fitted to the height it has.
 *
 * Height is the binding constraint: the view is a strip across the bottom of the page, so
 * the screens run along it and it is the sheet's depth that decides how big they can be.
 * Width then costs nothing, which is the point of the strip. Nothing is scaled up past life
 * size, so expanding to full height stops at 1 rather than blowing a phone up to a wall.
 */
export function screenScale(options: { device: Device; availableHeight: number }): number {
	const { device, availableHeight } = options;
	if (availableHeight <= 0) return 1;
	return Math.min(1, availableHeight / DEVICE_SIZES[device].height);
}
