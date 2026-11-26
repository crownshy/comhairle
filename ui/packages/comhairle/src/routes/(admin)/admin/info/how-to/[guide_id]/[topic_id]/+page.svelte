<script lang="ts">
	import { resolve } from '$app/paths';
	import ArrowRight from '@lucide/svelte/icons/arrow-right';
	import BookOpen from '@lucide/svelte/icons/book-open';
	import GuideRichText from '$lib/components/AdminGuide/GuideRichText.svelte';
	import GuideTopicContent from '$lib/components/AdminGuide/GuideTopicContent.svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
	let topic = $derived(data.topic);
</script>

<svelte:head>
	<title>{topic.title} - Comhairle Handbook</title>
</svelte:head>

<div
	class="grid w-full max-w-6xl min-w-0 gap-8 rounded-xl p-6 md:p-10 lg:grid-cols-[minmax(0,48rem)_16rem] lg:items-start lg:gap-10"
>
	<article class="max-w-3xl min-w-0">
		<GuideTopicContent guideTitle={data.guide.title} {topic} />

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
				<GuideRichText text={topic.related.text} />
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
</div>
