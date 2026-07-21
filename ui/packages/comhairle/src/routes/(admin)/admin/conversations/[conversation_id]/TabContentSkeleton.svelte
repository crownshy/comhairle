<script lang="ts">
	import { Skeleton } from '$lib/components/ui/skeleton';
</script>

<!-- Shown in the content column while a section tab's `load` runs, so a throttled
	 navigation reads as in-progress rather than frozen on the previous tab. Cosmetic:
	 a heading pair plus a few block placeholders, sized to the reading column. -->
<div class="flex flex-col gap-6" role="status" aria-label="Loading section">
	<div class="flex flex-col gap-3">
		<Skeleton class="h-8 w-64 max-w-full" />
		<Skeleton class="h-4 w-96 max-w-full" />
	</div>
	<div class="flex flex-col gap-4">
		{#each [0, 1, 2] as block (block)}
			<Skeleton class="h-28 w-full" />
		{/each}
	</div>
	<span class="sr-only">Loading…</span>
</div>
