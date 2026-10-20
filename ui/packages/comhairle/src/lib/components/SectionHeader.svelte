<script lang="ts" module>
	/** A link into the admin Handbook. */
	export type GuideLink = { href: string; label: string };

	export type SectionHeaderContent = {
		/** Small label above the title: the conversation stage, or the sidebar group. */
		eyebrow?: string;
		title: string;
		description?: string;
		/** Handbook topic for this page. Leave out when there is none yet. */
		guide?: GuideLink;
		/** A tab strip follows, so the header leaves its bottom edge to the strip. */
		hasTabs?: boolean;
		/** One short row, so an editor below keeps its height. */
		compact?: boolean;
	};
</script>

<script lang="ts">
	import type { Snippet } from 'svelte';
	import { BookOpen, ArrowUpRight } from '@lucide/svelte';

	type Props = {
		content: SectionHeaderContent;
		/** Page buttons shown at the right of the title row. */
		actions?: Snippet;
	};

	let { content, actions }: Props = $props();

	let spacing = $derived.by(() => {
		if (content.compact) return content.hasTabs ? 'pt-3 pb-1' : 'border-border border-b py-3';
		return content.hasTabs ? 'pt-page-top pb-4' : 'border-border border-b pt-page-top pb-6';
	});
</script>

{#snippet guideLink()}
	<div class="flex shrink-0 items-center gap-2 self-start">
		{@render actions?.()}
		{#if content.guide}
			<a
				href={content.guide.href}
				target="_blank"
				rel="noopener noreferrer"
				title="Open the guide in a new tab"
				class="border-border text-foreground hover:bg-accent inline-flex shrink-0 items-center gap-2 rounded-full border px-3 py-1.5 text-sm font-medium transition-colors"
			>
				<BookOpen class="text-primary size-4 shrink-0" />
				<span class="max-w-60 truncate">{content.guide.label}</span>
				<ArrowUpRight class="text-muted-foreground size-4 shrink-0" />
			</a>
		{/if}
	</div>
{/snippet}

<!-- When a tab strip follows, it supplies the bottom border so header and tabs read as one block. -->
<header class="bg-admin-background px-gutter shrink-0 {spacing}">
	{#if content.compact}
		<div class="max-w-admin mx-auto flex w-full items-center justify-between gap-3">
			<div class="flex min-w-0 items-center gap-1.5 text-sm">
				{#if content.eyebrow}
					<span class="text-muted-foreground shrink-0">{content.eyebrow}</span>
					<span class="text-muted-foreground" aria-hidden="true">/</span>
				{/if}
				<h1 class="text-foreground truncate font-semibold">{content.title}</h1>
			</div>
			{@render guideLink()}
		</div>
	{:else}
		<div
			class="max-w-admin mx-auto flex w-full flex-col gap-3 sm:flex-row sm:items-start sm:justify-between"
		>
			<div class="flex min-w-0 flex-col gap-1">
				{#if content.eyebrow}
					<p class="text-muted-foreground text-sm font-semibold">{content.eyebrow}</p>
				{/if}
				<h1 class="text-foreground text-3xl font-bold">{content.title}</h1>
				{#if content.description}
					<p class="text-muted-foreground text-base">{content.description}</p>
				{/if}
			</div>
			{@render guideLink()}
		</div>
	{/if}
</header>
