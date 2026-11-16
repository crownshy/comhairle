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
					<article
						class="bg-primary relative isolate flex h-[34rem] flex-col justify-end overflow-hidden rounded-3xl sm:h-[32rem] lg:h-[36rem]"
					>
						<!-- A badge in the same spot on every slide, so people can tell at a glance these
							are featured. Theme text on white: 9:1 or more in every theme. -->
						<span
							class="text-foreground absolute top-4 left-4 z-10 flex items-center gap-1.5 rounded-full bg-white px-3 py-1.5 text-sm font-semibold shadow-md lg:top-6 lg:left-6"
						>
							<Star class="size-4 fill-amber-400 text-amber-500" aria-hidden="true" />
							Featured
						</span>
						{#if conversation.featuredMedia?.kind === 'video'}
							<video
								src={conversation.featuredMedia.src}
								poster={conversation.imageUrl as string}
								autoplay
								muted
								loop
								playsinline
								aria-hidden="true"
								class="absolute inset-0 -z-20 size-full object-cover"
							></video>
						{:else if conversation.featuredMedia?.kind === 'gif'}
							<img
								src={conversation.featuredMedia.src}
								alt=""
								class="absolute inset-0 -z-20 size-full object-cover"
							/>
						{:else if conversation.imageUrl}
							<!-- No moving media yet: a slow pan and zoom keeps the photo alive. -->
							<img
								src={conversation.imageUrl as string}
								alt=""
								class="ken-burns absolute inset-0 -z-20 size-full object-cover"
							/>
						{:else}
							<MessageSquareText
								class="text-primary-foreground/20 absolute top-16 right-8 -z-20 size-40"
							/>
						{/if}
						<!-- Frosted glass behind the text: the photo blurred under a darkened, see-through theme
							colour (style block below), so white text stays at WCAG AA whatever the photo or GIF
							shows. It fades into the photo only past the text: above it on phones, to its right on
							large screens. Participants and closing date are left off to keep the slide light;
							the conversation page has them. -->
						<div
							class="text-primary-foreground relative flex flex-col gap-3 p-5 pt-8 sm:p-8 sm:pt-10 lg:h-full lg:w-[52%] lg:justify-end lg:p-12"
						>
							<div aria-hidden="true" class="frosted-glass"></div>
							<p class="text-base font-semibold">{conversation.organiser}</p>
							<h2
								class="text-2xl leading-tight font-bold tracking-tight sm:text-3xl lg:text-4xl"
							>
								{conversation.title}
							</h2>
							<p class="line-clamp-2 text-lg">
								{conversation.shortDescription}
							</p>
							<a
								href={hrefFor(conversation.id)}
								class="bg-primary-foreground text-primary mt-2 flex h-14 w-full items-center justify-center gap-2 rounded-full text-lg font-bold transition hover:opacity-90 sm:w-auto sm:self-start sm:px-10"
							>
								{actionFor(conversation)}
								<ArrowRight class="size-5" />
							</a>
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

	/* Frosted glass behind the slide text: the photo blurred, under the theme colour darkened
		by 30% at 80% strength. Darkening is what keeps white text at WCAG AA even if the blurred
		photo behind is white: about 6.3 Comhairle, 5.6 Scottish Government, 4.8 Waves. The glass
		reaches past the text and a mask fades it out there: upwards on phones, rightwards on
		large screens. */
	.frosted-glass {
		position: absolute;
		z-index: -1;
		inset: -5rem 0 0 0;
		background: color-mix(
			in srgb,
			color-mix(in srgb, var(--primary) 70%, black) 80%,
			transparent
		);
		backdrop-filter: blur(24px) saturate(1.2);
		-webkit-backdrop-filter: blur(24px) saturate(1.2);
		mask-image: linear-gradient(to top, black calc(100% - 5rem), transparent);
		pointer-events: none;
	}
	@media (min-width: 64rem) {
		.frosted-glass {
			inset: 0 -10rem 0 0;
			mask-image: linear-gradient(to right, black calc(100% - 10rem), transparent);
		}
	}
</style>
