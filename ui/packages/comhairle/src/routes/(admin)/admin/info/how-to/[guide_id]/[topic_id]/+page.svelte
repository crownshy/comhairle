<script lang="ts">
	import { resolve } from '$app/paths';
	import { ArrowRight, BookOpen, Lightbulb } from 'lucide-svelte';
	import { isExternalHref, parseGuideText } from '$lib/admin_guide_text';
	import GuideImageCarousel from '$lib/components/AdminGuide/GuideImageCarousel.svelte';
	import GuideUiIcon from '$lib/components/AdminGuide/GuideUiIcon.svelte';
	import type { AdminGuideImage } from '$lib/admin_guides';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
	let topic = $derived(data.topic);

	// Screenshots linked to steps or tips: hovering or clicking that text shows its image
	// in the carousel, and the text whose image is showing is highlighted.
	let slide = $state(0);
	function slidesFor(field: 'step' | 'tip') {
		return new Map(
			(topic.images ?? []).flatMap((image, i) =>
				image[field] ? [[image[field]! - 1, i] as const] : []
			)
		);
	}
	let stepSlides = $derived(slidesFor('step'));
	let tipSlides = $derived(slidesFor('tip'));
	$effect(() => {
		void topic.key;
		slide = 0;
	});
</script>

<!-- Guide text can mark **bold** UI names and [links](/path); parseGuideText
     splits it into pieces so nothing is injected as raw HTML. -->
{#snippet richText(text: string)}
	{#each parseGuideText(text) as part, i (i)}
		{#if part.kind === 'bold'}<strong class="font-semibold">{part.text}</strong>
		{:else if part.kind === 'icon'}<GuideUiIcon name={part.name} />
		{:else if part.kind === 'link'}<a
				href={part.href}
				class="text-primary font-medium underline underline-offset-2 hover:no-underline"
				target={isExternalHref(part.href) && !part.href.startsWith('mailto:')
					? '_blank'
					: undefined}
				rel={isExternalHref(part.href) ? 'noopener noreferrer' : undefined}>{part.text}</a
			>
		{:else}{part.text}{/if}
	{/each}
{/snippet}

<!-- Text linked to a carousel screenshot: hover, click or focus shows the image, and the
     text is highlighted in yellow while its image is showing. -->
{#snippet linkedText(text: string, target: number, label: string)}
	<span
		role="button"
		tabindex="0"
		aria-label={label}
		class="cursor-pointer rounded-sm box-decoration-clone px-0.5 transition-colors duration-300 {target ===
		slide
			? 'bg-yellow-200 dark:bg-yellow-400/30'
			: 'decoration-muted-foreground/50 underline decoration-dotted underline-offset-4'}"
		onmouseenter={() => (slide = target)}
		onfocus={() => (slide = target)}
		onclick={() => (slide = target)}
		onkeydown={(e) => {
			if (e.key === 'Enter' || e.key === ' ') {
				e.preventDefault();
				slide = target;
			}
		}}>{@render richText(text)}</span
	>
{/snippet}

<!-- A screenshot with an optional caption underneath. -->
{#snippet figure(image: AdminGuideImage, className = '')}
	<figure class={className}>
		<div class="border-border bg-muted w-full overflow-hidden rounded-xl border p-2 shadow-sm">
			<img src={image.src} alt={image.alt} class="block h-auto w-full rounded-xl" />
		</div>
		{#if image.caption}
			<figcaption class="text-muted-foreground mt-2 text-center text-sm">
				{@render richText(image.caption)}
			</figcaption>
		{/if}
	</figure>
{/snippet}

<svelte:head>
	<title>{topic.title} - Comhairle Admin Guide</title>
</svelte:head>

<main
	class="grid w-full max-w-6xl min-w-0 gap-8 rounded-xl p-6 md:p-10 lg:grid-cols-[minmax(0,48rem)_16rem] lg:items-start lg:gap-10"
>
	<article class="max-w-3xl min-w-0">
		<p class="text-muted-foreground text-sm font-medium">{data.guide.title}</p>
		<h1 class="text-foreground mt-1 text-2xl font-bold">{topic.title}</h1>
		<p class="text-muted-foreground mt-3 text-lg leading-7">
			{@render richText(topic.summary)}
		</p>

		{#if topic.images && topic.images.length > 1}
			{#key topic.key}
				<GuideImageCarousel
					images={topic.images}
					caption={richText}
					class="mt-6"
					bind:current={slide}
				/>
			{/key}
		{:else if topic.images?.[0] ?? topic.image}
			{@render figure((topic.images?.[0] ?? topic.image)!, 'mt-6')}
		{/if}

		{#if topic.environment}
			<!-- Advanced topics start by introducing the screen: each part, named as it
			     appears, with what it is for. -->
			<section class="mt-8" aria-labelledby="environment-heading">
				<h2 id="environment-heading" class="text-foreground text-lg font-semibold">
					{topic.environment.title}
				</h2>
				{#if topic.environment.intro}
					<p class="text-foreground mt-2 leading-6">
						{@render richText(topic.environment.intro)}
					</p>
				{/if}
				<dl class="border-border mt-4 divide-y border-y">
					{#each topic.environment.parts as part (part.name)}
						<div class="grid gap-1 py-3 sm:grid-cols-[12rem_1fr] sm:gap-6">
							<dt class="text-foreground text-sm font-semibold">{part.name}</dt>
							<dd class="text-muted-foreground text-sm leading-6">
								{@render richText(part.description)}
							</dd>
						</div>
					{/each}
				</dl>
			</section>
		{/if}

		{#if topic.steps?.length}
			<section class="mt-8" aria-labelledby="steps-heading">
				<h2 id="steps-heading" class="text-foreground text-lg font-semibold">Steps</h2>
				<ol class="mt-4 flex flex-col gap-4">
					{#each topic.steps as step, i (i)}
						<li class="flex gap-3">
							<span
								class="bg-accent text-accent-foreground flex size-7 shrink-0 items-center justify-center rounded-full text-sm font-semibold"
								aria-hidden="true">{i + 1}</span
							>
							<p class="text-foreground pt-0.5 leading-6">
								{#if stepSlides.has(i)}
									{@render linkedText(
										step,
										stepSlides.get(i)!,
										`Step ${i + 1}: show screenshot`
									)}
								{:else}
									{@render richText(step)}
								{/if}
							</p>
						</li>
					{/each}
				</ol>
			</section>
		{/if}

		{#if topic.terms?.length}
			<!-- Glossary: one card per term in a two-column grid. Each card has an id
			     (e.g. #term-conversation) so other pages can link straight to a term. -->
			<dl class="mt-8 grid gap-3 sm:grid-cols-2">
				{#each topic.terms as { term, definition } (term)}
					<div
						id={`term-${term.toLowerCase().replace(/[^a-z0-9]+/g, '-')}`}
						class="border-border bg-card hover:border-primary/40 target:border-primary target:ring-primary/20 scroll-mt-6 rounded-xl border p-4 transition-colors target:ring-2"
					>
						<dt class="flex items-center gap-3">
							<span
								class="bg-accent text-accent-foreground flex size-8 shrink-0 items-center justify-center rounded-lg text-sm font-bold"
								aria-hidden="true">{term.charAt(0)}</span
							>
							<span class="text-foreground font-semibold">{term}</span>
						</dt>
						<dd class="text-muted-foreground mt-3 text-sm leading-6">
							{@render richText(definition)}
						</dd>
					</div>
				{/each}
			</dl>
		{/if}

		{#if topic.decisions}
			<!-- Advanced topics: each decision says when to choose it, how to do it, and
			     what happens as a result. -->
			<section class="mt-10" aria-labelledby="decisions-heading">
				<h2 id="decisions-heading" class="text-foreground text-lg font-semibold">
					{topic.decisions.title}
				</h2>
				{#if topic.decisions.intro}
					<p class="text-foreground mt-2 leading-6">
						{@render richText(topic.decisions.intro)}
					</p>
				{/if}
				<ol class="mt-6 flex flex-col gap-8">
					{#each topic.decisions.options as option, i (option.title)}
						<li class="flex gap-4">
							<span
								class="bg-accent text-accent-foreground flex size-8 shrink-0 items-center justify-center rounded-full text-sm font-semibold"
								aria-hidden="true">{i + 1}</span
							>
							<div class="min-w-0 flex-1">
								<h3 class="text-foreground pt-1 font-semibold">{option.title}</h3>
								<p class="text-muted-foreground mt-1 text-sm leading-6">
									<span class="text-foreground font-medium">Choose this when</span
									>
									{@render richText(option.when)}
								</p>
								<p class="text-foreground mt-3 text-sm font-medium">How</p>
								<ol
									class="text-foreground mt-1 list-decimal pl-5 text-sm leading-6"
								>
									{#each option.how as step, j (j)}
										<li>{@render richText(step)}</li>
									{/each}
								</ol>
								<p
									class="border-primary text-foreground mt-3 border-l-2 pl-3 text-sm leading-6"
								>
									<span class="font-medium">What happens:</span>
									{@render richText(option.result)}
								</p>
								{#if option.image}
									{@render figure(option.image, 'mt-4')}
								{/if}
							</div>
						</li>
					{/each}
				</ol>
			</section>
		{/if}

		{#if topic.reference}
			<section class="mt-10" aria-labelledby="reference-heading">
				<h2 id="reference-heading" class="text-foreground text-lg font-semibold">
					{topic.reference.title}
				</h2>
				{#if topic.reference.intro}
					<p class="text-foreground mt-2 leading-6">
						{@render richText(topic.reference.intro)}
					</p>
				{/if}
				<dl class="mt-4 grid gap-3 sm:grid-cols-2">
					{#each topic.reference.items as item (item.code)}
						<div class="border-border flex gap-3 rounded-xl border p-4">
							<span
								class="bg-accent text-accent-foreground flex size-8 shrink-0 items-center justify-center rounded-lg text-sm font-bold"
								aria-hidden="true">{item.code}</span
							>
							<div>
								<dt class="text-foreground text-sm font-semibold">{item.name}</dt>
								<dd class="text-muted-foreground mt-1 text-sm leading-6">
									{@render richText(item.description)}
								</dd>
							</div>
						</div>
					{/each}
				</dl>
			</section>
		{/if}

		{#if topic.tips?.length}
			<aside class="bg-nav-background mt-8 rounded-lg p-5" aria-labelledby="tips-heading">
				<h2 id="tips-heading" class="text-foreground flex items-center gap-2 font-semibold">
					<Lightbulb class="text-primary size-4" aria-hidden="true" />
					Good to know
				</h2>
				<ul class="text-foreground mt-3 flex list-disc flex-col gap-2 pl-5 leading-6">
					{#each topic.tips as tip, i (i)}
						<li>
							{#if tipSlides.has(i)}
								{@render linkedText(
									tip,
									tipSlides.get(i)!,
									'Show screenshot for this tip'
								)}
							{:else}
								{@render richText(tip)}
							{/if}
						</li>
					{/each}
				</ul>
			</aside>
		{/if}

		{#if data.next}
			<nav class="border-border mt-10 border-t pt-6" aria-label="Next topic">
				<a
					href={resolve('/admin/info/how-to/[guide_id]/[topic_id]', {
						guide_id: data.next.guideKey,
						topic_id: data.next.topicKey
					})}
					class="group hover:bg-muted -mx-3 flex items-center justify-between rounded-lg px-3 py-3"
				>
					<span>
						<span class="text-muted-foreground block text-sm">Next</span>
						<span class="text-foreground font-semibold">{data.next.title}</span>
					</span>
					<ArrowRight
						class="text-primary size-5 transition-transform group-hover:translate-x-1"
						aria-hidden="true"
					/>
				</a>
			</nav>
		{/if}
	</article>

	{#if topic.related}
		<!-- Sticky pointer to a related topic: beside the content on large screens,
		     after it on small ones. -->
		<aside
			class="border-border bg-nav-background rounded-xl border p-5 lg:sticky lg:top-6"
			aria-labelledby="related-heading"
		>
			<BookOpen class="text-primary size-5" aria-hidden="true" />
			<h2 id="related-heading" class="text-foreground mt-3 font-semibold">
				{topic.related.title}
			</h2>
			<p class="text-muted-foreground mt-1 text-sm leading-6">
				{@render richText(topic.related.text)}
			</p>
			<a
				href={topic.related.href}
				class="group text-primary mt-3 inline-flex items-center gap-1 text-sm font-semibold hover:underline"
			>
				{topic.related.label}
				<ArrowRight
					class="size-4 transition-transform group-hover:translate-x-1"
					aria-hidden="true"
				/>
			</a>
		</aside>
	{/if}
</main>
