<script lang="ts">
	/**
	 * Placeholder for a learn article. It repeats LearnUI's column, auto margins and viewport
	 * padding included, so a short page stands in the middle of the step the way the real one
	 * will instead of hugging the top and then dropping when it arrives. Line heights and gaps
	 * track the `.prose` scale in app.css (18px body on 28px leading, 30px h1 and 26px h2 that
	 * step up at md) for the same reason.
	 *
	 * Given the page's stored document, it follows that page's outline (skeletonBlocks.ts), so
	 * a page that opens on a heading and a video shows a heading and a video box. Without one,
	 * or for markdown, it draws a generic article.
	 *
	 * No stand-in for the Listen offer above the article: `listen.available` is false until a
	 * rendered page is attached, so nothing here could tell whether the row is coming.
	 */
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { getLocale } from '$lib/paraglide/runtime.js';
	import { skeletonBlocks, type SkeletonBlock } from './skeletonBlocks';

	/** A learn tool config's `pages`: each page is its list of translations. */
	type Translation = { lang: string; content?: string };

	let { pages = [], page = 0 }: { pages?: Translation[][]; page?: number } = $props();

	// The same translation LearnUI will draw.
	let content = $derived(
		(pages[page] ?? []).find((translation) => translation.lang === getLocale())?.content
	);

	const GENERIC: SkeletonBlock[] = [
		{ kind: 'heading', level: 1, lines: 1 },
		{ kind: 'text', lines: 4 },
		{ kind: 'heading', level: 2, lines: 1 },
		{ kind: 'text', lines: 4 },
		{ kind: 'list', items: 3 }
	];

	let blocks = $derived(skeletonBlocks(content) ?? GENERIC);

	// Widths only; the ragged right edge is what makes a block read as text, and a short last
	// line is what makes it read as a paragraph.
	const LINE_WIDTHS = ['w-full', 'w-[95%]', 'w-[90%]', 'w-[93%]', 'w-[88%]'];
	const BULLET_WIDTHS = ['w-2/3', 'w-3/4', 'w-1/2'];

	function lineWidth(index: number, lines: number) {
		if (lines > 1 && index === lines - 1) return 'w-[60%]';
		return LINE_WIDTHS[index % LINE_WIDTHS.length];
	}

	function headingWidth(index: number, lines: number) {
		if (lines === 1) return 'w-3/4';
		return index === lines - 1 ? 'w-1/2' : 'w-[90%]';
	}

	const HEADING_HEIGHT = {
		1: 'h-[1.875rem] md:h-9',
		2: 'h-[1.625rem] md:h-[1.875rem]',
		3: 'h-[1.375rem] md:h-6'
	} as const;

	const HEADING_MARGIN = { 1: 'mt-8 mb-6', 2: 'mt-8 mb-4', 3: 'mt-6 mb-3' } as const;
</script>

<div class="mx-auto flex w-full grow flex-col">
	<div class="my-auto flex w-full flex-col py-[clamp(1.5rem,5vh,3rem)]">
		<article
			class="prose mx-auto w-full max-w-[46rem] *:first:mt-0"
			aria-busy="true"
			aria-live="polite"
		>
			{#each blocks as block, blockIndex (blockIndex)}
				{#if block.kind === 'heading'}
					<div class="space-y-2 {HEADING_MARGIN[block.level]}">
						{#each { length: block.lines }, index (index)}
							<Skeleton
								class="{HEADING_HEIGHT[block.level]} {headingWidth(
									index,
									block.lines
								)}"
							/>
						{/each}
					</div>
				{:else if block.kind === 'text'}
					<div class="my-5 space-y-3">
						{#each { length: block.lines }, index (index)}
							<Skeleton class="h-4 {lineWidth(index, block.lines)}" />
						{/each}
					</div>
				{:else if block.kind === 'quote'}
					<div class="border-border my-6 space-y-3 border-l-4 py-1 pl-5">
						{#each { length: block.lines }, index (index)}
							<Skeleton class="h-4 {lineWidth(index, block.lines)}" />
						{/each}
					</div>
				{:else if block.kind === 'list'}
					<div class="my-5 space-y-3 pl-4">
						{#each { length: block.items }, index (index)}
							<div class="flex items-center gap-3">
								<Skeleton class="h-2 w-2 shrink-0 rounded-full" />
								<Skeleton
									class="h-4 {BULLET_WIDTHS[index % BULLET_WIDTHS.length]}"
								/>
							</div>
						{/each}
					</div>
				{:else}
					<Skeleton
						class="w-full {block.ratio === 'video'
							? 'my-4 aspect-video'
							: 'my-8 aspect-[3/2]'}"
					/>
				{/if}
			{/each}

			<span class="sr-only">Loading learning content…</span>
		</article>
	</div>
</div>
