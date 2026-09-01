<script lang="ts">
	/**
	 * Before you start: everything about a conversation that is not the cover, as pages a
	 * participant reads one screen at a time (ADR-0024).
	 *
	 * Each page owns a screen, so a chip is a place rather than a point in a wall of text.
	 * The pages are still stacked in the ordinary scroll, so scrolling down walks them in
	 * order and the chips are the shortcut, not the only way through.
	 */
	import type { ComhairleDocument } from '@crownshy/api-client/api';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import SlideDots from '$lib/components/participant/SlideDots.svelte';
	import { carouselSwipe } from '$lib/components/participant/carouselSwipe';
	import type { BeforeYouStartPage } from '$lib/components/participant/beforeYouStart';
	import type { StepPreview } from '$lib/components/participant/stepPreview';
	import { cn } from '$lib/utils';
	import * as m from '$lib/paraglide/messages';

	let {
		pages,
		steps,
		conversationId,
		availableDocuments = []
	}: {
		pages: BeforeYouStartPage[];
		steps: StepPreview[];
		conversationId: string;
		availableDocuments?: ComhairleDocument[];
	} = $props();

	// Which chip is lit. Without it the strip is a row of identical buttons and reads as a
	// menu rather than as a position in the deck.
	let activeIndex = $state(0);

	function show(index: number) {
		if (index < 0 || index >= pages.length) return;
		document.getElementById(pages[index].id)?.scrollIntoView({ behavior: 'smooth' });
	}

	// Swiping is the same move as scrolling to the next page, for a thumb that is already
	// holding the phone. Vertical drags are left alone, so it never fights the scroll.
	const swipe = carouselSwipe(
		() => show(activeIndex - 1),
		() => show(activeIndex + 1)
	);

	$effect(() => {
		const nodes = pages
			.map((page) => document.getElementById(page.id))
			.filter((node): node is HTMLElement => !!node);
		if (!nodes.length) return;

		const observer = new IntersectionObserver(
			(entries) => {
				const visible = entries.find((entry) => entry.isIntersecting);
				if (!visible) return;
				const index = pages.findIndex((page) => page.id === visible.target.id);
				if (index >= 0) activeIndex = index;
			},
			// A band just under the sticky strip, so exactly one page is active at a time.
			{ rootMargin: '-96px 0px -70% 0px' }
		);
		nodes.forEach((node) => observer.observe(node));
		return () => observer.disconnect();
	});
</script>

{#if pages.length}
	<div id="conversation-detail">
		<div class="bg-background/95 sticky top-0 z-20 border-y backdrop-blur">
			<nav class="mx-auto flex w-full max-w-5xl gap-2 overflow-x-auto px-5 py-3 md:px-6">
				{#each pages as page, index (page.id)}
					<button
						type="button"
						class="max-w-[12rem] shrink-0 truncate rounded-full px-4 py-1.5 text-sm font-medium transition-colors {index ===
						activeIndex
							? 'bg-foreground text-background'
							: 'bg-accent text-accent-foreground'}"
						aria-current={index === activeIndex ? 'page' : undefined}
						onclick={() => show(index)}
					>
						{page.label}
					</button>
				{/each}
			</nav>
		</div>

		<!-- The last page reserves the call to action's height. The ones above it do not: their
			own bottom padding plus the next page's top padding is already more than enough. -->
		{#each pages as page, index (page.id)}
			<section
				id={page.id}
				class={cn(
					'mx-auto flex min-h-[100dvh] w-full max-w-5xl scroll-mt-24 flex-col px-5 pt-10 pb-16 md:px-6',
					index === pages.length - 1 && 'pb-32'
				)}
				aria-label={page.label}
				onpointerdown={swipe.onpointerdown}
				onpointerup={swipe.onpointerup}
				onpointercancel={swipe.onpointercancel}
			>
				<div class="grow">
					{#if page.heading}
						<h2 class="mb-4 text-2xl font-semibold">{page.heading}</h2>
					{/if}

					{#if page.kind === 'steps'}
						<ol class="flex flex-col gap-4">
							{#each steps as step, stepIndex (step.id)}
								{@const StepIcon = step.icon}
								<li class="flex items-center gap-4">
									<span
										class="bg-accent text-accent-foreground flex size-10 shrink-0 items-center justify-center rounded-full"
									>
										{#if StepIcon}
											<StepIcon class="size-5" aria-hidden="true" />
										{:else}
											<span class="text-sm font-medium">{stepIndex + 1}</span>
										{/if}
									</span>
									<span class="min-w-0 flex-1 text-base">{step.name}</span>
									<span class="text-muted-foreground shrink-0 text-sm">
										{#if step.minutes}{m.landing_step_minutes({
												count: step.minutes
											})}{/if}
										{#if step.optional}{step.minutes
												? ' · '
												: ''}{m.landing_step_optional()}{/if}
									</span>
								</li>
							{/each}
						</ol>
					{:else if page.content}
						<div class="prose prose-p:text-base w-full max-w-none text-base">
							<ContentRenderer
								content={page.content}
								{availableDocuments}
								{conversationId}
							/>
						</div>
					{/if}
				</div>

				{#if pages.length > 1}
					<div class="flex flex-col items-center gap-2 pt-10">
						<SlideDots {index} count={pages.length} />
						<p class="sr-only">
							{m.landing_page_of({ current: index + 1, total: pages.length })}
						</p>
					</div>
				{/if}
			</section>
		{/each}
	</div>
{/if}
