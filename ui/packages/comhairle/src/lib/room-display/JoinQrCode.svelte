<!--
	@component The join QR code, with a placeholder that holds its place until it draws.

	`svelte-qrcode` builds the code in its own `onMount` and renders `<img src="">`
	until then, which is what the server sends and what the browser paints on the first
	frame. An empty `src` is a broken image, and this one's `alt` is the join URL, so
	the wall showed a line of raw URL text that then snapped into a QR code, resizing
	whatever sat around it on the way. On the recruitment screen that is the first
	thing the room ever sees.

	So the box is sized here rather than by the image, which removes the reflow, and a
	skeleton fills it until the code is actually drawn. The skeleton renders
	server-side too: that is the part that covers the first frame, and the reason this
	is a component rather than a class on each call site.
-->
<script lang="ts">
	import { onMount } from 'svelte';
	import QrCode from 'svelte-qrcode';
	import { Skeleton } from '$lib/components/ui/skeleton';

	type Props = {
		/** Where the code points. */
		value: string;
		/**
		 * Bitmap size in px. Deliberately larger than the box it is drawn into, so the
		 * code stays crisp when a projector scales it up.
		 */
		resolution?: number;
		/** Tailwind sizing for the box, e.g. `size-24 lg:size-32`. */
		class?: string;
	};

	let { value, resolution = 512, class: className = '' }: Props = $props();

	let host = $state<HTMLDivElement | null>(null);
	let ready = $state(false);

	onMount(() => {
		const image = host?.querySelector('img');
		if (!image) return;
		// Svelte runs a child's onMount before its parent's, so the library has usually
		// filled the src in by now and there is no load event left to wait for. The
		// listener is for the case where it has not.
		if (image.getAttribute('src')) {
			ready = true;
			return;
		}
		const done = () => (ready = true);
		image.addEventListener('load', done);
		return () => image.removeEventListener('load', done);
	});
</script>

<div bind:this={host} class="relative {className}">
	{#if !ready}
		<!--
			Tinted rather than the default `bg-accent`: this always sits on the white
			card the code needs to stay scannable, so it has to read against white in
			both themes.
		-->
		<Skeleton
			class="absolute inset-0 size-full rounded-md bg-black/10 motion-reduce:animate-none"
		/>
	{/if}
	<div
		class="size-full transition-opacity duration-300 motion-reduce:transition-none"
		class:opacity-0={!ready}
	>
		<QrCode
			{value}
			size={String(resolution)}
			padding={null}
			errorCorrection="M"
			className="size-full"
		/>
	</div>
</div>
