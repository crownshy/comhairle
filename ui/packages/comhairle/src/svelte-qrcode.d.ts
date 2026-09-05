/**
 * `svelte-qrcode` ships a Svelte 3 component with no types, so every import of it is
 * a type error. Props mirror `src/lib/index.svelte` in the package.
 */
declare module 'svelte-qrcode' {
	import type { Component } from 'svelte';

	const QrCode: Component<{
		value: string;
		/** Rendered pixel size, as a string in the package's own API. */
		size?: string | number;
		padding?: number | null;
		errorCorrection?: 'L' | 'M' | 'Q' | 'H';
		background?: string;
		color?: string;
		className?: string;
	}>;

	export default QrCode;
}
