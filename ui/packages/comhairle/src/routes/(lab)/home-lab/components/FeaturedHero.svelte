<script lang="ts">
	import { ArrowRight, ChevronLeft, ChevronRight, MessageSquareText, Star } from 'lucide-svelte';
	import * as Carousel from '$lib/components/ui/carousel';
	import type { CarouselAPI } from '$lib/components/ui/carousel/context';
	import { cn } from '$lib/utils';
	import type { LabConversation } from '../mockData';

	type Props = {
		conversations: LabConversation[];
		hrefFor: (id: string) => string;
		/** The label for each slide's main button, e.g. Take part or Continue. */
		actionFor: (conversation: LabConversation) => string;
	};
	let { conversations, hrefFor, actionFor }: Props = $props();

	let api = $state<CarouselAPI>();
	let selected = $state(0);

	function connect(next: CarouselAPI | undefined) {
		api = next;
		next?.on('select', () => (selected = next.selectedScrollSnap()));
	}
</script>

<!-- Links are resolve() plus the scenario query string, which this rule can't see. -->
<!-- eslint-disable svelte/no-navigation-without-resolve -->

<section
	aria-roledescription="carousel"
	aria-label="Featured conversations"
	class="flex flex-col gap-3"
>
	<Carousel.Root
		setApi={connect}
		opts={{ align: 'start', loop: conversations.length > 1 }}
		class="w-full"
	>
		<Carousel.Content class="-ml-3">
			{#each conversations as conversation, i (conversation.id)}
				<Carousel.Item
					class="pl-3"
					aria-roledescription="slide"
					aria-label="{i + 1} of {conversations.length}"
				>
					<!-- Text on a white panel under the photo, so contrast never depends on the photo, GIF
						or video (dark text on white, organiser in theme colour: 4.9 or more in every theme).
						The panel has a notch at its top-left where the Featured badge sits on the photo.
						Participants and closing date are left off to keep the slide light; the
						conversation page has them. -->
					<article
						class="bg-card ring-border/60 flex h-full flex-col rounded-3xl p-2 shadow-md ring-1"
					>
						<div
							class="bg-primary/10 relative isolate h-56 overflow-hidden rounded-2xl sm:h-72 lg:h-[22rem]"
						>
							{#if conversation.featuredMedia?.kind === 'video'}
								<video
									src={conversation.featuredMedia.src}
									poster={conversation.imageUrl as string}
									autoplay
									muted
									loop
									playsinline
									aria-hidden="true"
									class="absolute inset-0 -z-10 size-full object-cover"
								></video>
							{:else if conversation.featuredMedia?.kind === 'gif'}
								<img
									src={conversation.featuredMedia.src}
									alt=""
									class="absolute inset-0 -z-10 size-full object-cover"
								/>
							{:else if conversation.imageUrl}
								<!-- No moving media yet: a slow pan and zoom keeps the photo alive. -->
								<img
									src={conversation.imageUrl as string}
									alt=""
									class="ken-burns absolute inset-0 -z-10 size-full object-cover"
								/>
							{:else}
								<MessageSquareText
									class="text-primary/30 absolute inset-0 m-auto size-24"
								/>
							{/if}
							<!-- A soft band of theme colour along the bottom: brand only, no text on it. -->
							<div
								aria-hidden="true"
								class="from-primary/45 absolute inset-x-0 bottom-0 h-1/3 bg-gradient-to-t to-transparent"
							></div>
						</div>

						<div class="featured-panel relative -mt-6 rounded-b-2xl">
							<!-- The raised part of the panel, right of the notch. -->
							<div aria-hidden="true" class="featured-tab"></div>
							<!-- Same spot on every slide, so people can tell at a glance these are featured.
								Theme text on white: 9:1 or more in every theme. -->
							<span
								class="text-foreground absolute bottom-full left-2 mb-2 flex items-center gap-1.5 rounded-full bg-white px-3 py-1.5 text-sm font-semibold shadow-md"
							>
								<Star
									class="size-4 fill-amber-400 text-amber-500"
									aria-hidden="true"
								/>
								Featured
							</span>
							<div
								class="flex flex-col gap-4 px-3 pt-3 pb-3 sm:px-4 lg:flex-row lg:items-end lg:justify-between lg:gap-10 lg:px-6 lg:pb-5"
							>
								<div class="flex min-w-0 flex-col gap-2">
									<p class="text-primary text-base font-semibold">
										{conversation.organiser}
									</p>
									<h2
										class="text-foreground text-2xl leading-tight font-bold tracking-tight sm:text-3xl"
									>
										{conversation.title}
									</h2>
									<p class="text-foreground line-clamp-2 text-lg">
										{conversation.shortDescription}
									</p>
								</div>
								<a
									href={hrefFor(conversation.id)}
									class="bg-primary text-primary-foreground flex h-14 w-full shrink-0 items-center justify-center gap-2 rounded-full text-lg font-bold transition hover:opacity-90 sm:w-auto sm:self-start sm:px-10 lg:self-end"
								>
									{actionFor(conversation)}
									<ArrowRight class="size-5" />
								</a>
							</div>
						</div>
					</article>
				</Carousel.Item>
			{/each}
		</Carousel.Content>
	</Carousel.Root>

	{#if conversations.length > 1}
		<div class="flex items-center justify-between gap-4">
			<div class="flex items-center gap-1">
				{#each conversations as conversation, i (conversation.id)}
					<button
						aria-label="Show featured conversation {i + 1}"
						aria-current={selected === i ? 'true' : undefined}
						onclick={() => api?.scrollTo(i)}
						class="flex size-8 items-center justify-center"
					>
						<span
							class={cn(
								'h-2 rounded-full transition-all',
								selected === i ? 'bg-primary w-6' : 'bg-muted-foreground/40 w-2'
							)}
						></span>
					</button>
				{/each}
			</div>
			<div class="flex gap-2">
				<button
					aria-label="Previous featured conversation"
					onclick={() => api?.scrollPrev()}
					class="border-border hover:bg-accent flex size-11 items-center justify-center rounded-full border"
				>
					<ChevronLeft class="size-5" />
				</button>
				<button
					aria-label="Next featured conversation"
					onclick={() => api?.scrollNext()}
					class="border-border hover:bg-accent flex size-11 items-center justify-center rounded-full border"
				>
					<ChevronRight class="size-5" />
				</button>
			</div>
		</div>
	{/if}
</section>

<style>
	@media (prefers-reduced-motion: no-preference) {
		.ken-burns {
			animation: ken-burns 18s ease-in-out infinite alternate;
		}
	}

	@keyframes ken-burns {
		from {
			transform: scale(1.05) translate(0, 0);
		}
		to {
			transform: scale(1.18) translate(-3%, -2%);
		}
	}

	/* The notch: the panel's top edge sits 1.5rem into the photo, and a raised tab covers the
		photo from the end of the notch to the right edge, 2.5rem higher. Where the tab meets the
		lower edge, a small inverted corner makes the curve. --notch is the notch width, sized to
		hold the Featured badge. */
	.featured-panel {
		--notch: 8.5rem;
		--tab: 2.5rem;
		--curve: 1.25rem;
		background: var(--card);
	}
	.featured-tab {
		position: absolute;
		bottom: 100%;
		left: var(--notch);
		right: 0;
		height: var(--tab);
		background: var(--card);
		border-top-left-radius: var(--curve);
		border-top-right-radius: 1rem;
	}
	.featured-tab::before {
		content: '';
		position: absolute;
		bottom: 0;
		left: calc(var(--curve) * -1);
		width: var(--curve);
		height: var(--curve);
		background: radial-gradient(
			circle at 0 0,
			transparent calc(var(--curve) - 0.5px),
			var(--card) var(--curve)
		);
	}
</style>
