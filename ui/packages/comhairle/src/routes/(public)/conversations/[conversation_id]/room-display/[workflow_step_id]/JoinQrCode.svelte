<!--
	@component The join QR code. Until `svelte-qrcode` draws, it renders an empty <img> whose alt
	text (the URL) flashes on screen, so this sizes the box itself and shows a skeleton instead.
-->
<script lang="ts">
	import { onMount } from 'svelte';
	import QrCode from 'svelte-qrcode';
	import { Skeleton } from '$lib/components/ui/skeleton';

	type Props = {
		value: string;
		/** Bitmap size in px. Larger than the box so the code stays sharp when projected. */
		resolution?: number;
		class?: string;
	};

	let { value, resolution = 512, class: className = '' }: Props = $props();

	let host = $state<HTMLDivElement | null>(null);
	let ready = $state(false);

	onMount(() => {
		const image = host?.querySelector('img');
		if (!image) return;
		// A child's onMount runs before its parent's, so the library has usually set src
		// already and the load event has passed. The listener covers the case where it has not.
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
		<!-- Tinted black, not `bg-accent`: the QR code always sits on white, in both themes. -->
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
