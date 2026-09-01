<script lang="ts">
	/**
	 * Before you start: everything about a conversation that is not the cover, as pages a
	 * participant reads one screen at a time (ADR-0024).
	 *
	 * The pages sit in the ordinary scroll and the scroll snaps to them, so one swipe down is
	 * one page and a page is never left half on screen. The chips are the shortcut, not the
	 * only way through.
	 */
	import type { ComhairleDocument } from '@crownshy/api-client/api';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import { carouselSwipe } from '$lib/components/participant/carouselSwipe';
	import type { BeforeYouStartPage } from '$lib/components/participant/beforeYouStart';
	import type { StepPreview } from '$lib/components/participant/stepPreview';
	import { ChevronDown } from 'lucide-svelte';
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

	/**
	 * Snapping belongs to the scroll container, which for this page is the document, so it
	 * has to go on the root element rather than on the deck. Scoped to the page's lifetime:
	 * every other route scrolls normally.
	 *
	 * The cover carries a snap point of its own, so landing at the top of the page is already
	 * on one and nothing jumps out from under the reader on load.
	 */
	$effect(() => {
		if (!pages.length) return;
		document.documentElement.classList.add('deck-snap');
		return () => document.documentElement.classList.remove('deck-snap');
	});

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
			<!-- Centred from the breakpoint up, to sit over the centred column. Left aligned
				below it, where the strip scrolls and has to start on the first chip. -->
			<nav
				class="mx-auto flex w-full max-w-5xl gap-2 overflow-x-auto px-5 py-3 md:justify-center md:px-6"
			>
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

		<!-- Every page reserves the call to action's height at the foot and the sticky strip's
			at the head, so a snapped page sits clear of both and the cue is always in view. -->
		{#each pages as page, index (page.id)}
			<section
				id={page.id}
				class="mx-auto flex min-h-[100dvh] w-full max-w-5xl snap-start flex-col px-5 pt-20 pb-24 md:px-6"
				aria-label={page.label}
				onpointerdown={swipe.onpointerdown}
				onpointerup={swipe.onpointerup}
				onpointercancel={swipe.onpointercancel}
			>
				<!-- `m-auto` rather than centring the section: a page taller than the screen
					then grows downwards instead of losing its first lines off the top.
					The column is centred and capped at `max-w-prose`, but its text stays ragged
					right: centred body text costs the reader the left edge they return to. -->
				<div class="m-auto w-full max-w-prose">
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

				<p class="sr-only">
					{m.landing_page_of({ current: index + 1, total: pages.length })}
				</p>

				{#if index < pages.length - 1}
					<div class="flex shrink-0 justify-center pt-6">
						<button
							type="button"
							class="text-muted-foreground hover:text-foreground flex size-11 items-center justify-center rounded-full transition-colors"
							onclick={() => show(index + 1)}
							aria-label={m.landing_next_page({ label: pages[index + 1].label })}
						>
							<span class="scroll-cue flex">
								<ChevronDown class="size-6" aria-hidden="true" />
							</span>
						</button>
					</div>
				{/if}
			</section>
		{/each}
	</div>
{/if}

<style>
	/* Mandatory rather than proximity: one swipe should land on a page rather than between
	   two. A page longer than the screen still scrolls freely, because its only snap point
	   is its top. */
	:global(html.deck-snap) {
		scroll-snap-type: y mandatory;
	}

	/* The cue is the only thing on the page that moves, so it reads as "keep going" rather
	   than as decoration. */
	.scroll-cue {
		animation: scroll-cue-nudge 2.4s ease-in-out infinite;
	}

	@keyframes scroll-cue-nudge {
		0%,
		100% {
			transform: translateY(0);
			opacity: 0.5;
		}
		50% {
			transform: translateY(5px);
			opacity: 1;
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.scroll-cue {
			animation: none;
			opacity: 0.7;
		}
	}
</style>
