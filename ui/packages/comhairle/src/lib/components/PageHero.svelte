<script lang="ts">
	import type { Snippet } from 'svelte';
	import { cn } from '$lib/utils';

	type Props = {
		children: Snippet;
		/** Less room below the content, for pages whose hero is only a heading. */
		compact?: boolean;
	};
	let { children, compact = false }: Props = $props();
</script>

<!-- Pulled up by the NavBar's height so the gradient runs behind the see-through bar; the top
	padding adds that height back. The gradient comes from each theme's accent colour, with two
	see-through white shapes for texture. Text is dark, so it reads at WCAG AA throughout. -->
<section
	class="page-hero text-foreground relative isolate -mt-[5.5rem] overflow-hidden rounded-b-[2rem] md:rounded-b-[3rem]"
>
	<div aria-hidden="true" class="page-hero-shape page-hero-shape-a"></div>
	<div aria-hidden="true" class="page-hero-shape page-hero-shape-b"></div>
	<div
		class={cn(
			'mx-auto w-full max-w-[1300px] px-4 md:px-8',
			compact
				? 'pt-[8.5rem] pb-12 md:pt-[10rem] md:pb-16'
				: 'pt-[9.5rem] pb-16 md:pt-[12.5rem] md:pb-28'
		)}
	>
		<div class="mx-auto flex w-full max-w-6xl flex-col items-start gap-8">
			{@render children()}
		</div>
	</div>
</section>

<style>
	.page-hero {
		background: linear-gradient(
			to bottom,
			color-mix(in oklab, var(--sidebar-primary) 8%, var(--background)),
			color-mix(in oklab, var(--sidebar-primary) 50%, var(--background))
		);
	}
	/* Two big see-through planes on the right, overlapping like light through a window. */
	.page-hero-shape {
		position: absolute;
		z-index: -1;
		inset: 0;
		pointer-events: none;
	}
	.page-hero-shape-a {
		background: rgb(255 255 255 / 0.3);
		clip-path: polygon(100% 25%, 100% 100%, 30% 100%);
	}
	.page-hero-shape-b {
		background: rgb(255 255 255 / 0.2);
		clip-path: polygon(72% 0%, 100% 0%, 100% 100%, 60% 100%);
	}
</style>
