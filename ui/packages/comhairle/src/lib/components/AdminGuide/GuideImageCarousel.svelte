<script lang="ts">
	import { ChevronLeft, ChevronRight } from '@lucide/svelte';
	import * as Carousel from '$lib/components/ui/carousel';
	import type { CarouselAPI } from '$lib/components/ui/carousel/context';
	import type { AdminGuideImage } from '$lib/admin_guides';
	import type { Snippet } from 'svelte';

	type Props = {
		images: AdminGuideImage[];
		/** Renders a caption, so it can use the guide's **bold** and link formatting. */
		caption: Snippet<[string]>;
		class?: string;
		/** Index of the image shown. Bind to it to move the carousel from outside. */
		current?: number;
	};

	let { images, caption, class: className = '', current = $bindable(0) }: Props = $props();

	let api = $state<CarouselAPI>();

	// Embla tells us when the slide changes (chevrons, swipe or arrow keys).
	$effect(() => {
		if (!api) return;
		const onSelect = () => (current = api!.selectedScrollSnap());
		onSelect();
		api.on('select', onSelect);
		return () => api?.off('select', onSelect);
	});

	// …and when something outside sets `current` (e.g. hovering a step), slide there.
	$effect(() => {
		if (api && api.selectedScrollSnap() !== current) api.scrollTo(current);
	});

	let currentImage = $derived(images[current]);
	const chevron =
		'bg-card/90 text-foreground border-border hover:bg-card absolute top-1/2 z-10 flex size-9 -translate-y-1/2 items-center justify-center rounded-full border shadow-sm transition-colors disabled:pointer-events-none disabled:opacity-0';
</script>

<figure class={className} aria-label={`Screenshots, ${current + 1} of ${images.length}`}>
	<Carousel.Root setApi={(a) => (api = a)} class="w-full">
		<div class="border-border bg-muted w-full overflow-hidden rounded-xl border p-2 shadow-sm">
			<Carousel.Content class="-ml-2">
				{#each images as image, i (image.src)}
					<Carousel.Item class="pl-2">
						<img
							src={image.src}
							alt={image.alt}
							class="block h-auto w-full rounded-xl"
							aria-hidden={i !== current}
						/>
					</Carousel.Item>
				{/each}
			</Carousel.Content>
		</div>

		<button
			type="button"
			class="{chevron} left-4"
			disabled={current === 0}
			onclick={() => api?.scrollPrev()}
			aria-label="Previous screenshot"
		>
			<ChevronLeft class="size-5" aria-hidden="true" />
		</button>
		<button
			type="button"
			class="{chevron} right-4"
			disabled={current === images.length - 1}
			onclick={() => api?.scrollNext()}
			aria-label="Next screenshot"
		>
			<ChevronRight class="size-5" aria-hidden="true" />
		</button>
	</Carousel.Root>

	{#if currentImage?.caption}
		<figcaption class="text-muted-foreground mt-2 text-center text-sm" aria-live="polite">
			{@render caption(currentImage.caption)}
		</figcaption>
	{/if}
	<div class="mt-2 flex items-center justify-center gap-1.5">
		{#each images as image, i (image.src)}
			<button
				type="button"
				class="size-2 rounded-full transition-colors {i === current
					? 'bg-primary'
					: 'bg-border hover:bg-muted-foreground'}"
				aria-label={`Show screenshot ${i + 1} of ${images.length}`}
				aria-current={i === current}
				onclick={() => api?.scrollTo(i)}
			></button>
		{/each}
	</div>
</figure>
