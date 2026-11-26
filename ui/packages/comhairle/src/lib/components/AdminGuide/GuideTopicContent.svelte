<script lang="ts">
	import Lightbulb from '@lucide/svelte/icons/lightbulb';
	import type { AdminGuideImage, AdminGuideTopic } from '$lib/admin_guides';
	import { planGlossaryTerms } from '$lib/admin_guide_glossary';
	import GuideImageCarousel from './GuideImageCarousel.svelte';
	import GuideRichText from './GuideRichText.svelte';

	// The body of a handbook how-to topic: title, summary, screenshots, steps, decisions,
	// reference and tips. Used by the topic page and, with `embedded`, by the "learn more"
	// dialog/panel opened from a glossary tooltip (no tooltips inside, so they don't nest).
	let {
		guideTitle,
		topic,
		embedded = false
	}: { guideTitle: string; topic: AdminGuideTopic; embedded?: boolean } = $props();

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

	// Glossary tooltips: the first mention of each glossary word on this page gets one.
	// List the page's text in the order it's shown; captions are left out so the tooltip
	// lands in the main reading flow.
	let glossaryPlan = $derived(
		embedded
			? new Map<string, Set<string>>()
			: planGlossaryTerms([
					topic.summary,
					topic.environment?.intro,
					...(topic.environment?.parts.map((part) => part.description) ?? []),
					...(topic.steps ?? []),
					topic.decisions?.intro,
					...(topic.decisions?.options.flatMap((o) => [o.when, ...o.how, o.result]) ??
						[]),
					topic.reference?.intro,
					...(topic.reference?.items.map((item) => item.description) ?? []),
					...(topic.tips ?? [])
				])
	);
</script>

{#snippet richText(text: string)}<GuideRichText {text} terms={glossaryPlan.get(text)} />{/snippet}

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

<p class="text-muted-foreground text-sm font-medium">{guideTitle}</p>
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
							<span class="text-foreground font-medium">Choose this when</span>
							{@render richText(option.when)}
						</p>
						<p class="text-foreground mt-3 text-sm font-medium">How</p>
						<ol class="text-foreground mt-1 list-decimal pl-5 text-sm leading-6">
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
						{@render linkedText(tip, tipSlides.get(i)!, 'Show screenshot for this tip')}
					{:else}
						{@render richText(tip)}
					{/if}
				</li>
			{/each}
		</ul>
	</aside>
{/if}
