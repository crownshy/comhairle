<script lang="ts">
	import { resolve } from '$app/paths';
	import { ArrowRight, Lightbulb } from 'lucide-svelte';
	import { isExternalHref, parseGuideText } from '$lib/admin_guide_text';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
	let topic = $derived(data.topic);
</script>

<!-- Guide text can mark **bold** UI names and [links](/path); parseGuideText
     splits it into pieces so nothing is injected as raw HTML. -->
{#snippet richText(text: string)}
	{#each parseGuideText(text) as part, i (i)}
		{#if part.kind === 'bold'}<strong class="font-semibold">{part.text}</strong>
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

<svelte:head>
	<title>{topic.title} - Comhairle Admin Guide</title>
</svelte:head>

<main class="w-full max-w-6xl min-w-0 rounded-xl p-6 md:p-10">
	<article class="max-w-3xl">
		<p class="text-muted-foreground text-sm font-medium">{data.guide.title}</p>
		<h1 class="text-foreground mt-1 text-2xl font-bold">{topic.title}</h1>
		<p class="text-muted-foreground mt-3 text-lg leading-7">
			{@render richText(topic.summary)}
		</p>

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
							<p class="text-foreground pt-0.5 leading-6">{@render richText(step)}</p>
						</li>
					{/each}
				</ol>
			</section>
		{/if}

		{#if topic.terms?.length}
			<dl class="mt-8 flex flex-col gap-5">
				{#each topic.terms as { term, definition } (term)}
					<div>
						<dt class="text-foreground font-semibold">{term}</dt>
						<dd class="text-foreground mt-1 leading-6">
							{@render richText(definition)}
						</dd>
					</div>
				{/each}
			</dl>
		{/if}

		{#if topic.tips?.length}
			<aside class="bg-nav-background mt-8 rounded-lg p-5" aria-labelledby="tips-heading">
				<h2 id="tips-heading" class="text-foreground flex items-center gap-2 font-semibold">
					<Lightbulb class="text-primary size-4" aria-hidden="true" />
					Good to know
				</h2>
				<ul class="text-foreground mt-3 flex list-disc flex-col gap-2 pl-5 leading-6">
					{#each topic.tips as tip, i (i)}
						<li>{@render richText(tip)}</li>
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
</main>
