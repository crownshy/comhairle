<script lang="ts">
	import { tick } from 'svelte';
	import { Button } from '$lib/components/ui/button';
	import { cn } from '$lib/utils';
	import MinusIcon from '@lucide/svelte/icons/minus';
	import PlusIcon from '@lucide/svelte/icons/plus';

	type Props = { src: string; alt: string };

	let { src, alt }: Props = $props();

	// Either fit to the viewer or an explicit scale (1 === 100% of the image's natural size).
	type Zoom = 'fit' | number;

	const MIN_ZOOM = 0.25;
	const MAX_ZOOM = 4;
	const ZOOM_STEP = 1.25;

	let zoom = $state<Zoom>('fit');
	let image = $state<HTMLImageElement | null>(null);
	let scrollContainer = $state<HTMLDivElement | null>(null);
	let naturalWidth = $state(0);

	const zoomLabel = $derived(zoom === 'fit' ? 'Fit' : `${Math.round(zoom * 100)}%`);

	function currentScale(): number {
		if (zoom !== 'fit') return zoom;
		if (!image || !naturalWidth) return 1;
		return image.clientWidth / naturalWidth;
	}

	function clampZoom(scale: number): number {
		return Math.min(Math.max(scale, MIN_ZOOM), MAX_ZOOM);
	}

	function zoomIn() {
		zoom = clampZoom(currentScale() * ZOOM_STEP);
	}

	function zoomOut() {
		zoom = clampZoom(currentScale() / ZOOM_STEP);
	}

	// Clicking toggles between fit and a zoom large enough to read small print, keeping the
	// clicked spot under the pointer so the reader lands on what they were trying to read.
	async function toggleZoom(event: MouseEvent) {
		if (zoom !== 'fit' || !image || !scrollContainer) {
			zoom = 'fit';
			return;
		}
		const rect = image.getBoundingClientRect();
		const xRatio = (event.clientX - rect.left) / rect.width;
		const yRatio = (event.clientY - rect.top) / rect.height;
		const pointerX = event.clientX - scrollContainer.getBoundingClientRect().left;
		const pointerY = event.clientY - scrollContainer.getBoundingClientRect().top;

		zoom = clampZoom(Math.max(1, currentScale() * 2));
		await tick();

		scrollContainer.scrollLeft = image.offsetLeft + image.clientWidth * xRatio - pointerX;
		scrollContainer.scrollTop = image.offsetTop + image.clientHeight * yRatio - pointerY;
	}
</script>

<div class="flex h-full w-full flex-col">
	<div class="border-border bg-background flex items-center justify-end gap-2 border-b px-4 py-2">
		<Button
			variant="outline"
			size="sm"
			aria-label="Zoom out"
			title="Zoom out"
			disabled={!naturalWidth || (zoom !== 'fit' && zoom <= MIN_ZOOM)}
			onclick={zoomOut}
		>
			<MinusIcon class="size-4" />
		</Button>
		<Button
			variant="outline"
			size="sm"
			class="min-w-18"
			aria-label="Fit to screen"
			title="Fit to screen"
			disabled={zoom === 'fit'}
			onclick={() => (zoom = 'fit')}
		>
			{zoomLabel}
		</Button>
		<Button
			variant="outline"
			size="sm"
			aria-label="Zoom in"
			title="Zoom in"
			disabled={!naturalWidth || (zoom !== 'fit' && zoom >= MAX_ZOOM)}
			onclick={zoomIn}
		>
			<PlusIcon class="size-4" />
		</Button>
	</div>

	<!-- m-auto on the image (rather than flex centring) keeps a zoomed image scrollable to
		its left and top edges. -->
	<div bind:this={scrollContainer} class="flex min-h-0 flex-1 overflow-auto p-4">
		<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_noninteractive_element_interactions -->
		<img
			bind:this={image}
			{src}
			{alt}
			bind:naturalWidth
			onclick={toggleZoom}
			style:width={zoom === 'fit' ? undefined : `${naturalWidth * zoom}px`}
			class={cn(
				'm-auto',
				zoom === 'fit'
					? 'max-h-full max-w-full cursor-zoom-in object-contain'
					: 'max-w-none cursor-zoom-out'
			)}
		/>
	</div>
</div>
