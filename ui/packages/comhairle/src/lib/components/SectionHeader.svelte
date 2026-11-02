<script lang="ts" module>
	/** A link into the admin Handbook. */
	export type GuideLink = { href: string; label: string };

	export type SectionHeaderContent = {
		/** A link back to the parent page, shown above the eyebrow. */
		back?: { href: string; label: string };
		/** Small label above the title: the conversation stage, or the sidebar group. */
		eyebrow?: string;
		title: string;
		description?: string;
		/** Handbook topic for this page. Leave out when there is none yet. */
		guide?: GuideLink;
	};
</script>

<script lang="ts">
	import type { Snippet } from 'svelte';
	import { ArrowLeft, ArrowUpRight, BookOpen } from 'lucide-svelte';
	import { cn } from '$lib/utils';
	import { helpStore } from '$lib/help/helpStore.svelte';

	type Props = {
		content: SectionHeaderContent;
		/** A tab strip follows, so the header leaves its bottom edge to the strip. */
		hasTabs?: boolean;
		/** One short row, so an editor below keeps its height. */
		compact?: boolean;
		/** Page buttons shown at the right of the title row. */
		actions?: Snippet;
		/** The page below is full-bleed, so the header sits at the left gutter instead of centring. */
		fullBleed?: boolean;
	};

	let { content, hasTabs = false, compact = false, actions, fullBleed = false }: Props = $props();

	let column = $derived(fullBleed ? 'w-full' : 'max-w-admin mx-auto w-full');
	// Shares this page's guide with the help button while the header is on screen.
	$effect(() => {
		const guide = content.guide;
		if (!guide) return;
		helpStore.setGuide(guide);
		return () => helpStore.clearGuide(guide);
	});
</script>

{#snippet trailing()}
	<div class="flex shrink-0 items-center gap-2 self-start">
		{@render actions?.()}
	</div>
{/snippet}

<!-- When a tab strip follows, it supplies the bottom border so header and tabs read as one block. -->
<header
	data-tour="section-header"
	class={cn(
		'bg-admin-background px-gutter shrink-0',
		!hasTabs && 'border-border border-b',
		compact && (hasTabs ? 'pt-3 pb-1' : 'py-3'),
		!compact && (hasTabs ? 'pt-page-top pb-4' : 'pt-page-top pb-6')
	)}
>
	{#if compact}
		<div class={cn(column, 'flex items-center justify-between gap-3')}>
			<div class="flex min-w-0 items-center gap-1.5 text-base">
				{#if content.back}
					<!-- eslint-disable svelte/no-navigation-without-resolve -- back hrefs are built with resolve() -->
					<a
						href={content.back.href}
						class="text-muted-foreground hover:text-foreground inline-flex shrink-0 items-center gap-1.5 font-medium transition-colors"
					>
						<ArrowLeft class="size-4" />
						{content.back.label}
					</a>
					<!-- eslint-enable svelte/no-navigation-without-resolve -->
					<span class="text-muted-foreground" aria-hidden="true">/</span>
				{/if}
				{#if content.eyebrow}
					<span class="text-muted-foreground shrink-0">{content.eyebrow}</span>
					<span class="text-muted-foreground" aria-hidden="true">/</span>
				{/if}
				<h1 class="text-foreground truncate font-semibold">{content.title}</h1>
			</div>
			{@render trailing()}
		</div>
	{:else}
		<div
			class={cn(column, 'flex flex-col gap-3 sm:flex-row sm:items-start sm:justify-between')}
		>
			<div class="flex min-w-0 flex-col gap-1">
				{#if content.back}
					<!-- eslint-disable svelte/no-navigation-without-resolve -- back hrefs are built with resolve() -->
					<a
						href={content.back.href}
						class="text-muted-foreground hover:text-foreground mb-2 inline-flex items-center gap-1.5 self-start text-base font-medium transition-colors"
					>
						<ArrowLeft class="size-4" />
						{content.back.label}
					</a>
					<!-- eslint-enable svelte/no-navigation-without-resolve -->
				{/if}
				{#if content.eyebrow}
					<p class="text-muted-foreground text-sm font-semibold">{content.eyebrow}</p>
				{/if}
				<h1 class="text-foreground text-3xl font-bold">{content.title}</h1>
				{#if content.description}
					<p class="text-muted-foreground text-base">{content.description}</p>
				{/if}
			</div>
			{@render trailing()}
		</div>
	{/if}
</header>
