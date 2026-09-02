<script lang="ts">
	/**
	 * Placeholder for a learn article. Line heights and gaps track the `.prose` scale in
	 * app.css (18px body on 28px leading, 30px h1, 26px h2) so the real article drops into
	 * the same rhythm rather than nudging everything down when it arrives.
	 */
	import { Skeleton } from '$lib/components/ui/skeleton';

	// Widths only; the ragged right edge is what makes a block read as text.
	const opening = ['w-full', 'w-[95%]', 'w-[90%]', 'w-[60%]'];
	const body = ['w-full', 'w-[92%]', 'w-[85%]', 'w-[70%]'];
	const bullets = ['w-2/3', 'w-3/4', 'w-1/2'];
</script>

<article class="prose mx-auto w-full" aria-busy="true" aria-live="polite">
	<Skeleton class="mb-6 h-9 w-3/4" />

	<div class="mb-6 space-y-3">
		{#each opening as width (width)}
			<Skeleton class="h-4 {width}" />
		{/each}
	</div>

	<Skeleton class="mt-8 mb-4 h-7 w-1/2" />

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
