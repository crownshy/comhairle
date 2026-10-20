<script lang="ts">
	import { BookOpen, ArrowUpRight } from '@lucide/svelte';
	import type { SectionHeaderContent } from './sectionHeader';

	type Props = {
		/** What to show, from `sectionHeader()`. */
		content: SectionHeaderContent;
	};

	let { content }: Props = $props();

	let spacing = $derived.by(() => {
		if (content.compact) return content.hasTabs ? 'pt-3 pb-1' : 'border-border border-b py-3';
		return content.hasTabs ? 'pt-page-top pb-4' : 'border-border border-b pt-page-top pb-6';
	});
</script>

{#snippet guideLink()}
	<a
		href={content.guide.href}
		target="_blank"
		rel="noopener noreferrer"
		title="Open the guide in a new tab"
		class="border-border text-foreground hover:bg-accent inline-flex shrink-0 items-center gap-2 self-start rounded-full border px-3 py-1.5 text-sm font-medium transition-colors"
	>
		<BookOpen class="text-primary size-4 shrink-0" />
		<span class="max-w-60 truncate">{content.guide.label}</span>
		<ArrowUpRight class="text-muted-foreground size-4 shrink-0" />
	</a>
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
