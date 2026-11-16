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
	in the theme colour that carries on from the top bar, with rounded bottom corners. -->
<section
	aria-labelledby="pitch-heading"
	class="bg-primary text-primary-foreground ml-[calc(50%-50vw)] w-screen rounded-b-[2rem] md:rounded-b-[3rem]"
>
	<div
		class="mx-auto flex w-full max-w-[1300px] flex-col items-start gap-6 px-4 pt-14 pb-16 md:px-8 md:pt-24 md:pb-28"
	>
		<div class="mx-auto flex w-full max-w-6xl flex-col items-start gap-6">
			<div class="flex max-w-3xl flex-col gap-3">
				<h1
					id="pitch-heading"
					class="text-[1.75rem] leading-tight font-bold text-balance sm:text-4xl lg:text-[2.625rem]"
				>
					Join a conversation and help shape better decisions
				</h1>
				<p class="text-lg text-pretty opacity-90 sm:text-xl">
					Learn about a topic, share your views and hear what others think.
				</p>
			</div>
			{#if showAction}
				<Button
					href="#all"
					onclick={scrollToList}
					variant="outline"
					class="text-primary h-12 w-full gap-2 rounded-full border-transparent bg-white px-6 text-base font-semibold hover:bg-white/90 sm:w-auto"
				>
					Explore conversations
					<ArrowDown class="size-5" />
				</Button>
			{/if}
		</div>
	</div>
</section>
