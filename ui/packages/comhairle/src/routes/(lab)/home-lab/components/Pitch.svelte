<script lang="ts">
	import { ArrowDown } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';

	type Props = {
		/** Hide the button when there's no list to jump to (empty state). */
		showAction: boolean;
	};
	let { showAction }: Props = $props();

	// Glide down to the list rather than jump, so people see what they pass on the way. People
	// who ask their device to reduce motion still jump straight there. Focus moves to the list
	// heading so keyboard and screen reader users land in the same place.
	function scrollToList(event: MouseEvent) {
		const list = document.getElementById('all');
		if (!list) return;
		event.preventDefault();
		const reduceMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
		list.scrollIntoView({ behavior: reduceMotion ? 'auto' : 'smooth', block: 'start' });
		document.getElementById('all-heading')?.focus({ preventScroll: true });
	}
</script>

<!-- For visitors who don't know the platform yet; logged-in people skip it. A full-width block
	with rounded bottom corners that runs up behind the see-through top bar. Its light gradient
	comes from each theme's accent colour, with two soft see-through shapes for a bit of
	texture. Text is dark, so it reads at WCAG AA on the lightest and darkest parts. -->
<section
	aria-labelledby="pitch-heading"
	class="pitch text-foreground relative isolate ml-[calc(50%-50vw)] w-screen overflow-hidden rounded-b-[2rem] md:rounded-b-[3rem]"
>
	<div aria-hidden="true" class="pitch-shape pitch-shape-a"></div>
	<div aria-hidden="true" class="pitch-shape pitch-shape-b"></div>

	<!-- Top padding includes the top bar's height (4rem, 5rem from md), which sits on top. -->
	<div
		class="mx-auto flex w-full max-w-[1300px] flex-col items-start gap-6 px-4 pt-[7.5rem] pb-16 md:px-8 md:pt-[11rem] md:pb-28"
	>
		<div class="mx-auto flex w-full max-w-6xl flex-col items-start gap-8">
			<div class="flex max-w-3xl flex-col gap-4">
				<h1
					id="pitch-heading"
					class="text-[1.75rem] leading-tight font-bold text-balance sm:text-4xl lg:text-[2.625rem]"
				>
					Join a conversation and help shape better decisions
				</h1>
				<p class="text-lg text-pretty sm:text-xl">
					Learn about a topic, share your views and hear what others think.
				</p>
			</div>
			{#if showAction}
				<Button
					href="#all"
					onclick={scrollToList}
					class="h-12 w-full gap-2 rounded-full px-6 text-base font-semibold sm:w-auto"
				>
					Explore conversations
					<ArrowDown class="size-5" />
				</Button>
			{/if}
		</div>
	</div>
</section>

<style>
	.pitch {
		background: linear-gradient(
			to bottom,
			color-mix(in oklab, var(--sidebar-primary) 8%, var(--background)),
			color-mix(in oklab, var(--sidebar-primary) 50%, var(--background))
		);
	}
	/* Two big see-through planes on the right, overlapping like light through a window. */
	.pitch-shape {
		position: absolute;
		z-index: -1;
		inset: 0;
		pointer-events: none;
	}
	.pitch-shape-a {
		background: rgb(255 255 255 / 0.3);
		clip-path: polygon(100% 25%, 100% 100%, 30% 100%);
	}
	.pitch-shape-b {
		background: rgb(255 255 255 / 0.2);
		clip-path: polygon(72% 0%, 100% 0%, 100% 100%, 60% 100%);
	}
</style>
