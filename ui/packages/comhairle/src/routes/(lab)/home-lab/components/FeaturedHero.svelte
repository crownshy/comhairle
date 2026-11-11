<script lang="ts">
	import {
		ArrowRight,
		ChevronLeft,
		ChevronRight,
		Clock,
		MessageSquareText,
		Users
	} from 'lucide-svelte';
	import * as Carousel from '$lib/components/ui/carousel';
	import type { CarouselAPI } from '$lib/components/ui/carousel/context';
	import { cn } from '$lib/utils';
	import { participantsLabel, timingLabel, type LabConversation } from '../mockData';

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
						<!-- Overlay in the organisation's colour: solid behind the text, clear at the top. -->
						<div
							class="from-primary via-primary/85 lg:via-primary/70 absolute inset-0 -z-10 bg-gradient-to-t to-transparent to-75% lg:bg-gradient-to-r lg:to-70%"
						></div>

						<div
							class="text-primary-foreground flex flex-col gap-3 p-5 sm:p-8 lg:max-w-2xl lg:p-12"
						>
							<span class="text-base font-semibold tracking-wide uppercase opacity-90"
								>Featured</span
							>
							<h2
								class="text-3xl leading-tight font-bold tracking-tight sm:text-4xl lg:text-5xl"
							>
								{conversation.title}
							</h2>
							<p class="line-clamp-2 text-lg opacity-95">
								{conversation.shortDescription}
							</p>
							<span class="flex flex-wrap gap-x-5 gap-y-1 text-base opacity-90">
								{#if conversation.participantCount > 0}
									<span class="flex items-center gap-1.5"
										><Users class="size-5" />
										{participantsLabel(conversation)}</span
									>
								{/if}
								<span class="flex items-center gap-1.5"
									><Clock class="size-5" />
									{timingLabel(conversation)}</span
								>
							</span>
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
</style>
