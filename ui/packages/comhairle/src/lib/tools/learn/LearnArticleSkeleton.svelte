<script lang="ts">
	/**
	 * Placeholder for a learn article. It repeats LearnUI's column, auto margins and viewport
	 * padding included, so a short page stands in the middle of the step the way the real one
	 * will instead of hugging the top and then dropping when it arrives. Line heights and gaps
	 * track the `.prose` scale in app.css (18px body on 28px leading, 30px h1 and 26px h2 that
	 * step up at md) for the same reason.
	 *
	 * No stand-in for the Listen offer above the article: `listen.available` is false until a
	 * rendered page is attached, so nothing here could tell whether the row is coming.
	 */
	import { Skeleton } from '$lib/components/ui/skeleton';

	// Widths only; the ragged right edge is what makes a block read as text.
	const opening = ['w-full', 'w-[95%]', 'w-[90%]', 'w-[60%]'];
	const body = ['w-full', 'w-[92%]', 'w-[85%]', 'w-[70%]'];
	const bullets = ['w-2/3', 'w-3/4', 'w-1/2'];
</script>

<div class="mx-auto flex w-full grow flex-col">
	<div class="my-auto flex w-full flex-col py-[clamp(1.5rem,5vh,3rem)]">
		<article class="prose mx-auto w-full" aria-busy="true" aria-live="polite">
			<Skeleton class="mb-6 h-[1.875rem] w-3/4 md:h-9" />

			<div class="mb-6 space-y-3">
				{#each opening as width (width)}
					<Skeleton class="h-4 {width}" />
				{/each}
			</div>

			<Skeleton class="mt-8 mb-4 h-[1.625rem] w-1/2 md:h-[1.875rem]" />

			<div class="mb-6 space-y-3">
				{#each body as width (width)}
					<Skeleton class="h-4 {width}" />
				{/each}
			</div>

			<div class="space-y-3 pl-4">
				{#each bullets as width (width)}
					<div class="flex items-center gap-3">
						<Skeleton class="h-2 w-2 shrink-0 rounded-full" />
						<Skeleton class="h-4 {width}" />
					</div>
				{/each}
			</div>

			<span class="sr-only">Loading learning content…</span>
		</article>
	</div>
</div>
